use crate::utils::CustomErr;
use blake2b_simd::{Hash, Params};
use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use hex;
use rand::rngs::OsRng;
use std::error::Error;
use std::{env, fmt::Display, str};

pub struct SuiAccount {
    pair: Keypair,
}

pub fn msg_hash(msg: &Vec<u8>) -> Hash {
    let mut state = Params::new().hash_length(32).to_state();
    state.update(&msg).finalize()
}

impl SuiAccount {
    pub fn from_env(name: &str) -> Result<Self, Box<dyn Error>> {
        match env::var(name) {
            Ok(val) => {
                if val == "" {
                    Err(Box::new(CustomErr::new("env value empty!")))
                } else {
                    let key_bytes = hex::decode(val).unwrap();
                    match SecretKey::from_bytes(&key_bytes) {
                        Err(err) => Err(Box::new(err)),
                        Ok(secret) => {
                            let public: PublicKey = (&secret).into();
                            Ok(SuiAccount {
                                pair: Keypair { secret, public },
                            })
                        }
                    }
                }
            }
            Err(err) => Err(Box::new(err)),
        }
    }

    pub fn new_account() -> Self {
        let mut csprng = OsRng {};
        let pair = Keypair::generate(&mut csprng);
        Self { pair }
    }
    pub fn to_address(&self) -> String {
        let mut payload: Vec<u8> = vec![0];
        payload.extend_from_slice(self.pair.public.as_bytes());
        let h = msg_hash(&payload);
        format!("0x{}", h.to_hex())
    }
}

impl Display for SuiAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "sui account: {} ", &self.to_address())
    }
}
