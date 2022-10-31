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
use std::{env, thread, error::Error, str::FromStr, time::Duration};
use futures::executor::block_on;

mod behavior;
mod connection;

use behavior::{kademlia, mdns, identify, protocol};
use connection::{swarm};


// #[async_std::main]
fn main() -> Result<(), Box<dyn Error>> {

    // Create a random key for ourselves.
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    let my_future = protocol::process_swarm_events(local_key.clone(), local_peer_id);
    thread::spawn(move || block_on(my_future).expect("Thread Spawn Error"));

    App::new()
        .add_plugins(DefaultPlugins)
        .run();

    Ok(())
}