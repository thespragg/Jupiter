use jupiterlib::*;

fn main() {
    let mut blockchain = Blockchain::new();
    let next = Block::next(blockchain.get_latest_block(),"validator".to_string());
    blockchain.insert_block(next);

    jupiter_crypt::generate_private_key();

    // println!("{:?}",blockchain);
    // println!("{}",blockchain.is_valid());
}
