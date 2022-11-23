use crate::{
    blockchain::{block::Block, transactions::Transaction},
};
use libp2p::{core::identity::PublicKey, identity, PeerId};
use rocksdb::Error as DBError;
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;
use std::collections::{HashMap, VecDeque};

use super::connection::peers;

#[derive(Debug, Serialize, Deserialize)]

pub struct ProtocolHelper {
    pub friends_list: FriendsList,
    pub accounts: ValueList,
    pub block_helper: BlockHelper,
    pub node_status: NodeStatus,
    pub mem_pool: MemPool,
    pub pending_blocks: VecDeque<Block>,
    pub pontential_chains: Vec<PotentialChain>,
}

impl ProtocolHelper {

    pub fn default() -> ProtocolHelper {
        ProtocolHelper {
            friends_list: genesis_f(),
            accounts: ValueList::default(),
            block_helper: BlockHelper::default(),
            node_status: NodeStatus::Pending,
            mem_pool: MemPool::default(),
            pending_blocks: VecDeque::new(),
            pontential_chains: Vec::new(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MemPool {
    pub pool: Vec<Transaction>,
}

impl MemPool {
    pub fn default() -> MemPool {
        MemPool { pool: Vec::new() }
    }
    // remove transaction at index
    pub fn rm_tx(&mut self, index: usize) {
        self.pool.remove(index);
    }
    // returns whether mempool contains transaction and at what index

    pub fn contain(&self, tx_new: &Transaction) -> (bool, Option<usize>) {
        let mut index = 0;
        for tx in &self.pool {
            index += 1;
            if tx_new == tx {
                return (true, Some(index));
            }
        }
        return (false, None);
    }
    // pushes to tx to mempool vec
    pub fn add_tx(&mut self, tx: Transaction) {
        self.pool.push(tx);
    }
    // removes all txs from block from mempool

    pub fn valid_block(&mut self, block: &Block) {
        for tx in &block.tx {
            match self.contain(tx) {
                (true, int) => {
                    self.rm_tx(int.unwrap());
                }
                (false, _) => {}
            }
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct PotentialChain {
    pub current_i: usize,
    pub status: bool,
    pub block_help: BlockHelper,
    pub account: ValueList,
}

impl PotentialChain {

    pub fn new(h: Vec<String>) -> PotentialChain {
        PotentialChain {
            current_i: 0,
            status: false,
            block_help: BlockHelper { chain: h, work: 0 },
            account: ValueList::default(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]

pub struct BlockHelper {
    pub chain: Vec<String>,
    pub work: usize,
}

impl BlockHelper {

    pub fn default() -> BlockHelper {
        BlockHelper {
            chain: Vec::new(),
            work: 0,
        }
    }
    // add hash to end of chain
    pub fn add_to_chain(&mut self, hash: String) {
        self.chain.push(hash);
    }
    // changes last block of block helper
    pub fn work_increment(&mut self, work: usize) {
        self.work += work;
    }
}
#[derive(Debug, Serialize, Deserialize)]

pub enum NodeStatus {
    Confirmed,
    Pending,
    PendingFriend,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FriendsList {
    friends: Vec<PeerId>,
}

impl FriendsList {
    pub fn friends(&self) -> &Vec<PeerId> {
        &self.friends
    }
}

pub fn genesis_f() -> FriendsList {
    let private = identity::Keypair::from_protobuf_encoding(&peers::P1KEY).expect("Decoding Error");
    let peerid = PeerId::from(private.public());
    let mut friends: Vec<PeerId> = Vec::new();
    friends.push(peerid);
    FriendsList { friends }
}

#[derive(Debug)]

pub enum InsightError {
    DBError(DBError),
    SerdeError(SerdeError),
}

pub enum GameRequest {
    AddFriend(PeerId),
    RemoveFriend(PeerId),
    SendTransaction(PublicKey, u32),
    CreateBlock(),
}

pub enum BackendRequest {
    Start(ValueList),
    AddFriend(PeerId),
    RemoveFriend(PeerId),
    SendTransaction(PublicKey, u32),
    CreateBlock(),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValueList {
    pub list: HashMap<PeerId, AccountInfo>,
}

impl ValueList {

    pub fn default() -> ValueList {
        ValueList {
            list: HashMap::new(),
        }
    }
    pub fn account(&self, peer: &PeerId) -> Option<&AccountInfo> {
        self.list.get(peer)
    }

    pub fn add(&mut self, peer: PeerId, v: u32) {
        let acnt = self.list.entry(peer).or_insert(AccountInfo::default());
        let mut x = *acnt;
        x.value_add(v);
        self.list.insert(peer, x);
    }

    pub fn sub(&mut self, peer: PeerId, v: u32) {
        let acnt = self.list.entry(peer).or_insert(AccountInfo::default());
        let mut x = *acnt;
        x.value_sub(v);
        self.list.insert(peer, x);
    }

    pub fn nonce_increment(&mut self, peer: PeerId) {
        let acnt = self.list.entry(peer).or_insert(AccountInfo::default());
        let mut x = *acnt;
        x.nonce_inc();
        self.list.insert(peer, x);
    }

    pub fn valid_block(&mut self, block: &Block) {
        for tx in &block.tx {
            let peer_s: PeerId = PeerId::from_public_key(
                &PublicKey::from_protobuf_encoding(&tx.data.sender).unwrap(),
            );
            let peer_r: PeerId = PeerId::from_public_key(
                &PublicKey::from_protobuf_encoding(&tx.data.recepient).unwrap(),
            );
            self.add(peer_r, tx.data.value);
            self.sub(peer_s, tx.data.value);
            self.nonce_increment(peer_s)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]

pub struct AccountInfo {
    pub value: u32,
    pub nonce: u32,
}

impl AccountInfo {
    pub fn default() -> AccountInfo {
        AccountInfo { value: 0, nonce: 1 }
    }
    pub fn value_add(&mut self, v: u32) {
        self.value += v;
    }
    pub fn value_sub(&mut self, v: u32) {
        self.value -= v;
    }
    pub fn nonce_inc(&mut self) {
        self.nonce += 1;
    }
}