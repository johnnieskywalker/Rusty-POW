use crypto::digest::Digest;
use crypto::sha2::Sha256;
use lazy_static::lazy_static;
use num_bigint::BigInt;
use num_traits::FromPrimitive;
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

pub const TARGET_DIFFICULTY_HEX: &str =
    "0fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
lazy_static! {
    pub static ref TARGET_DIFFICULTY: BigInt =
        BigInt::parse_bytes(TARGET_DIFFICULTY_HEX.as_bytes(), 16).unwrap();
}
pub const MAX_TRANSACTIONS: usize = 10;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    sender: String,
    to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    id: usize,
    transactions: Vec<Transaction>,
    nonce: u64,
    hash: String,
}

pub struct Blockchain {
    mempool: Arc<Mutex<Vec<Transaction>>>,
    blocks: Arc<Mutex<Vec<Block>>>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            mempool: Arc::new(Mutex::new(Vec::new())),
            blocks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_transaction(&self, sender: String, to: String) {
        let mut mempool = self.mempool.lock().unwrap();
        mempool.push(Transaction { sender, to });
    }

    pub fn mine(&self) {
        let mut mempool = self.mempool.lock().unwrap();
        let mut blocks = self.blocks.lock().unwrap();

        let mut mined_transactions = Vec::new();
        while mined_transactions.len() < MAX_TRANSACTIONS && !mempool.is_empty() {
            mined_transactions.push(mempool.pop().unwrap());
        }

        let mut block = Block {
            id: blocks.len(),
            transactions: mined_transactions,
            nonce: 0,
            hash: String::new(),
        };

        loop {
            let json_block = serde_json::to_string(&block).unwrap();
            let mut hasher = Sha256::new();
            hasher.input_str(&json_block);
            let hash = hasher.result_str();

            let int_hash = match BigInt::parse_bytes(hash.as_bytes(), 16) {
                Some(value) => value,
                None => {
                    eprintln!("Error parsing hash: hash is not a valid hex string");
                    return;
                }
            };

            if int_hash < *TARGET_DIFFICULTY {
                block.hash = hash.to_string();
                break;
            }

            block.nonce += 1;
        }

        blocks.push(block);
    }

    pub fn blocks(&self) -> Vec<Block> {
        let blocks = self.blocks.lock().unwrap();
        blocks.clone()
    }

    pub fn mempool(&self) -> Vec<Transaction> {
        let mempool = self.mempool.lock().unwrap();
        mempool.clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn setup_blockchain_with_transactions(num_transactions: usize) -> Blockchain {
        let blockchain = Blockchain::new();
        for _ in 0..num_transactions {
            blockchain.add_transaction("bob".to_string(), "alice".to_string());
        }
        blockchain
    }

    #[test]
    fn mine_with_five_mempool_transactions() {
        let blockchain = setup_blockchain_with_transactions(5);
        blockchain.mine();

        let blocks = blockchain.blocks();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks.last().unwrap().transactions.len(), 5);
        assert_eq!(blockchain.mempool().len(), 0);
    }

    #[test]
    fn mine_with_more_than_max_transactions() {
        let blockchain = setup_blockchain_with_transactions(MAX_TRANSACTIONS + 5);
        blockchain.mine();

        let blocks = blockchain.blocks();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks.last().unwrap().transactions.len(), MAX_TRANSACTIONS);
        assert_eq!(blockchain.mempool().len(), 5);

        blockchain.mine();

        let blocks = blockchain.blocks();
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks.last().unwrap().transactions.len(), 5);
        assert_eq!(blockchain.mempool().len(), 0);
    }

    #[test]
    fn mine_with_no_transactions() {
        let blockchain = Blockchain::new();
        blockchain.mine();

        let blocks = blockchain.blocks();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks.last().unwrap().transactions.len(), 0);
    }
}
