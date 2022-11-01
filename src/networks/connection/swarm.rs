//use async_std::stream::StreamExt;
use async_std::task;
use futures::StreamExt;
use libp2p::core::identity::Keypair;
use libp2p::core::muxing::StreamMuxerBox;
use libp2p::core::transport::{Boxed, OrTransport};
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::{GetClosestPeersError, Kademlia, KademliaConfig, KademliaEvent, QueryResult};
use libp2p::multiaddr::Protocol;
use libp2p::relay::v2::client::Client;
use libp2p::{
    autonat, dcutr,
    futures::select,
    gossipsub::{Gossipsub, GossipsubEvent, IdentTopic as Topic},
    identify,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    ping,
    relay::v2,
    request_response::{
        ProtocolSupport, RequestResponse, RequestResponseEvent, RequestResponseMessage,
    },
};
use libp2p::{core, dns, identity, mplex, noise, tcp, websocket, yamux, PeerId, Transport};

use libp2p::{
    development_transport,
    swarm::{Swarm, SwarmEvent},
    Multiaddr,
};
use std::net::Ipv4Addr;
use std::{env, error::Error, str::FromStr, thread, time::Duration};

use crate::behavior::my_behavior::MyBehavior;

const BOOTNODES: [&str; 4] = [
    "QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN",
    "QmQCU2EcMqAqQPR2i9bChDtGNJchTbq5TbXJJ16u19uLTa",
    "QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb",
    "QmcZf59bWwK5XFi76CZX8cbJ4BhTzzA3gU1ZjYZcYW3dwt",
];

// pub async fn create_transport(
//     local_key: Keypair,
//     local_peer_id: PeerId,
// ) -> std::io::Result<Boxed<(PeerId, StreamMuxerBox)>>{
//     let main_transport = development_transport(local_key.clone()).await?;
//     let (mut relay_transport, relay_client) =
//             v2::client::Client::new_transport_and_behaviour(local_peer_id);
//     let transport = main_transport.or_transport(relay_transport);
//     Ok(transport.boxed())
// }

pub async fn build_transport(
    keypair: identity::Keypair,
    local_peer_id: PeerId,
) -> std::io::Result<(
    core::transport::Boxed<(PeerId, core::muxing::StreamMuxerBox)>,
    Client,
)> {
    let (mut relay_transport, relay_client) =
        v2::client::Client::new_transport_and_behaviour(local_peer_id);
    let transport = {
        let dns_tcp = dns::DnsConfig::system(tcp::TcpTransport::new(
            tcp::GenTcpConfig::new().nodelay(true),
        ))
        .await?;
        let ws_dns_tcp = websocket::WsConfig::new(
            dns::DnsConfig::system(tcp::TcpTransport::new(
                tcp::GenTcpConfig::new().nodelay(true),
            ))
            .await?,
        );
        let t = dns_tcp.or_transport(ws_dns_tcp);
        t.or_transport(relay_transport)
    };

    let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
        .into_authentic(&keypair)
        .expect("Signing libp2p-noise static DH keypair failed.");

    Ok((
        transport
            .upgrade(core::upgrade::Version::V1)
            .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
            .multiplex(core::upgrade::SelectUpgrade::new(
                yamux::YamuxConfig::default(),
                mplex::MplexConfig::default(),
            ))
            .timeout(std::time::Duration::from_secs(20))
            .boxed(),
        relay_client,
    ))
}

pub async fn create_swarm(
    local_key: Keypair,
    local_peer_id: PeerId,
) -> std::io::Result<Swarm<MyBehavior>> {
    //let transport = development_transport(local_key.clone()).await?;
    let (mut transport, relay_client) = build_transport(local_key.clone(), local_peer_id).await?;

    // Create a swarm to manage peers and events.
    let mut swarm = {
        // Create a Kademlia behaviour.
        // let mut cfg = KademliaConfig::default();
        // cfg.set_query_timeout(Duration::from_secs(5 * 60));
        let store = MemoryStore::new(local_peer_id);
        // let mut kademlia = Kademlia::with_config(local_peer_id, store, cfg);
        let mut kademlia = Kademlia::new(local_peer_id, store);

        // Add the bootnodes to the local routing table. `libp2p-dns` built
        // into the `transport` resolves the `dnsaddr` when Kademlia tries
        // to dial these nodes.
        let bootaddr = Multiaddr::from_str("/dnsaddr/bootstrap.libp2p.io").unwrap();
        for peer in &BOOTNODES {
            kademlia.add_address(&PeerId::from_str(peer).unwrap(), bootaddr.clone());
        }

        // Create a mdns behavior
        let mdns = Mdns::new(MdnsConfig::default())?;

        // Create an identify behavior
        let identify = identify::Behaviour::new(identify::Config::new(
            "1.0".to_string(),
            local_key.clone().public(),
        ));

        // Create a ping behavior
        let ping = ping::Behaviour::new(ping::Config::new().with_keep_alive(true));

        // Create an autonat behavior
        let mut autonat = autonat::Behaviour::new(local_peer_id, autonat::Config::default());
        // Specify servers
        for peer in &BOOTNODES {
            autonat.add_server(PeerId::from_str(peer).unwrap(), Some(bootaddr.clone()));
        }

        // Create a relay behavior
        // Relay transport and client created in build_transport
        // Establish relayed connections by dialing /p2p-circuit addresses.
        let dst_addr_via_relay = Multiaddr::empty()
            //.with(Protocol::Memory(40)) // Relay address. // "/ip4/147.75.87.27/tcp/4001
            .with(Protocol::Ip4(Ipv4Addr::new(147, 75, 87, 27)))
            .with(Protocol::Tcp(4001))
            .with(Protocol::P2p(
                PeerId::from_str("QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb")
                    .unwrap()
                    .into(),
            )) // Relay peer id.
            .with(Protocol::P2pCircuit) // Signal to connect via relay and not directly.
            .with(Protocol::P2p(
                PeerId::from_str("QmQCU2EcMqAqQPR2i9bChDtGNJchTbq5TbXJJ16u19uLTa")
                    .unwrap()
                    .into(),
            )); // Destination peer id.
        transport.dial(dst_addr_via_relay).unwrap();

        // Listen for incoming relayed connections via specific relay.
        let relay_addr = Multiaddr::empty()
            //.with(Protocol::Memory(40)) // Relay address.
            .with(Protocol::Ip4(Ipv4Addr::new(147, 75, 87, 27)))
            .with(Protocol::Tcp(4001))
            .with(Protocol::P2p(
                PeerId::from_str("QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb")
                    .unwrap()
                    .into(),
            )) // Relay peer id.
            .with(Protocol::P2pCircuit); // Signal to listen via remote relay node.
        transport.listen_on(relay_addr).unwrap();

        // Create a dcutr behavior
        let dcutr = dcutr::behaviour::Behaviour::new();
        // A and B must coordinate dial

        let behaviour = MyBehavior {
            kademlia,
            identify,
            mdns,
            ping,
            autonat,
            relay: relay_client,
            dcutr
        };

        Swarm::new(transport, behaviour, local_peer_id)
    };
    Ok(swarm)
}
