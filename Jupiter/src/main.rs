mod block;
use block::Block;
mod transaction;
use transaction::Transaction;

fn main() {
    let block = Block{
        index: 0,
        timestamp: 0,
        transactions: Vec::new(),
        hash: Vec::new(),
        prev_block_hash: Vec::new(),
        validator: String::new(),
    };

    println!("{:?}",block)
}
