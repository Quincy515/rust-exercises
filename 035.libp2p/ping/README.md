- [rust\_libp2p入门教程](#rust_libp2p入门教程)
  - [ping - pong](#ping---pong)
    - [第一步](#第一步)
    - [网络身份](#网络身份)
    - [传输层 Transport](#传输层-transport)
    - [网络行为](#网络行为)
    - [Swarm 连接处理](#swarm-连接处理)
    - [Multiaddr 地址解析](#multiaddr-地址解析)
    - [轮训 Swarm](#轮训-swarm)
    - [运行两个节点](#运行两个节点)

# rust_libp2p入门教程

https://docs.rs/libp2p/0.50.0/libp2p/tutorials/ping/index.html
https://mp.weixin.qq.com/s/p3g3fqYlKkb0ot6bTAsSTA

## ping - pong

我们将构建一个p2p链接，一方发送ping，另一方回复pong

### 第一步

创建一个新的项目：`cargo new ping` 在文件中添加 `libp2p` 和 `futures` 依赖

```toml
[dependencies]
futures = "0.3"
libp2p = "0.50.0"
```

### 网络身份

我们需要一个身份在网络中通讯，就想微信聊天中的微信号，有了微信号就能找到对方，并进行通信。在 `libp2p` 中身份是一个密钥对，每一个节点的身份通过 `PeerID` 标识，`PeerID` 是通过公钥派生出来的。

```rust
use futures::executor::block_on;  
use libp2p::{identity, ping, PeerId};  
  
fn main() -> Result<(), Box<dyn std::error::Error>> {  
    let local_key = identity::Keypair::generate_ed25519();  
    let local_peer_id = PeerId::from(local_key.public());  
    println!("Local peer id: {local_peer_id:?}");  
    
    Ok(())  
}
```

使用cargo run每次会产生不同的PeerID。

```shell
network on  main [?] is 📦 v0.1.0 via 🦀 v1.65.0 
❯ cargo run --example ping                
    Finished dev [unoptimized + debuginfo] target(s) in 1.11s
     Running `target/debug/examples/ping`
Local peer id: PeerId("12D3KooWRDhSpumPQDHXcLHgzWohhcCG7uvEMA4K272A3bx2Xqzz")

network on  main [?] is 📦 v0.1.0 via 🦀 v1.65.0 
❯ cargo run --example ping
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/examples/ping`
Local peer id: PeerId("12D3KooWJSWKpG3VoQuZxwGpj7Zx59XNv8TzWQ5jRsjttpCffneR")

network on  main [?] is 📦 v0.1.0 via 🦀 v1.65.0 
❯ cargo run --example ping
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/examples/ping`
Local peer id: PeerId("12D3KooWLNuT2HkHj9tynoHdwMu11P5rRtvB31s7QGyz89G13eF8")
```

### 传输层 Transport

我们要从 **A** 到 **B** 发送一些字节。`libp2p` 中的传输提供了面向连接的通信通道（例如 **TCP**），并在诸如身份验证和加密协议之类的基础上进行了升级。代码中需要实现 `Transport trait`。我们没有为本教程自己构建传输，而是使用便利功能 [`development_transport`](https://docs.rs/libp2p/0.50.0/libp2p/fn.development_transport.html#) 来创建带有 `noise` 的 **TCP** 传输以进行认证加密。此外，`development_transport` 构建了一个多路传输，借此多个逻辑子流可以在同一基础（**TCP**）连接上共存。有关子流复用的更多详细信息，请查看 `crate::core::muxing` 和 `yamux`。

查看 [Feature flags](https://docs.rs/crate/libp2p/0.50.0/features) This version has **62** feature flags, **0** of them enabled by **default**.

而 [`development_transport` 源代码中需要特定的 features](https://docs.rs/libp2p/0.50.0/src/libp2p/lib.rs.html#159-217)
```toml
libp2p = {version = "0.50.0", features = [
	"tcp",
	"dns",
	"async-std",
	"websocket",
	"noise",
	"mplex",
	"yamux",
]}
```

```rust
use futures::executor::block_on;  
use libp2p::{identity, ping, PeerId};  
  
fn main() -> Result<(), Box<dyn std::error::Error>> {  
    let local_key = identity::Keypair::generate_ed25519();  
    let local_peer_id = PeerId::from(local_key.public());  
    println!("Local peer id: {local_peer_id:?}");  
    
    let transport = block_on(libp2p::development_transport(local_key))?;  
    
    Ok(())  
}
```

### 网络行为

现在是时候看看 `rust-libp2p` 的另一个核心特征 **NetworkBehaviour**。前面介绍的特征传输定义了如何在网络上发送字节，而 **NetworkBehaviour** 定义了要在网络上发送哪些字节。为了更具体一点，让我们看一下 **NetworkBehaviour** 特性的一个简单实现 **Ping NetworkBehaviour**。您可能已经猜到，类似于 `ping` 网络工具，`libp2p Ping` 将 `ping` 发送到远程，并希望依次接收到 `pong`。**Ping NetworkBehaviour** 不在乎 `ping` 或 `pong` 消息如何在网络上发送，无论它们是通过 `TCP` 发送的，还是通过 `noise` 加密的还是纯文本形式的。它只关心在网络上发送什么消息。**Transport** 和 **NetworkBehaviour** 这两个 `traits` 使我们可以将发送什么字节与怎么发送字节完全分开。

考虑到以上几点，让我们扩展示例，最后创建一个 **Ping NetworkBehaviour**：

```rust
use futures::executor::block_on;  
use libp2p::{identity, ping, PeerId};  
  
fn main() -> Result<(), Box<dyn std::error::Error>> {  
    let local_key = identity::Keypair::generate_ed25519();  
    let local_peer_id = PeerId::from(local_key.public());  
    println!("Local peer id: {local_peer_id:?}");  
    
    let transport = block_on(libp2p::development_transport(local_key))?;  
    
    // Create a ping network behaviour.  
    //    
    // For illustrative purposes, the ping protocol is configured to    
    // keep the connection alive, so a continuous sequence of pings    
    // can be observed.    
    let behaviour = ping::Behaviour::new(ping::Config::new().with_keep_alive(true));  
    
    Ok(())  
}
```

注意这里使用 [`ping::Behaviour`](https://docs.rs/libp2p/0.50.0/libp2p/ping/struct.Behaviour.html) 而  [libp2p](https://docs.rs/libp2p/0.50.0/libp2p/index.html)::[ping](https://docs.rs/libp2p/0.50.0/libp2p/ping/index.html#) Available on **crate feature `ping`** only.

所以需要增加 

```toml
libp2p = {version = "0.50.0", features = [  
  "tcp",  
  "dns",  
  "async-std",  
  "websocket",  
  "noise",  
  "mplex",  
  "yamux",  
  "ping",  
]}
```

### Swarm 连接处理

**Swarm** 负责管理节点之间连接的创建、维护、销毁。包括协议多路复用、流多路复用、**NAT** 穿透和连接中继，同时进行多路传输。

现在我们有了传输和网络行为，我们需要将两者连接起来的东西，使两者都能取得进展。这项工作是由 **Swarm** 完成的。简而言之，**Swarm** 会同时驱动  **Transport** 和 **NetworkBehaviour**，将命令从**NetworkBehaviour** 传递到，并将事件从 **Transport** 传递到 **NetworkBehaviour**。

```rust
use futures::executor::block_on;  
use libp2p::{identity, ping, PeerId};  
  
fn main() -> Result<(), Box<dyn std::error::Error>> {  
    let local_key = identity::Keypair::generate_ed25519();  
    let local_peer_id = PeerId::from(local_key.public());  
    println!("Local peer id: {local_peer_id:?}");  
    
    let transport = block_on(libp2p::development_transport(local_key))?;  
    
    // Create a ping network behaviour.  
    //    
    // For illustrative purposes, the ping protocol is configured to    
    // keep the connection alive, so a continuous sequence of pings    
    // can be observed.    
    let behaviour = ping::Behaviour::new(ping::Config::new().with_keep_alive(true));  
    
    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);  
    
    Ok(())  
}
```

### Multiaddr 地址解析

有了 [**Swarm**](https://docs.rs/libp2p/0.50.0/libp2p/struct.Swarm.html) 之后，我们所有人都可以侦听传入的连接。我们只需要向 [**Swarm**](https://docs.rs/libp2p/0.50.0/libp2p/struct.Swarm.html) 传递一个地址，就像 [`std::net::TcpListener::bind`](https://doc.rust-lang.org/nightly/std/net/tcp/struct.TcpListener.html#method.bind) 一样。但是，我们没有传递 **IP** 地址，而是传递了 **Multiaddr**; `libp2p` 支持多种不同的底层协议，把标准化节点的地址称为 [**Multiaddr**](https://docs.rs/libp2p/0.50.0/libp2p/struct.Multiaddr.html)，目前 `libp2p` 主要支持以下几种地址格式：

-   **`/ip4/127.0.0.1/tcp/4001/ipfs/QmNodeID`**: 这种格式跟传统的 TCP 网络里是一样的，直接可以解析出对应的 IPv4 地址和端口号；
-   **`/ipfs/QmNodeID`**: 这种格式的地址适用于 IPFS 网络，只有节点ID的地址，需要节点路由模块找到节点对应的IP地址，然后再进行连接；
-   **`/dns4/http://ipfs.ipfsbit.com/tcp/443/wss/p2p-webrtc-star`**: 这种地址需要调用multiaddr-dns组件，把域名解析成IP地址，然后再进行连接；
-   **`/p2p-circuit/p2p/QmNodeID`**: 这种地址是relay地址，用于中继网络，需要首先连接一个中继节点，才能连接到目的节点；

[**Multiaddr**](https://docs.rs/libp2p/0.50.0/libp2p/struct.Multiaddr.html) 是一个自我描述的网络地址和协议栈，用于建立与对等体的连接。更具体的细节可以在 [addressing concept](https://docs.libp2p.io/concepts/fundamentals/addressing/) 及其规范存储库 [multiaddr](https://github.com/multiformats/multiaddr/) 上找到有关 **Multiaddr** 的很好的介绍。

让我们的本地节点侦听所有接口以及一个随机端口。此外，如果在 **CLI** 上提供，请指示我们的本地节点拨打远程对等方。

```rust
use futures::executor::block_on;  
use libp2p::{identity, ping, PeerId};  
  
fn main() -> Result<(), Box<dyn std::error::Error>> {  
    let local_key = identity::Keypair::generate_ed25519();  
    let local_peer_id = PeerId::from(local_key.public());  
    println!("Local peer id: {local_peer_id:?}");  
    
    let transport = block_on(libp2p::development_transport(local_key))?;  
    
    // Create a ping network behaviour.  
    //    
    // For illustrative purposes, the ping protocol is configured to    
    // keep the connection alive, so a continuous sequence of pings    
    // can be observed.    
    let behaviour = ping::Behaviour::new(ping::Config::new().with_keep_alive(true));  
    
    let mut swarm = Swarm::new(transport, behaviour, local_peer_id); 
    
    // Dial the peer identified by the multi-address given as the second  
	// command-line argument, if any.  
	if let Some(addr) = std::env::args().nth(1) {
	    let remote: Multiaddr = addr.parse()?;
	    swarm.dial(remote)?;
	    println!("Dialed {}", addr)
	}
	
	Ok(())
}
```

### 轮训 Swarm

现在，我们已经准备就绪。最后一步是循环驱动 **Swarm**，以便在我们在 **CLI** 上指定地址的情况下侦听传入的连接并建立传出的连接。

```rust
use std::task::Poll;  
use futures::executor::block_on;  
use futures::{future, StreamExt};  
use libp2p::{identity, ping, PeerId, Swarm, Multiaddr};
  
fn main() -> Result<(), Box<dyn std::error::Error>> {  
    let local_key = identity::Keypair::generate_ed25519();  
    let local_peer_id = PeerId::from(local_key.public());  
    println!("Local peer id: {local_peer_id:?}");  
    
    let transport = block_on(libp2p::development_transport(local_key))?;  
    
    // Create a ping network behaviour.  
    //    
    // For illustrative purposes, the ping protocol is configured to    
    // keep the connection alive, so a continuous sequence of pings    
    // can be observed.    
    let behaviour = ping::Behaviour::new(ping::Config::new().with_keep_alive(true));  
    
    let mut swarm = Swarm::new(transport, behaviour, local_peer_id); 
    
    // Dial the peer identified by the multi-address given as the second  
	// command-line argument, if any.  
	if let Some(addr) = std::env::args().nth(1) {
	    let remote: Multiaddr = addr.parse()?;
	    swarm.dial(remote)?;
	    println!("Dialed {}", addr)
	}
	  
	let mut listening = false;  
	block_on(future::poll_fn(move |cx| loop {  
	    match swarm.poll_next_unpin(cx) {  
	        Poll::Ready(Some(event)) => println!("{:?}", event),  
	        Poll::Ready(None) => return Poll::Ready(()),  
	        Poll::Pending => {  
	            if !listening {  
	                for addr in Swarm::listeners(&swarm) {  
	                    println!("Listening on {}", addr);  
	                    listening = true;  
	                }  
	            }  
	            return Poll::Pending;  
	        }  
	    }  
	}));
	
	Ok(())
}
```

### 运行两个节点
在第一个终端运行 `cargo run --example ping`

```shell
❯ cargo run --example ping
    Finished dev [unoptimized + debuginfo] target(s) in 12.18s
     Running `target/debug/examples/ping`
Local peer id: PeerId("12D3KooWRNVD5XVJq37KJpSmgjaszLEXr36qEnUyWCx9sD9fAHb5")
NewListenAddr { listener_id: ListenerId(1727563851104570741), address: "/ip4/127.0.0.1/tcp/61365" }
NewListenAddr { listener_id: ListenerId(1727563851104570741), address: "/ip4/192.168.3.178/tcp/61365" }
Listening on /ip4/127.0.0.1/tcp/61365
Listening on /ip4/192.168.3.178/tcp/61365
```

看到输出中监听的端口是61365，第二个节点执行命令：`cargo run --example ping -- /ip4/127.0.0.1/tcp/61365`

```shell
❯ cargo run --example ping -- /ip4/127.0.0.1/tcp/61365
    Finished dev [unoptimized + debuginfo] target(s) in 1.90s
     Running `target/debug/examples/ping /ip4/127.0.0.1/tcp/61365`
Local peer id: PeerId("12D3KooWLwnkz35oDnoEWdQsw7v413jxkfkA7yftF24QUyRYi8Y6")
Dialed /ip4/127.0.0.1/tcp/61365
NewListenAddr { listener_id: ListenerId(7312522014901404981), address: "/ip4/127.0.0.1/tcp/61769" }
NewListenAddr { listener_id: ListenerId(7312522014901404981), address: "/ip4/192.168.3.178/tcp/61769" }
Listening on /ip4/127.0.0.1/tcp/61769
Listening on /ip4/192.168.3.178/tcp/61769
ConnectionEstablished { peer_id: PeerId("12D3KooWRNVD5XVJq37KJpSmgjaszLEXr36qEnUyWCx9sD9fAHb5"), endpoint: Dialer { address: "/ip4/127.0.0.1/tcp/61365", role_override: Dialer }, num_established: 1, concurrent_dial_errors: Some([]) }
Behaviour(Event { peer: PeerId("12D3KooWRNVD5XVJq37KJpSmgjaszLEXr36qEnUyWCx9sD9fAHb5"), result: Ok(Pong) })
Behaviour(Event { peer: PeerId("12D3KooWRNVD5XVJq37KJpSmgjaszLEXr36qEnUyWCx9sD9fAHb5"), result: Ok(Ping { rtt: 507.966µs }) })
Behaviour(Event { peer: PeerId("12D3KooWRNVD5XVJq37KJpSmgjaszLEXr36qEnUyWCx9sD9fAHb5"), result: Ok(Pong) })
Behaviour(Event { peer: PeerId("12D3KooWRNVD5XVJq37KJpSmgjaszLEXr36qEnUyWCx9sD9fAHb5"), result: Ok(Ping { rtt: 480.319µs }) })
Behaviour(Event { peer: PeerId("12D3KooWRNVD5XVJq37KJpSmgjaszLEXr36qEnUyWCx9sD9fAHb5"), result: Ok(Pong) })
Behaviour(Event { peer: PeerId("12D3KooWRNVD5XVJq37KJpSmgjaszLEXr36qEnUyWCx9sD9fAHb5"), result: Ok(Ping { rtt: 770.016µs }) })
```
