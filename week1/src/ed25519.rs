// [dependencies]
// serde_json = "1.0"
// serde = { version = "1.0", features = ["derive"] }
// ansi_term = "0.12.1"
// rand = "0.7"
// ed25519-dalek = "1"
// rand_core = { version = "0.5" }
// hex = "0.4.3"
// blake2b_simd = "1.0.1"

extern crate ed25519_dalek;

use blake2b_simd::{Hash, Params};
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};
use hex;
use rand::rngs::OsRng;
use std::{env, str};

use crate::utils;

pub fn msg_hash(msg: &Vec<u8>) -> Hash {
    let mut state = Params::new().hash_length(32).to_state();
    state.update(&msg).finalize()
}

pub fn to_address(pk: &PublicKey) -> String {
    let mut payload: Vec<u8> = vec![0];
    payload.extend_from_slice(pk.as_bytes());

    // println!("public key add prefix : {:?}", &payload);
    let h = msg_hash(&payload);
    println!("{}", Vec::len(&payload));
    format!("0x{}", h.to_hex())
}

// secret_key=599f6ec8dfc486cffeebb8ddab1e5c23913b16fbaf87388c68fdf5cfcd80bf4e  week1
pub fn get_keypair() -> Keypair {
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

pub fn sui_signer_validator(msg: &[u8], sui_signature: String) {
    utils::mark_line("signature validator ");
    let signature_bytes: Vec<u8> = base64::decode(sui_signature).unwrap();
    if signature_bytes[0] != 0 {
        println!("not supported yet...");
    } else {
        let pub_bytes = &signature_bytes[signature_bytes.len() - 32..];
        let pub_key = PublicKey::from_bytes(pub_bytes).unwrap();

        let mut intent_message: Vec<u8> = vec![3, 0, 0];
        intent_message.append(&mut msg.to_vec());
        println!("intent message : {}", hex::encode(&intent_message));
        let h: Hash = msg_hash(&intent_message);
        println!("blake2b: {}", hex::encode(h.as_bytes()));

        let msg_signature =
            Signature::from_bytes(&signature_bytes[1..signature_bytes.len() - 32]).unwrap();
        match pub_key.verify(h.as_bytes(), &msg_signature) {
            Ok(_) => {
                println!("signature right");
            }
            Err(err) => {
                println!("{}", err.to_string());
            }
        };
    }
}

pub fn sui_sign_data_example() {
    const MESSAGE: &[u8] = b"hello world";
    let keypair: Keypair = get_keypair();
    let pub_bytes: &[u8; 32] = keypair.public.as_bytes();
    println!("public key : {}", hex::encode(&pub_bytes));
    println!("address : {}", to_address(&keypair.public));
    let secret_bytes = keypair.secret.as_bytes();
    println!("secret key : {}", hex::encode(&secret_bytes));

    println!("message : {}", str::from_utf8(&MESSAGE).unwrap());

    let mut intent_message: Vec<u8> = vec![3, 0, 0];
    intent_message.append(&mut MESSAGE.to_vec());
    println!("intent message : {}", hex::encode(&intent_message));
    let h = msg_hash(&intent_message);
    println!("blake2b: {}", hex::encode(h.as_bytes()));
    let signature: Signature = keypair.sign(h.as_bytes());
    // assert!(keypair.verify(message, &signature).is_ok());
    println!("message signature : {}", signature);
    // println!("message signature raw : {:?}", signature.to_bytes());

    let mut wrapper_signature: Vec<u8> = vec![0x00];
    wrapper_signature.append(&mut signature.to_bytes().to_vec());
    wrapper_signature.append(&mut pub_bytes.to_vec());

    // println!("wrapper signature: {:?}", wrapper_signature);
    let encoded = base64::encode(&wrapper_signature);
    println!("sui wallet signature : {}", encoded);

    sui_signer_validator(MESSAGE, encoded);

    println!("now use public key verify...");
    match keypair.public.verify(h.as_bytes(), &signature) {
        Ok(_) => println!("{}", "done verification"),
        Err(err) => println!("{}", err.to_string()),
    }
}
