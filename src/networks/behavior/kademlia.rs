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
    development_transport, identity, 
    swarm::{Swarm, SwarmEvent},
    Multiaddr, PeerId,
};
use libp2p::core::identity::Keypair;
use std::{env, thread, error::Error, str::FromStr, time::Duration};

use crate::connection::swarm;
use crate::behavior::my_behavior;

pub async fn kademlia(
    local_key: Keypair, 
    local_peer_id: PeerId, 
) -> Result<(), Box<dyn Error>> {

    let mut swarm = swarm::create_swarm(local_key, local_peer_id).await?;

    // Order Kademlia to search for a peer.
    let to_search: PeerId = if let Some(peer_id) = env::args().nth(1) {
        peer_id.parse()?
    } else {
        identity::Keypair::generate_ed25519().public().into()
    };

    println!("Searching for the closest peers to {:?}", to_search);
    swarm.behaviour_mut().kademlia.get_closest_peers(to_search);

    // Kick it off!
    task::block_on(async move {
        loop {
            let event = swarm.select_next_some().await;
            if let SwarmEvent::Behaviour(my_behavior::Event::Kademlia(KademliaEvent::OutboundQueryCompleted {
                result: QueryResult::GetClosestPeers(result),
                ..
            })) = event
            {
                match result {
                    Ok(ok) => {
                        if !ok.peers.is_empty() {
                            println!("Query finished with closest peers: {:#?}", ok.peers)
                        } else {
                            // The example is considered failed as there
                            // should always be at least 1 reachable peer.
                            println!("Query finished with no closest peers.")
                        }
                    }
                    Err(GetClosestPeersError::Timeout { peers, .. }) => {
                        if !peers.is_empty() {
                            println!("Query timed out with closest peers: {:#?}", peers)
                        } else {
                            // The example is considered failed as there
                            // should always be at least 1 reachable peer.
                            println!("Query timed out with no closest peers.");
                        }
                    }
                };

                break;
            }
        }

        Ok(())
    })
}