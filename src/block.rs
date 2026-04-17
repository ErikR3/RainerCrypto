// Blockstruktur och hashing
use crate::transaction::Transaction;
use hex_literal::hex;
use sha2::{Digest, Sha256};
use time::Date;

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
    fn calculate_hash(&self, nonce: u64) -> String {
        let hash_value = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.previous_hash, nonce, self.merkle_root
        );
        let hash256 = Sha256::digest(hash_value.as_bytes());
        hex::encode(hash256)
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
}
