use anyhow::Result;
use std::borrow::Cow;
use futures::StreamExt;
use libp2p::{identity, mdns, Multiaddr, noise, PeerId, Swarm, tcp, Transport, yamux};
use libp2p::core::upgrade;
use libp2p::floodsub::{Floodsub, FloodsubEvent, Topic};
use libp2p::swarm::{NetworkBehaviour, SwarmEvent};
use tokio::io::{AsyncBufReadExt, BufReader, stdin};

/// 处理 p2p 网络的 behavior 数据结构
/// 里面的每个域需要实现 NetworkBehaviour, 或者使用 #[behaviour(ignore)]
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "ChatBehaviorEvent")]
struct ChatBehavior {
    /// flood subscription, 比较浪费带宽, gossipsub 是更好的选择
    floodsub: Floodsub,
    /// 本地节点发现机制
    mdns: mdns::tokio::Behaviour,
    // 在 behavior 结构中，你也可以放其他数据，但需要 ignore
    // #[behaviour(ignore)]
    // _useless: String,
}

#[allow(clippy::large_enum_variant)]
enum ChatBehaviorEvent {
    Floodsub(FloodsubEvent),
    Mdns(mdns::Event),
}

impl ChatBehavior {
    /// 创建一个新的 ChatBehavior
    pub async fn new(id: PeerId) -> Result<Self> {
        Ok(Self {
            floodsub: Floodsub::new(id),
            mdns: mdns::Behaviour::new(Default::default())?,
        })
    }
}

impl From<FloodsubEvent> for ChatBehaviorEvent {
    fn from(event: FloodsubEvent) -> Self {
        ChatBehaviorEvent::Floodsub(event)
    }
}

impl From<mdns::Event> for ChatBehaviorEvent {
    fn from(event: mdns::Event) -> Self {
        ChatBehaviorEvent::Mdns(event)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // 如果带参数，当成一个 topic
    let name = match std::env::args().nth(1) {
        Some(arg) => Cow::Owned(arg),
        None => Cow::Borrowed("lobby"),
    };

    // 创建 floodsub topic
    let topic = Topic::new(name);

    // 创建 swarm
    let mut swarm = create_swarm(topic.clone()).await?;

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // 获取 stdin 的每一行
    let mut stdin = BufReader::new(stdin()).lines();

    // main loop
    loop {
        tokio::select! {
            line = stdin.next_line() => {
                let line = line?.expect("stdin closed");
                swarm.behaviour_mut().floodsub.publish(topic.clone(), line.as_bytes());
            }
            event = swarm.select_next_some () => {
                match event {
                    SwarmEvent::NewListenAddr {address,..} => {println!("Listening on {address:?}");}
                    SwarmEvent::Behaviour(ChatBehaviorEvent::Floodsub(FloodsubEvent::Message(message))) => {
                        println!("Received: '{:?}' from {:?}", String::from_utf8_lossy(&message.data), message.source);
                    }
                    SwarmEvent::Behaviour(ChatBehaviorEvent::Mdns(event)) => {
                        match event {
                            mdns::Event::Discovered(list) => {
                                for (peer, _) in list {
                                    swarm.behaviour_mut().floodsub.add_node_to_partial_view(peer);
                                }
                            }
                            mdns::Event::Expired(list) => {
                                for (peer, _) in list {
                                    if !swarm.behaviour().mdns.has_node(&peer ) {
                                        swarm.behaviour_mut().floodsub.remove_node_from_partial_view(&peer);
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

async fn create_swarm(topic: Topic) -> Result<Swarm<ChatBehavior>> {
    // 创建 identity(密钥对)
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {:?}", peer_id);

    // 使用 noise protocol 来处理加密和认证
    let noise_keys = noise::Keypair::<noise::X25519Spec>::new().into_authentic(&id_keys)?;

    // 创建传输层
    let transport = tcp::tokio::Transport::new(tcp::Config::default().nodelay(true))
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
        .multiplex(yamux::YamuxConfig::default())
        .boxed();

    // 创建 chat behaviour
    let mut behavior = ChatBehavior::new(peer_id).await?;
    // 订阅某个主题
    behavior.floodsub.subscribe(topic.clone());
    // 创建 swarm
    let mut swarm = Swarm::with_tokio_executor(transport, behavior, peer_id);

    // 如果指定，则连接到另一个节点
    if let Some(to_dial) = std::env::args().nth(2) {
        let addr: Multiaddr = to_dial.parse()?;
        swarm.dial(addr)?;
        println!("Dialed {to_dial:?}");
    }

    Ok(swarm)
}
