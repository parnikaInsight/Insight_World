use libp2p::core::identity::Keypair;
use libp2p::core::identity::PublicKey;
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use sha2::Sha256;

use crate::networks::structs::ValueList;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]

pub struct Transaction {
    pub data: TxData,
    pub signature: Vec<u8>,
}

impl Transaction {
    pub fn new(k: Keypair, t: &PublicKey, v: u32, n: u32) -> Transaction {
        let bytes = k.public().to_protobuf_encoding();
        let d = TxData::new(bytes, t.to_protobuf_encoding(), v, n);
        let hash_d = d.hash();
        Transaction {
            data: d,
            signature: k.sign(&*hash_d.as_bytes()).unwrap(),
        }
    }

    pub fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        let serialized = serde_json::to_string(self).unwrap();
        hasher.update(serialized);
        let result: String = format!("{:X}", hasher.finalize());
        result
    }

    pub fn verify_transaction_sig(&self) -> bool {
        let pubkey: PublicKey = PublicKey::from_protobuf_encoding(&self.data.sender).unwrap();
        let v_data = self.data.hash();
        let msg: &[u8] = &*v_data.as_bytes();
        let sig: &[u8] = &self.signature;
        pubkey.verify(msg, sig)
    }

    pub fn verify_value(&self, values: &ValueList) -> bool {
        let pubkey: PublicKey = PublicKey::from_protobuf_encoding(&self.data.sender).unwrap();
        let acc = values.account(&PeerId::from_public_key(&pubkey)).unwrap();
        if acc.value <= self.data.value && acc.nonce == self.data.nonce {
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]

pub struct TxData {
    pub sender: Vec<u8>,
    pub recepient: Vec<u8>,
    pub value: u32,
    pub nonce: u32,
}

impl TxData {

    pub fn new(from: Vec<u8>, to: Vec<u8>, val: u32, once: u32) -> TxData {
        TxData {
            sender: from,
            recepient: to,
            value: val,
            nonce: once,
        }
    }

    pub fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        let serialized = serde_json::to_string(self).unwrap();
        hasher.update(serialized);
        let result: String = format!("{:X}", hasher.finalize());
        result
    }
}

/*
pub struct MemPool {
    txs: Vec<Transaction>,
}
impl MemPool {
    pub fn default() -> MemPool {
        MemPool {
            txs: Vec::<Transaction>::new(),
        }
    }
    pub fn push(&mut self, tx: Transaction) {
        self.txs.push(tx);
    }
    pub fn pop(&mut self) -> Option<Transaction> {
        self.txs.pop()
    }
    pub fn rm(&mut self, tx: &Vec<Transaction>) {
        self.txs.retain(|x| contains(x, tx));
    }
}
pub fn contains(x: &Transaction, tx: &Vec<Transaction>) -> bool {
    for t in tx {
        if x == t {
            return false;
        }
    }
    true
}
*/