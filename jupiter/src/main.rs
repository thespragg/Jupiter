use jupiterlib::*;

fn main() {
    let mut blockchain = Blockchain::new();
    let next = Block::next(blockchain.get_latest_block(),"validator".to_string());
    blockchain.insert_block(next);

    println!("{:?}",blockchain);
    println!("{}",blockchain.is_valid());
}
