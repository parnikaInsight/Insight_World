use futures::executor::LocalPool;
use futures::future::FutureExt;
use futures::io::{AsyncRead, AsyncWrite};
use futures::stream::StreamExt;
use futures::task::Spawn;
use libp2p::core::multiaddr::{Multiaddr, Protocol};
use libp2p::core::muxing::StreamMuxerBox;
use libp2p::core::transport::choice::OrTransport;
use libp2p::core::transport::{Boxed, MemoryTransport, Transport};
use libp2p::core::PublicKey;
use libp2p::core::{identity, upgrade, PeerId};
use libp2p::ping::{Ping, PingConfig, PingEvent};
use libp2p::plaintext::PlainText2Config;
use libp2p::relay::v2::client;
use libp2p::relay::v2::relay;
use libp2p::NetworkBehaviour;
use libp2p::{ping, relay::v2};
use libp2p::swarm::{AddressScore, NetworkBehaviour, Swarm, SwarmEvent};
use std::time::Duration;

use super::my_behavior;

pub async fn wait_for_reservation(
    client: &mut Swarm<my_behavior::MyBehavior>,
    client_addr: Multiaddr,
    relay_peer_id: PeerId,
    is_renewal: bool,
) {
    let mut new_listen_addr = false;
    let mut reservation_req_accepted = false;

    loop {
        match client.select_next_some().await {
            //SwarmEvent::Behaviour(ClientEvent::Relay(client::Event::ReservationReqAccepted {
            SwarmEvent::Behaviour(my_behavior::Event::RelayClient(v2::client::Event::ReservationReqAccepted {
                relay_peer_id: peer_id,
                renewal,
                ..
            })) if relay_peer_id == peer_id && renewal == is_renewal => {
                reservation_req_accepted = true;
                if new_listen_addr {
                    break;
                }
            }
            SwarmEvent::NewListenAddr { address, .. } if address == client_addr => {
                new_listen_addr = true;
                if reservation_req_accepted {
                    break;
                }
            }
            SwarmEvent::Behaviour(my_behavior::Event::Ping(ping::Event{ peer, result})) => {}
            e => panic!("{:?}", e),
        }
    }
}

pub async fn wait_for_dial(client: &mut Swarm<my_behavior::MyBehavior>, remote: PeerId) -> bool {
    loop {
        match client.select_next_some().await {
            SwarmEvent::Dialing(peer_id) if peer_id == remote => {}
            SwarmEvent::ConnectionEstablished { peer_id, .. } if peer_id == remote => return true,
            SwarmEvent::OutgoingConnectionError { peer_id, .. } if peer_id == Some(remote) => {
                return false
            }
            //SwarmEvent::Behaviour(ClientEvent::Ping(_)) => {}
            SwarmEvent::Behaviour(my_behavior::Event::Ping(ping::Event{ peer, result})) => {}
            e => panic!("{:?}", e),
        }
    }
}