use std::borrow::Cow;
use std::error::Error;

use async_std::io;
use futures::{
    prelude::{*, stream::StreamExt},
    select,
};
use libp2p::{
    floodsub::{self, Floodsub, FloodsubEvent},
    identity, mdns,
    Multiaddr,
    PeerId, swarm::{NetworkBehaviour, SwarmEvent}, Swarm,
};

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Create a random PeerId
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {local_peer_id:?}");

    // Set up an encrypted DNS-enabled TCP Transport over the Mplex and Yamux protocols
    let transport = libp2p::development_transport(local_key).await?;

    // Create a Floodsub topic
    let floodsub_topic = floodsub::Topic::new("chat");

    // We create a custom network behaviour that combines floodsub and mDNS.
    // Use the derive to generate delegating NetworkBehaviour impl.
    #[derive(NetworkBehaviour)]
    #[behaviour(out_event = "OutEvent")]
    struct MyBehaviour {
        floodsub: Floodsub,
        mdns: mdns::async_io::Behaviour,
    }

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug)]
    enum OutEvent {
        Floodsub(FloodsubEvent),
        Mdns(mdns::Event),
    }

    impl From<mdns::Event> for OutEvent {
        fn from(v: mdns::Event) -> Self {
            Self::Mdns(v)
        }
    }

    impl From<FloodsubEvent> for OutEvent {
        fn from(v: FloodsubEvent) -> Self {
            Self::Floodsub(v)
        }
    }

    // Create a Swarm to manage peers and events
    let mut swarm = {
        let mdns = mdns::async_io::Behaviour::new(mdns::Config::default())?;
        let mut behaviour = MyBehaviour {
            floodsub: Floodsub::new(local_peer_id),
            mdns,
        };

        behaviour.floodsub.subscribe(floodsub_topic.clone());
        Swarm::with_threadpool_executor(transport, behaviour, local_peer_id)
    };

    // 获取用户名
    let name = match std::env::args().nth(1) {
        Some(arg) => Cow::Owned(arg),
        None => Cow::Borrowed("anonymous"),
    };

    // Reach out to another node if specified
    if let Some(to_dial) = std::env::args().nth(2) {
        let addr: Multiaddr = to_dial.parse()?;
        swarm.dial(addr)?;
        println!("Dialed {to_dial:?}")
    }

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();

    // Listen on all interfaces and whatever port the OS assigns
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Kick it off
    loop {
        select! {
            line = stdin.select_next_some() => swarm
                .behaviour_mut()
                .floodsub
                .publish(floodsub_topic.clone(), line.expect("Stdin not to close").as_bytes()),
            event = swarm.select_next_some() => match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening on {address:?}");
                }
                SwarmEvent::Behaviour(OutEvent::Floodsub(
                    FloodsubEvent::Message(message)
                )) => {
                    let content = String::from_utf8_lossy(&message.data);
                    if content.contains('@') {
                        let username = format!("@{}", name);
                        if content.contains(&username) {
                            println!(
                                "Received: '{:?}' from {:?}",
                                content,
                                message.source
                            );
                        }
                    } else {
                        println!(
                            "Received: '{:?}' from {:?}",
                            content,
                            message.source
                        );
                    }
                }
                SwarmEvent::Behaviour(OutEvent::Mdns(
                    mdns::Event::Discovered(list)
                )) => {
                    for (peer, _) in list {
                        swarm
                            .behaviour_mut()
                            .floodsub
                            .add_node_to_partial_view(peer);
                        // println!("{peer:?} discovered");
                    }
                }
                // SwarmEvent::Behaviour(OutEvent::Mdns(mdns::Event::Expired(
                //     list
                // ))) => {
                //     for (peer, _) in list {
                //         if !swarm.behaviour_mut().mdns.has_node(&peer) {
                //             swarm
                //                 .behaviour_mut()
                //                 .floodsub
                //                 .remove_node_from_partial_view(&peer);
                //             println!("{peer:?} expired");
                //         }
                //     }
                // },
                _ => {}
            }
        }
    }
}
