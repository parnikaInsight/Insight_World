//! identify example
//!
//! In the first terminal window, run:
//!
//! ```sh
//! cargo run --example identify
//! ```
//! It will print the [`PeerId`] and the listening addresses, e.g. `Listening on
//! "/ip4/127.0.0.1/tcp/24915"`
//!
//! In the second terminal window, start a new instance of the example with:
//!
//! ```sh
//! cargo run --example identify -- /ip4/127.0.0.1/tcp/24915
//! ```
//! The two nodes establish a connection, negotiate the identify protocol
//! and will send each other identify info which is then printed to the console.

use bevy::prelude::*;
use async_std::task;
use futures::StreamExt;
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::{GetClosestPeersError, Kademlia, KademliaConfig, KademliaEvent, QueryResult};
use libp2p::{
    identify, 
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    development_transport, identity, 
    swarm::{Swarm, SwarmEvent},
    Multiaddr, PeerId,
};
use libp2p::core::identity::Keypair;
use std::{env, thread, error::Error, str::FromStr, time::Duration};

use crate::networks::connection::swarm;
use crate::networks::behavior::my_behavior;

pub async fn identify(
    local_key: Keypair, 
    local_peer_id: PeerId, 
) -> Result<(), Box<dyn Error>> {

    let mut swarm = swarm::create_swarm(local_key, local_peer_id).await?;

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Dial the peer identified by the multi-address given as the second
    // command-line argument, if any.
    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {}", addr)
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {:?}", address),
            // Prints peer id identify info is being sent to.
            SwarmEvent::Behaviour(my_behavior::Event::Identify(identify::Event::Sent { peer_id, .. })) => {
                println!("Sent identify info to {:?}", peer_id)
            }
            // Prints out the info received via the identify event
            SwarmEvent::Behaviour(my_behavior::Event::Identify(identify::Event::Received { info, .. })) => {
                println!("Received {:?}", info)
            }
            _ => {}
        }
    }
}
