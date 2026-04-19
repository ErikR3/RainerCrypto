pub mod block;
pub mod blockchain;
pub mod transaction;
pub mod utils;
mod wallet;

use crate::blockchain::Blockchain;

fn main() {
    let mut chain = Blockchain::new(3);

    for i in 0..150 {
        chain.add_block();
    }

    for block in chain.chain() {
        println!("Index:    {}", block.index());
        println!("Hash:     {}", block.hash());
        println!("Prev:     {}", block.previous_hash());
        println!("Nonce:    {}", block.nonce());
        println!("Difficulty:    {}", block.difficulty());
        println!("---");
    }

    println!("Giltig: {}", chain.is_valid());
}
