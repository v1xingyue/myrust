extern crate ed25519_dalek;

use blake2b_simd::blake2b;
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer};
use hex;
use rand::rngs::OsRng;
use std::{env, str};

// secret_key=599f6ec8dfc486cffeebb8ddab1e5c23913b16fbaf87388c68fdf5cfcd80bf4e  week1
fn get_keypair() -> Keypair {
    match env::var("secret_key") {
        Ok(val) => {
            println!("key from env : {}", val);
            if val == "" {
                let mut csprng = OsRng {};
                return Keypair::generate(&mut csprng);
            } else {
                let key_bytes = hex::decode(val).unwrap();
                println!("secret len : {}", key_bytes.len());
                let secret: SecretKey = SecretKey::from_bytes(&key_bytes).unwrap();
                let public: PublicKey = (&secret).into();

                Keypair::from(Keypair { secret, public })
            }
        }
        Err(_) => {
            let mut csprng = OsRng {};
            return Keypair::generate(&mut csprng);
        }
    }
}

pub fn ed25519_example() {
    let keypair: Keypair = get_keypair();
    let pub_bytes: &[u8; 32] = keypair.public.as_bytes();
    println!("public key : {}", hex::encode(&pub_bytes));
    let secret_bytes = keypair.secret.as_bytes();
    println!("secret key : {}", hex::encode(&secret_bytes));
    let message: &[u8] = b"hello world";
    println!("message : {}", str::from_utf8(&message).unwrap());
    let h = blake2b(message);
    println!("blake2b: {}", hex::encode(h.as_bytes()));
    let signature: Signature = keypair.sign(h.as_bytes());
    // assert!(keypair.verify(message, &signature).is_ok());
    println!("signature : {}", signature);
}
