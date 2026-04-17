// Blockstruktur och hashing
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
    fn calculate_hash(block: &Block) -> u8 {
        let hash_value = format!(
            "{}{}{}{}{}",
            block.index.to_string(),
            block.timestamp.to_string(),
            block.previous_hash,
            block.nonce.to_string(),
            block.merkle_root
        );
        let hash256 = Sha256::digest(&hash_value.as_bytes());
        let hex_string = format!("{:x}", hash256);
    }
}
