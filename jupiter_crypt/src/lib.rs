extern crate getrandom;

fn get_random_buf() -> Result<[u8; 32], getrandom::Error> {
    let mut buf = [0u8; 32];
    getrandom::getrandom(&mut buf)?;
    Ok(buf)
}

pub fn generate_private_key(){
    let key = get_random_buf();
    println!("{:?}", hex::encode(key.unwrap()));
}