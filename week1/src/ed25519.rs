extern crate ed25519_dalek;

use crate::utils;

use blake2b_simd::{Params, State};
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};
use hex;
use rand::rngs::OsRng;
use std::{env, str};

fn get_state() -> State {
    Params::new().hash_length(32).to_state()
}

fn to_address(pk: &PublicKey) -> String {
    utils::mark_line("address test");

    let mut payload: Vec<u8> = vec![0];
    payload.extend_from_slice(pk.as_bytes());

    println!("public key add prefix : {:?}", &payload);

    let mut state = get_state();
    let h = state.update(&payload).finalize();

    println!("{}", Vec::len(&payload));

    println!(" hex hash : {}", h.to_hex());

    utils::mark_line("address test done");
    // hex::encode(pk.to_bytes())

    format!("0x{}", h.to_hex())
}

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
    println!("address : {}", to_address(&keypair.public));
    let secret_bytes = keypair.secret.as_bytes();
    println!("secret key : {}", hex::encode(&secret_bytes));
    let message: &[u8] = b"hello world";
    println!("message : {}", str::from_utf8(&message).unwrap());

    let mut intent_message: Vec<u8> = vec![3, 0, 0];
    intent_message.append(&mut message.to_vec());
    println!("intent message : {}", hex::encode(&intent_message));
    let mut state = get_state();
    let h = state.update(&intent_message).finalize();
    println!("blake2b: {}", hex::encode(h.as_bytes()));
    let signature: Signature = keypair.sign(h.as_bytes());
    // assert!(keypair.verify(message, &signature).is_ok());
    println!("signature : {}", signature);

    println!("now use public key verify...");
    match keypair.public.verify(h.as_bytes(), &signature) {
        Ok(_) => println!("{}", "done verification"),
        Err(err) => println!("{}", err.to_string()),
    }
}
