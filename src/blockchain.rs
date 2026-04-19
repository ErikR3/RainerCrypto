// Kedjan, Validering och mining

use crate::block::Block;
use crate::transaction::Transaction;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Blockchain {
    chain: Vec<Block>,
    difficulty: u32,
    mempool: Vec<Transaction>,
}

impl Blockchain {
    pub fn new(difficulty: u32) -> Blockchain {
        let genesis_block = Block::create_genesis_block(difficulty);

        Blockchain {
            chain: vec![genesis_block],
            difficulty: difficulty,
            mempool: vec![],
        }
    }
    pub fn add_block(&mut self) {
        let previous_hash = self.chain.last().unwrap().hash().to_string();
        let transactions = self.mempool.clone();
        let index = self.chain.len() as u64;

        let block = Block::create_block(previous_hash, self.difficulty, index, transactions);

        self.chain.push(block);
        self.mempool.clear();
    }
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            if i < self.chain.len() {
                let current = &self.chain[i];
                let previous = &self.chain[i - 1];
                if current.hash() != Block::calculate_hash(current, current.nonce()) {
                    return false;
                }
                if current.previous_hash() != previous.hash() {
                    return false;
                }
            }
        }
        true
    }

    pub fn get_balance(&self, adress: &str) -> u64 {
        let mut balance: u64 = 0;
        for block in &self.chain {
            for tx in block.transactions() {
                if tx.reciever() == adress {
                    balance += 1;
                }
                if tx.sender() == adress {
                    balance -= 1;
                }
            }
        }

        balance
    }

    fn add_transaction(&mut self, sender: String, reciever: String, amount: u64) {
        if amount > 0 {
            if sender != reciever {
                if self.get_balance(&sender) >= amount {
                    self.mempool
                        .push(Transaction::new(sender, reciever, amount))
                }
            }
        }
    }

    pub fn chain(&self) -> &[Block] {
        &self.chain
    }

    pub fn mempool(&self) -> &[Transaction] {
        &self.mempool
    }
}
