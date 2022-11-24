//! Demonstrates how to perform Kademlia queries on the IPFS network.
//!
//! You can pass as parameter a base58 peer ID to search for. If you don't pass any parameter, a
//! peer ID will be generated randomly.
//!
//! //! A basic key value store demonstrating libp2p and the mDNS and Kademlia protocols.
//!
//! 1. Using two terminal windows, start two instances. If you local network
//!    allows mDNS, they will automatically connect.
//!
//! 2. Type `PUT my-key my-value` in terminal one and hit return.
//!
//! 3. Type `GET my-key` in terminal two and hit return.
//!
//! 4. Close with Ctrl-c.
//!
//! You can also store provider records instead of key value records.
//!
//! 1. Using two terminal windows, start two instances. If you local network
//!    allows mDNS, they will automatically connect.
//!
//! 2. Type `PUT_PROVIDER my-key` in terminal one and hit return.
//!
//! 3. Type `GET_PROVIDERS my-key` in terminal two and hit return.
//!
//! 4. Close with Ctrl-c.

use async_std::task;
use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};
use futures::StreamExt;
use libp2p::core::either::EitherError;
use libp2p::core::identity::Keypair;
use libp2p::gossipsub::subscription_filter::AllowAllSubscriptionFilter;
use libp2p::gossipsub::{Gossipsub, IdentityTransform, IdentTopic as Topic};
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::{
    record::Key, AddProviderOk, GetClosestPeersError, Kademlia, KademliaConfig, KademliaEvent,
    PeerRecord, PutRecordOk, QueryResult, Quorum, Record,
};
use libp2p::multihash::IdentityHasher;
use libp2p::{
    development_transport, identity,
    swarm::{Swarm, SwarmEvent},
    Multiaddr, PeerId,
};
use std::collections::hash_map::DefaultHasher;
use std::{env, error::Error, str::FromStr, thread, time::Duration};
use void::Void;

use crate::networks::behavior::my_behavior;
use crate::networks::connection::swarm;

use super::my_behavior::MyBehavior;

pub async fn kademlia(local_key: Keypair, local_peer_id: PeerId) -> Result<(), Box<dyn Error>> {
    let mut swarm = swarm::create_swarm(local_key, local_peer_id).await?;

    // // Order Kademlia to search for a peer.
    // let to_search: PeerId = if let Some(peer_id) = env::args().nth(1) {
    //     peer_id.parse()?
    // } else {
    //     identity::Keypair::generate_ed25519().public().into()
    // };

    // println!("Searching for the closest peers to {:?}", to_search);
    // swarm.behaviour_mut().kademlia.get_closest_peers(to_search);

    // Kick it off!
    task::block_on(async move {
        loop {
            let event = swarm.select_next_some().await;
            if let SwarmEvent::Behaviour(my_behavior::Event::Kademlia(
                KademliaEvent::OutboundQueryCompleted { result, .. },
            )) = event
            {
                match result {
                    QueryResult::GetClosestPeers(Ok(ok)) => {
                        if !ok.peers.is_empty() {
                            println!("Query finished with closest peers: {:#?}", ok.peers)
                        } else {
                            // The example is considered failed as there
                            // should always be at least 1 reachable peer.
                            println!("Query finished with no closest peers.")
                        }
                    }
                    QueryResult::GetClosestPeers(Err(GetClosestPeersError::Timeout {
                        peers,
                        ..
                    })) => {
                        if !peers.is_empty() {
                            println!("Query timed out with closest peers: {:#?}", peers)
                        } else {
                            // The example is considered failed as there
                            // should always be at least 1 reachable peer.
                            println!("Query timed out with no closest peers.");
                        }
                    }
                    QueryResult::GetProviders(Ok(ok)) => {
                        for peer in ok.providers {
                            println!(
                                "Peer {:?} provides key {:?}",
                                peer,
                                std::str::from_utf8(ok.key.as_ref()).unwrap()
                            );
                        }
                    }
                    QueryResult::GetProviders(Err(err)) => {
                        eprintln!("Failed to get providers: {:?}", err);
                    }
                    QueryResult::GetRecord(Ok(ok)) => {
                        for PeerRecord {
                            record: Record { key, value, .. },
                            ..
                        } in ok.records
                        {
                            println!(
                                "Got record {:?} {:?}",
                                std::str::from_utf8(key.as_ref()).unwrap(),
                                std::str::from_utf8(&value).unwrap(),
                            );
                        }
                    }
                    QueryResult::GetRecord(Err(err)) => {
                        eprintln!("Failed to get record: {:?}", err);
                    }
                    QueryResult::PutRecord(Ok(PutRecordOk { key })) => {
                        println!(
                            "Successfully put record {:?}",
                            std::str::from_utf8(key.as_ref()).unwrap()
                        );
                    }
                    QueryResult::PutRecord(Err(err)) => {
                        eprintln!("Failed to put record: {:?}", err);
                    }
                    QueryResult::StartProviding(Ok(AddProviderOk { key })) => {
                        println!(
                            "Successfully put provider record {:?}",
                            std::str::from_utf8(key.as_ref()).unwrap()
                        );
                    }
                    QueryResult::StartProviding(Err(err)) => {
                        eprintln!("Failed to put provider record: {:?}", err);
                    }
                    _ => {}
                };

                break;
            }
        }

        Ok(())
    })
}

pub fn kademlia_query_results(result: QueryResult) {
    match result {
        QueryResult::GetClosestPeers(Ok(ok)) => {
            if !ok.peers.is_empty() {
                println!("Query finished with closest peers: {:#?}", ok.peers)
            } else {
                // The example is considered failed as there
                // should always be at least 1 reachable peer.
                println!("Query finished with no closest peers.")
            }
        }
        QueryResult::GetClosestPeers(Err(GetClosestPeersError::Timeout { peers, .. })) => {
            if !peers.is_empty() {
                println!("Query timed out with closest peers: {:#?}", peers)
            } else {
                // The example is considered failed as there
                // should always be at least 1 reachable peer.
                println!("Query timed out with no closest peers.");
            }
        }
        QueryResult::GetProviders(Ok(ok)) => {
            for peer in ok.providers {
                println!(
                    "Peer {:?} provides key {:?}",
                    peer,
                    std::str::from_utf8(ok.key.as_ref()).unwrap()
                );
            }
        }
        QueryResult::GetProviders(Err(err)) => {
            eprintln!("Failed to get providers: {:?}", err);
        }
        QueryResult::GetRecord(Ok(ok)) => {
            for PeerRecord {
                record: Record { key, value, .. },
                ..
            } in ok.records
            {
                println!(
                    "Got record {:?} {:?}",
                    std::str::from_utf8(key.as_ref()).unwrap(),
                    std::str::from_utf8(&value).unwrap(),
                );
            }
        }
        QueryResult::GetRecord(Err(err)) => {
            eprintln!("Failed to get record: {:?}", err);
        }
        QueryResult::PutRecord(Ok(PutRecordOk { key })) => {
            println!(
                "Successfully put record {:?}",
                std::str::from_utf8(key.as_ref()).unwrap()
            );
        }
        QueryResult::PutRecord(Err(err)) => {
            eprintln!("Failed to put record: {:?}", err);
        }
        QueryResult::StartProviding(Ok(AddProviderOk { key })) => {
            println!(
                "Successfully put provider record {:?}",
                std::str::from_utf8(key.as_ref()).unwrap()
            );
        }
        QueryResult::StartProviding(Err(err)) => {
            eprintln!("Failed to put provider record: {:?}", err);
        }
        _ => {}
    };
}

pub fn handle_input_line(
    // kademlia: &mut Kademlia<MemoryStore>,
    // gossipsub: &mut Gossipsub<IdentityTransform, AllowAllSubscriptionFilter>,
    swarm: &mut MyBehavior,
    //topic: Topic, 
    line: String,
    networks_sender: Sender<String>,
    networks_receiver: Receiver<String>,
) {
    let mut args = line.split(' ');

    let res = networks_receiver.recv();
    match res {
        Ok(string) => {
            if string == "PUBLISH" {
                // only works if GET_PROVIDERS is typed soon after publishing--must be polling for metaverse updates automatically
                println!("publisheeeed");
                swarm.kademlia
                    .start_providing(Key::new(&String::from("experiment_world")))
                    .expect("Failed to start providing key");
                // Send notification to gamers about having published a metaverse update
                // if let Err(e) = swarm.gossipsub.publish(topic, "experiment_world published") {
                //     println!("Publish error: {:?}", e);
                // }
                //WRONG- networks_sender.send(String::from("NOW_PROV"));
            }
        }
        _ => (),
    }

    match args.next() {
        Some("GET") => {
            let key = {
                match args.next() {
                    Some(key) => Key::new(&key),
                    None => {
                        eprintln!("Expected key");
                        return;
                    }
                }
            };
            swarm.kademlia.get_record(key, Quorum::One);
        }
        Some("GET_PROVIDERS") => {
            let key = {
                match args.next() {
                    Some(key) => Key::new(&key),
                    None => {
                        eprintln!("Expected key");
                        return;
                    }
                }
            };
            swarm.kademlia.get_providers(key);
        }
        Some("PUT") => {
            let key = {
                match args.next() {
                    Some(key) => Key::new(&key),
                    None => {
                        eprintln!("Expected key");
                        return;
                    }
                }
            };
            let value = {
                match args.next() {
                    Some(value) => value.as_bytes().to_vec(),
                    None => {
                        eprintln!("Expected value");
                        return;
                    }
                }
            };
            let record = Record {
                key,
                value,
                publisher: None,
                expires: None,
            };
            swarm. kademlia
                .put_record(record, Quorum::One)
                .expect("Failed to store record locally.");
        }
        Some("PUT_PROVIDER") => {
            let key = {
                match args.next() {
                    Some(key) => Key::new(&key),
                    None => {
                        eprintln!("Expected key");
                        return;
                    }
                }
            };

            swarm.kademlia
                .start_providing(key)
                .expect("Failed to start providing key");
        }
        _ => {
            eprintln!("expected GET, GET_PROVIDERS, PUT or PUT_PROVIDER");
        }
    }
}
