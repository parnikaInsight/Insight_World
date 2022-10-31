use async_std::task;
use futures::StreamExt;
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::{GetClosestPeersError, Kademlia, KademliaConfig, KademliaEvent, QueryResult};
use libp2p::{
    development_transport, identity, 
    swarm::{Swarm, SwarmEvent},
    Multiaddr, PeerId,
};
use libp2p::core::identity::Keypair;
use std::{env, thread, error::Error, str::FromStr, time::Duration};
use libp2p::{
    futures::select,
    gossipsub::{Gossipsub, GossipsubEvent, IdentTopic as Topic},
    identify, ping,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    request_response::{
        ProtocolSupport, RequestResponse, RequestResponseEvent, RequestResponseMessage,
    }, 
};

use crate::behavior::my_behavior::MyBehavior;

const BOOTNODES: [&str; 4] = [
    "QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN",
    "QmQCU2EcMqAqQPR2i9bChDtGNJchTbq5TbXJJ16u19uLTa",
    "QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb",
    "QmcZf59bWwK5XFi76CZX8cbJ4BhTzzA3gU1ZjYZcYW3dwt",
];

pub async fn create_swarm(local_key: Keypair, local_peer_id: PeerId) -> std::io::Result<Swarm<MyBehavior>> {
    
    let transport = development_transport(local_key.clone()).await?;

    // Create a swarm to manage peers and events.
    let mut swarm = {
        // Create a Kademlia behaviour.
        // let mut cfg = KademliaConfig::default();
        // cfg.set_query_timeout(Duration::from_secs(5 * 60));
        let store = MemoryStore::new(local_peer_id);
        // let mut kademlia = Kademlia::with_config(local_peer_id, store, cfg);
        let mut kademlia = Kademlia::new(local_peer_id, store);
        
        // // Add the bootnodes to the local routing table. `libp2p-dns` built
        // // into the `transport` resolves the `dnsaddr` when Kademlia tries
        // // to dial these nodes.
        // let bootaddr = Multiaddr::from_str("/dnsaddr/bootstrap.libp2p.io").unwrap();
        // for peer in &BOOTNODES {
        //     kademlia.add_address(&PeerId::from_str(peer).unwrap(), bootaddr.clone());
        // }

        let mdns = Mdns::new(MdnsConfig::default())?;
        
        let identify = identify::Behaviour::new(identify::Config::new(
            "1.0".to_string(),
            local_key.clone().public(),
        ));
        
        let ping = ping::Behaviour::new(ping::Config::new().with_keep_alive(true));

        let behaviour = MyBehavior {
            kademlia,
            identify,
            mdns,
            ping,
        };

        Swarm::new(transport, behaviour, local_peer_id)
    };
    Ok(swarm)
}