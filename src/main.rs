pub mod block;
pub mod blockchain;
pub mod transaction;
pub mod utils;

use crate::block::Block;
use crate::blockchain::Blockchain;

fn main() {
    let mut chain = Blockchain::new(2);

    chain.add_block();
    chain.add_block();
    chain.add_block();
    chain.add_block();
    chain.add_block();
    chain.add_block();
    chain.add_block();
    chain.add_block();
    chain.add_block();
    chain.add_block();
    chain.add_block();
    chain.add_block();
    chain.add_block();
    chain.add_block();

    for block in chain.chain() {
        println!("Index:    {}", block.index());
        println!("Hash:     {}", block.hash());
        println!("Prev:     {}", block.previous_hash());
        println!("Nonce:    {}", block.nonce());
        println!("---");
    }

    println!("Giltig: {}", chain.is_valid());
}
