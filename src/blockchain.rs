// Kedjan, Validering och mining

use crate::block::Block;
use crate::transaction::Transaction;

pub struct Blockchain {
    chain: Vec<Block>,
    difficulty: u32,
    mempool: Vec<Transaction>,
}

impl Blockchain {
    fn new() {}
    fn add_block() {}
    fn is_valid(&self) -> bool {
        let mut index: usize = 1;

        loop {
            if index > self.chain.len() {
                index += 1;
            } else {
                break;
            }
        }

        true
    }
    fn get_balance(adress: &str) -> u64 {
        1
    }
    fn add_transaction() {}
}
