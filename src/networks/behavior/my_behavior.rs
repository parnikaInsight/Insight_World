use libp2p::{
    gossipsub::GossipsubEvent,
    identify,
    kad::{store::MemoryStore, Kademlia, KademliaEvent},
    mdns::{Mdns, MdnsEvent},
    request_response::{RequestResponse, RequestResponseEvent},
    NetworkBehaviour,
};

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "Event")]
pub struct MyBehavior {
    pub kademlia: Kademlia<MemoryStore>,
    pub identify: identify::Behaviour,
    pub mdns: Mdns,
}

pub enum Event {
    Kademlia(KademliaEvent),
    Identify(identify::Event),
    Mdns(MdnsEvent),
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
