//! Demonstrates how to perform Kademlia queries on the IPFS network.
//!
//! You can pass as parameter a base58 peer ID to search for. If you don't pass any parameter, a
//! peer ID will be generated randomly.

use bevy::prelude::*;
use async_std::task;
use futures::StreamExt;
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::{GetClosestPeersError, Kademlia, KademliaConfig, KademliaEvent, QueryResult};
use libp2p::{
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    development_transport, identity, 
    swarm::{Swarm, SwarmEvent},
    Multiaddr, PeerId,
};
use libp2p::core::identity::Keypair;
use std::{env, thread, error::Error, str::FromStr, time::Duration};

use crate::connection::swarm;
use crate::behavior::my_behavior;

pub async fn mdns(
    local_key: Keypair, 
    local_peer_id: PeerId, 
) -> Result<(), Box<dyn Error>> {

    let mut swarm = swarm::create_swarm(local_key, local_peer_id).await?;

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::Behaviour(my_behavior::Event::Mdns(MdnsEvent::Discovered(peers))) => {
                for (peer, addr) in peers {
                    println!("discovered {} {}", peer, addr);
                }
            }
            SwarmEvent::Behaviour(my_behavior::Event::Mdns(MdnsEvent::Expired(expired))) => {
                for (peer, addr) in expired {
                    println!("expired {} {}", peer, addr);
                }
            }
            _ => {}
        }
    }
}