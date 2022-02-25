use chrono::*;
use crypto_hash::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub timestamp: i64,
    pub details: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub number: u64,
    timestamp: i64,
    pub nonce: u64,
    pub transaction_list: Vec<Transaction>,
    previous_hash: String,
}

pub const PREFIX: &str = "00";

impl Block {
    pub fn genesis() -> Self {
        let transaction = Transaction {
            id: String::from("1"),
            details: String::from("Genesis block"),
            timestamp: Utc::now().timestamp(),
        };
        Block {
            number: 1,
            timestamp: Utc::now().timestamp(),
            nonce: 0,
            transaction_list: vec![transaction],
            previous_hash: String::from("0"),
        }
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn gen_hash(block: &Block) -> String {
        hex_digest(Algorithm::SHA256, block.serialize().as_bytes())
    }

    pub fn is_block_valid(hash: &str, prefix: &str) -> bool {
        hash.starts_with(prefix)
    }

    pub fn new(transactions: Vec<Transaction>, previous_block: &Block) -> Block {
        Block {
            number: previous_block.number + 1,
            timestamp: Utc::now().timestamp(),
            nonce: 0,
            transaction_list: transactions,
            previous_hash: Self::gen_hash(previous_block),
        }
    }

    pub fn mine_block(candidate: &mut Block, prefix: &str) {
        while !Self::is_block_valid(&Self::gen_hash(candidate), prefix) {
            println!("{}", candidate.nonce);
            candidate.nonce += 1
        }
    }
}
