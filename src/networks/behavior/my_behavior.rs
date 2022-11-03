use libp2p::{
    gossipsub::GossipsubEvent,
    identify, ping, dcutr,
    kad::{store::MemoryStore, Kademlia, KademliaEvent},
    mdns::{Mdns, MdnsEvent},
    NetworkBehaviour,
    autonat, relay::v2,
};

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "Event")]
pub struct MyBehavior {
    pub kademlia: Kademlia<MemoryStore>,
    pub identify: identify::Behaviour,
    pub mdns: Mdns,
    pub ping: ping::Behaviour,
    pub autonat: autonat::Behaviour,
    pub relay: v2::client::Client,
    pub dcutr: dcutr::behaviour::Behaviour,

}

#[derive(Debug)]
pub enum Event {
    Kademlia(KademliaEvent),
    Identify(identify::Event),
    Mdns(MdnsEvent),
    Ping(ping::Event),
    Autonat(autonat::Event),
    RelayClient(v2::client::Event),
    Dcutr(dcutr::behaviour::Event)
}

impl From<identify::Event> for Event {
    fn from(event: identify::Event) -> Self {
        Self::Identify(event)
    }
}

impl From<KademliaEvent> for Event {
    fn from(event: KademliaEvent) -> Self {
        Self::Kademlia(event)
    }
}

impl From<MdnsEvent> for Event {
    fn from(event: MdnsEvent) -> Self {
        Self::Mdns(event)
    }
}

impl From<ping::Event> for Event {
    fn from(event: ping::Event) -> Self {
        Self::Ping(event)
    }
}

impl From<autonat::Event> for Event {
    fn from(event: autonat::Event) -> Self {
        Self::Autonat(event)
    }
}

impl From<v2::client::Event> for Event {
    fn from(event: v2::client::Event) -> Self {
        Self::RelayClient(event)
    }
}

impl From<dcutr::behaviour::Event> for Event {
    fn from(event: dcutr::behaviour::Event) -> Self {
        Self::Dcutr(event)
    }
}