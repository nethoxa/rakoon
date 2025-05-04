use crate::errors::Error;
use alloy::{hex::decode, signers::k256::ecdsa::SigningKey};

pub mod constants;
pub mod errors;
pub mod types;

pub fn parse_sk(sk: &str) -> Result<SigningKey, Error> {
    let sk = SigningKey::from_slice(&decode(sk).unwrap()).unwrap();
    Ok(sk)
}
