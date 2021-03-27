use jupiterlib::*;

fn main() {
    let mut blockchain = Blockchain::new();
    let next = Block::next(blockchain.get_latest_block(),"validator".to_string());
    blockchain.insert_block(next);

    let wallet = Wallet::new();

    println!("{}", String::from_utf8(wallet.private_key).unwrap());
    println!("{}", String::from_utf8(wallet.public_key).unwrap());
    println!("{:?}",blockchain);
    println!("{}",blockchain.is_valid());
}
