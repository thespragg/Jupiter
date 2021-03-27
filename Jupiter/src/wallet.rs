extern crate openssl;

use openssl::rsa::Rsa;
use openssl::symm::Cipher;

pub struct Wallet {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

impl Wallet {
    pub fn new() -> Wallet {
        let passphrase = "test";

        let rsa = Rsa::generate(1024).unwrap();
        let private_key: Vec<u8> = rsa
            .private_key_to_pem_passphrase(Cipher::aes_128_ctr(), passphrase.as_bytes())
            .unwrap();
        let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();

        Wallet {
            private_key: private_key,
            public_key: public_key,
        }
    }
}
