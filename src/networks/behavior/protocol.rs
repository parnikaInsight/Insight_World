use async_std::io;
use bevy::prelude::*;
use futures::AsyncBufReadExt;
use futures::StreamExt;
use futures::select;
use libp2p::core::identity::Keypair;
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::{
    record::Key, AddProviderOk, GetClosestPeersError, Kademlia, KademliaConfig, KademliaEvent,
    PeerRecord, PutRecordOk, QueryResult, Quorum, Record,
};
use libp2p::{
    development_transport, identify, ping, autonat, dcutr,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::{Swarm, SwarmEvent},
    Multiaddr, PeerId, relay::v2,
};
use std::{env, error::Error, str::FromStr, thread, time::Duration};

use crate::behavior::{kademlia, my_behavior};
use crate::connection::swarm;

use super::my_behavior::MyBehavior;

pub async fn process_swarm_events(
    local_key: Keypair,
    local_peer_id: PeerId,
) -> Result<(), Box<dyn Error>> {
    let mut swarm = swarm::create_swarm(local_key, local_peer_id).await?;

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    println!("My PeerId: {}", local_peer_id);

    // Order Kademlia to search for a peer.

    // For Ping and Identify - dial multiaddress
    // Dial the peer identified by the multi-address given as the second
    // command-line argument, if any.
    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {}", addr)
    }

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();

    loop {
        select! {
            line = stdin.select_next_some() => kademlia::handle_input_line(&mut swarm.behaviour_mut().kademlia, line.expect("Stdin not to close")),
            event = swarm.select_next_some() =>
            match event {

                // // Mdns
                // SwarmEvent::Behaviour(my_behavior::Event::Mdns(MdnsEvent::Discovered(peers))) => {
                //     for (peer_id, addr) in peers {
                //         println!("discovered {} {}", peer_id, addr);
                //         swarm.behaviour_mut().kademlia.add_address(&peer_id, addr);
                //     }
                // }
                // SwarmEvent::Behaviour(my_behavior::Event::Mdns(MdnsEvent::Expired(expired))) => {
                //     for (peer, addr) in expired {
                //         println!("expired {} {}", peer, addr);
                //     }
                // }

                // // Kademlia (needs mdns)
                // SwarmEvent::Behaviour(my_behavior::Event::Kademlia(
                //     KademliaEvent::OutboundQueryCompleted { result, ..},
                // )) => {
                //     kademlia::kademlia_query_results(result);
                // },

                // // Identify (dial multiaddress)
                // SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {:?}", address),
                // // Prints peer id identify info is being sent to.
                // SwarmEvent::Behaviour(my_behavior::Event::Identify(identify::Event::Sent { peer_id, .. })) => {
                //     println!("Sent identify info to {:?}", peer_id)
                // }
                // // Prints out the info received via the identify event
                // SwarmEvent::Behaviour(my_behavior::Event::Identify(identify::Event::Received { info, .. })) => {
                //     println!("Received {:?}", info)
                // }

                // // Ping (dial multiaddress)
                // SwarmEvent::Behaviour(my_behavior::Event::Ping(ping::Event{ peer, result})) => {
                //     println!("Ping {:?} {:?}", peer, result);
                // },

                // // Autonat
                // SwarmEvent::Behaviour(my_behavior::Event::Autonat(autonat::Event::InboundProbe(ip_event))) => {
                //     println!("autonat inbound {:?}", ip_event);
                // },
                // SwarmEvent::Behaviour(my_behavior::Event::Autonat(autonat::Event::OutboundProbe(op_event))) => {
                //     println!("autonat outbound {:?}", op_event);
                // },
                // SwarmEvent::Behaviour(my_behavior::Event::Autonat(autonat::Event::StatusChanged {old, new})) => {
                //     println!("autonat status changed- old: {:?}, new: {:?}", old, new);
                //     // if new status is private, act as client relay
                //     // if new status is public, no need for holepunching and can start advertising listen address
                //     // can also act as relay node
                // },

                // // Relay Client
                // SwarmEvent::Behaviour(my_behavior::Event::RelayClient(v2::client::Event::ReservationReqAccepted {relay_peer_id, renewal, limit})) => {
                //     println!("1 ReservationReqAccepted {:?}", relay_peer_id);
                // },
                // SwarmEvent::Behaviour(my_behavior::Event::RelayClient(v2::client::Event::ReservationReqFailed {relay_peer_id, renewal, error})) => {
                //     println!("2 ReservationReqFailed {:?}", relay_peer_id);
                // },
                // SwarmEvent::Behaviour(my_behavior::Event::RelayClient(v2::client::Event::OutboundCircuitEstablished {relay_peer_id, limit})) => {
                //     println!("3 OutboundCircuitEstablished {:?}", relay_peer_id);
                // },
                // SwarmEvent::Behaviour(my_behavior::Event::RelayClient(v2::client::Event::OutboundCircuitReqFailed {relay_peer_id, error})) => {
                //     println!("4 OutboundCircuitReqFailed {:?}", relay_peer_id);
                // },
                // SwarmEvent::Behaviour(my_behavior::Event::RelayClient(v2::client::Event::InboundCircuitEstablished {src_peer_id, limit})) => {
                //     println!("5 InboundCircuitEstablished {:?}", src_peer_id);
                // },
                // SwarmEvent::Behaviour(my_behavior::Event::RelayClient(v2::client::Event::InboundCircuitReqFailed {relay_peer_id, error})) => {
                //     println!("6 InboundCircuitReqFailed {:?}", relay_peer_id);
                // },
                // SwarmEvent::Behaviour(my_behavior::Event::RelayClient(v2::client::Event::InboundCircuitReqDenied {src_peer_id})) => {
                //     println!("7 InboundCircuitReqDenied {:?}", src_peer_id);
                // },
                // SwarmEvent::Behaviour(my_behavior::Event::RelayClient(v2::client::Event::InboundCircuitReqDenyFailed {src_peer_id, error})) => {
                //     println!("8 InboundCircuitReqDenyFailed {:?}", src_peer_id);
                // },

                // DCUtR
                SwarmEvent::Behaviour(my_behavior::Event::Dcutr(dcutr::behaviour::Event::InitiatedDirectConnectionUpgrade {remote_peer_id, local_relayed_addr})) => {
                    println!("1 InitiatedDirectConnectionUpgrade {:?}", remote_peer_id);
                },
                SwarmEvent::Behaviour(my_behavior::Event::Dcutr(dcutr::behaviour::Event::RemoteInitiatedDirectConnectionUpgrade {remote_peer_id, remote_relayed_addr})) => {
                    println!("1 InitiatedDirectConnectionUpgrade {:?}", remote_peer_id);
                },
                SwarmEvent::Behaviour(my_behavior::Event::Dcutr(dcutr::behaviour::Event::DirectConnectionUpgradeSucceeded {remote_peer_id})) => {
                    println!("1 InitiatedDirectConnectionUpgrade {:?}", remote_peer_id);
                },
                SwarmEvent::Behaviour(my_behavior::Event::Dcutr(dcutr::behaviour::Event::DirectConnectionUpgradeFailed {remote_peer_id, error})) => {
                    println!("1 InitiatedDirectConnectionUpgrade {:?}", remote_peer_id);
                },
                

                _ => {}
            }
        }
    }
}