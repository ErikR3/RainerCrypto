// Blockstruktur och hashing
use crate::transaction::Transaction;
use crate::utils::hash_data;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Block {
    index: u64,
    timestamp: u64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    nonce: u64,
    hash: String,
    merkle_root: String,
    difficulty: u32,
}

impl Block {
    pub fn create_genesis_block(difficulty: u32) -> Block {
        let mut genesis = Block {
            index: 0,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("tiden gick bakåt")
                .as_secs(),
            transactions: vec![],
            previous_hash: String::from("0000000000000000"),
            nonce: 0,
            hash: String::from(""),
            merkle_root: String::from(""),
            difficulty: difficulty,
        };

        genesis.mine();
        genesis
    }

    pub fn create_block(previous_hash: String, difficulty: u32, index: u64) -> Block {
        let mut block = Block {
            index: index,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("tiden gick bakåt")
                .as_secs(),
            transactions: vec![],
            previous_hash: previous_hash,
            nonce: 0,
            hash: String::from(""),
            merkle_root: String::from(""),
            difficulty: difficulty,
        };
        block.mine();
        block
    }

    pub fn calculate_hash(&self, nonce: u64) -> String {
        let hash_value = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.previous_hash, nonce, self.merkle_root
        );
        hash_data(&hash_value)
    }

    fn mine(&mut self) {
        let target = "0".repeat(self.difficulty as usize);
        let mut hash;
        let mut nonce: u64 = 0;

        loop {
            hash = self.calculate_hash(nonce);

            if !hash.starts_with(&target) {
                nonce += 1;
            } else {
                self.nonce = nonce;
                self.hash = hash;
                break;
            }
        }
    }

    pub fn index(&self) -> u64 {
        self.index
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }

    pub fn previous_hash(&self) -> &str {
        &self.previous_hash
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }

    pub fn transactions(&self) -> &[Transaction] {
        &self.transactions
    }
}
