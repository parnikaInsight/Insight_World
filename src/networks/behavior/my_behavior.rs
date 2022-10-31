use libp2p::{
    gossipsub::GossipsubEvent,
    identify, ping,
    kad::{store::MemoryStore, Kademlia, KademliaEvent},
    mdns::{Mdns, MdnsEvent},
    NetworkBehaviour,
};

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "Event")]
pub struct MyBehavior {
    pub kademlia: Kademlia<MemoryStore>,
    pub identify: identify::Behaviour,
    pub mdns: Mdns,
    pub ping: ping::Behaviour,
}

pub enum Event {
    Kademlia(KademliaEvent),
    Identify(identify::Event),
    Mdns(MdnsEvent),
    Ping(ping::Event),
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
