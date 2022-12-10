- [libp2p 简介](#libp2p-简介)
  - [网络身份：Network identity](#网络身份network-identity)
  - [传输协议：Transport](#传输协议transport)
  - [网络行为：Network behaviour](#网络行为network-behaviour)
  - [Swarm](#swarm)
- [ping - pong](#ping---pong)
  - [第一步](#第一步)
  - [网络身份](#网络身份)
  - [传输层 Transport](#传输层-transport)
  - [网络行为](#网络行为)
  - [Swarm 连接处理](#swarm-连接处理)
  - [Multiaddr 地址解析](#multiaddr-地址解析)
  - [轮训 Swarm](#轮训-swarm)
  - [运行两个节点](#运行两个节点)
- [参考文档](#参考文档)

## libp2p 简介

libp2p 包含一系列协议的实现，这些协议共同作用，完成了：

- p2p 网络的传输层（下图绿色）：支持几乎所有的主流传输协议，甚至允许不同节点间使用不同的传输层，比如 native 节点间优先使用 QUIC，而 native 和 web 节点间使用 websocket。
- 节点发现（黄色，注意这里 PKI 是指基于 PKI 的节点身份）：一般本地网络可以使用 mDNS，大规模 p2p 网络一般使用 bootstrap 来连接初始节点，然后通过 gossip 获取更多节点信息，并通过 Kad DHT 来查找节点。
- 节点路由（蓝色）：主要使用 Kad DHT 通过多跳来路由到网络中任意一个节点
- 内容路由（紫色）：如果点对点发送消息，可以通过 Kad DHT，如果在网络中 flood，可以通过 floodsub 和 gossipsub 来对某个 topic 的内容进行广播。
- NAT traversal（红色）：包括主流的 hole punching 解决方案

![图片](https://mmbiz.qpic.cn/mmbiz_png/SER9L29WQ09cIicM70pZn0645kXkgqmzNcRseIyEDpvp9aDapY5487AyzXFRg6nxIY2kyH8kXhiayxFOWADt7tDA/640?wx_fmt=png&wxfrom=5&wx_lazy=1&wx_co=1)

（图片来源：A network framework for decentralized P2P application development [2]）

对于开发者而言，相对于掌握这些协议的细节而言，更重要的掌握如何使用 libp2p 构造一个 p2p 网络，所以，我们需要掌握以下概念和它们的使用。

### 网络身份：Network identity

对于一个 p2p 网络，一个节点想让别人认识它并接受它的一个前提是它要有可以被识别的节点身份。这个就是 network identity。libp2p 使用公钥/私钥对来产生 network identity：私钥用于数据的签名，公钥作为 `PeerId`。一般来说，一个节点在初始化之后，应该产生一个配置文件，保存节点的公钥私钥，以便节点以后反复运行时能够使用相同的身份。

### 传输协议：Transport

前面我们提到，在 p2p 网络中，节点间传输协议的选择需要非常多样，这是因为网络中有可能运行着各种版本，甚至不同实现的节点，因而，支持一个范围广泛的传输协议供节点连接时协商，便变得非常必要。此外，p2p 网络需要额外的安全性，不仅仅是消息的加密，还有消息发送者的身份验证 —— 你可以将其类比成使能了 mutual auth 的 TLS：客户端验证服务器的证书来确保连接的是合法的服务器，而服务器同时也验证客户端的证书确保访问的是合法的客户端。

为了实现这一目的，libp2p 抽象出了 `Transport` 层，它负责传输协议的协商，包括使用什么样的传输协议，使用什么样的安全机制，以及如果做多路复用（stream multiplexing）。基本上，这对应了 ISO/OSI 模型的：传输层（比如用 TCP）和会话层（比如用 Noise + Yamux）。如果你使用过 websocket，那么你对一个 HTTP 连接 "upgrade" 成 websocket 并不陌生，libp2p 在这些层之间也是一层层 upgrade 的。我们看个例子：

```rust
let noise_keys = noise::Keypair::<noise::X25519Spec>::new()  
    .into_authentic(&keypair)  
    .unwrap();  
let transport_layer = {  
    let tcp = tcp::TcpConfig::new().nodelay(true);  
    let dns_tcp = dns::DnsConfig::system(tcp).await?;  
    let ws_dns_tcp = websocket::WsConfig::new(dns_tcp.clone());  
    dns_tcp.or_transport(ws_dns_tcp)  
};  
let transport = transport_layer  
    .upgrade(Version::V1)  
    .authenticate(NoiseConfig::xx(noise_keys).into_authenticated())  
    .multiplex(YamuxConfig::default())  
    .timeout(Duration::from_secs(20))  
    .boxed();
```

这段代码并不难懂，非常简洁干练，没有 Rust 经验的人也能猜出来在干什么。但里面包含的信息量巨大：

- 传输层使用 TCP 协议或者 websocket（优先 TCP 协议）。
- 安全层使用 noise protocol，使用 XX 模式握手（双方都不知道对方的 static key）
- Multiplex 使用 Yamux。如果你对 Yamux 不太了解，可以回想一下 HTTP/2 是如何做 multiplex 的， Yamux 的原理与之非常类似。

可以看到，短短几行代码，我们就构建了一个非常安全（noise protocol），非常高效（yamux），以及非常灵活（tcp / websocket）的传输协议。

### 网络行为：Network behaviour

如果说 `Transport` 定义了 **如何在网络中传输数据**，那么 `NetworkBehaviour` 则定义了 **在网络中要传输什么样的数据**，或者说，你的 p2p 协议本身。你只需要专注于数据本身而不必考虑最终数据是如何加密，使用什么协议发送出去。我觉得这个设计非常优雅，它清晰地像我们展示了什么是 Separation of Concerns。

libp2p 自带了一系列 Network behaviour —— 上图中大部分的协议，都通过 Network behaviour 实现，我举几个例子：

- Ping：节点和其 peer 之间 keepalive，以及网络诊断的工具。
- mDNS：本地节点发现协议。
- floodsub：floodsub protocol 实现。它是 libp2p 几种 pubsub 方案之一。适用于小型网络中消息的广播。
- kad：Kademlia 协议的实现。

除了 libp2p 默认实现的一系列 Network behaviour 外，你也可以创建新的 Network behaviour，并且把你的协议和已有的 behaviour 组合在一起。

要实现你自己的 Network behaviour，你需要：

- 实现 `UpgradeInfo`：这告诉 libp2p 你的协议的唯一标识符，比如：`/ipfs/ping/1.0.0`。
- 实现 `InboundUpgrade` 和 `OutboundUpgrade`：这是协议输入输出数据的处理。
- 实现 `ProtocolsHandler` 和  `NetworkBehaviour`：协议的主要处理流程。

### Swarm

上文提到 `Network` 定义了如何发送数据，`NetworkBehaviour` 定义了什么时候发送什么样的数据，但我们还缺一个中间人，把 `NetworkBehaviour` 要发送的数据交给 `Network` 发送出去，并且把 `Network` 收到的数据交给 `NetworkBehaviour` 处理。这个中间人就是 `Swarm`。

我们用一段代码看看 `Swarm` 是如何驱动 `Network` 和 `NetworkBehaviour` 前进的：

```rust
let transport = ...; // 参见上文代码  
let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));  
let mut swarm = Swarm::new(transport, behaviour, local_peer_id);  
swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;  
loop {  
    let event = swarm.next().await?;  
    println!("event: {:?}", event);  
}
```

这里可以看到，`Swarm` 需要 `Network` 和 `NetworkBehaviour` 才能创建。swarm 可以监听一个特殊的地址 —— 这里 `/ip4/0.0.0.0/tcp/0` 是一个 IPFS 定义的 multiaddr，可以很方便地表述多层协议的地址。之后，我们可以通过 `swarm.next()` 来驱动 `Swarm` 处理数据。一般而言，`Swarm` 获取到的数据，已经交给 `NetworkBehaviour` 处理了，只有未定义的数据，才会被收到上文中的流程进行进一步处理。

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

- **`/ip4/127.0.0.1/tcp/4001/ipfs/QmNodeID`**: 这种格式跟传统的 TCP 网络里是一样的，直接可以解析出对应的 IPv4 地址和端口号；
- **`/ipfs/QmNodeID`**: 这种格式的地址适用于 IPFS 网络，只有节点ID的地址，需要节点路由模块找到节点对应的IP地址，然后再进行连接；
- **`/dns4/http://ipfs.ipfsbit.com/tcp/443/wss/p2p-webrtc-star`**: 这种地址需要调用multiaddr-dns组件，把域名解析成IP地址，然后再进行连接；
- **`/p2p-circuit/p2p/QmNodeID`**: 这种地址是relay地址，用于中继网络，需要首先连接一个中继节点，才能连接到目的节点；

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

## 参考文档

[1] Telecommunication systems and technology: https://www.slideserve.com/myrrh/telecomunication-systems-and-technology-powerpoint-ppt-presentation

[2] A network framework for decentralized P2P application development: https://ipfs.io/ipfs/QmdzerM4fsnNGVf5jnogxu6QpXQa5rHDK87oHStC5xGCS4/

[3] rust-libp2p: https://docs.rs/libp2p/0.50.0/libp2p/tutorials/ping/index.html

[4] 探索 libp2p：基本知识: https://mp.weixin.qq.com/s/0dtf-OrPb01iRuTl1Bv-0Q

[5] rust_libp2p入门教程: https://mp.weixin.qq.com/s/p3g3fqYlKkb0ot6bTAsSTA
