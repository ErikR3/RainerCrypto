// Kedjan, Validering och mining

use crate::block::Block;
use crate::transaction::Transaction;

pub struct Blockchain {
    chain: Vec<Block>,
    difficulty: u32,
    mempool: Vec<Transaction>,
}

impl Blockchain {
    fn is_valid(&self) -> bool {}
}
