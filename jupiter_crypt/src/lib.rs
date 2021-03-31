extern crate rand;
use rand::{Rng};
use rand::os::{OsRng};

pub fn generate_private_key() {
    let mut r = OsRng::new().unwrap();
    let mut my_secure_bytes = vec![0u8; 1500];
    r.fill_bytes(&mut my_secure_bytes);
    let my_secure_int: u64 = r.gen();
    println!("First few bytes = {:?}; random int = {:?}",
        &my_secure_bytes[..5], my_secure_int);
}