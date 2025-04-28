pub mod errors;
pub mod setters;
pub mod txs;

use crate::errors::EngineError;
use alloy::{eips::eip1559, signers::k256::ecdsa::SigningKey};

#[derive(PartialEq, Eq)]
pub enum EngineStatus {
    Stopped,
    Running,
}

pub struct Engine {
    status: EngineStatus,
    faucet: SigningKey,
    keys: Vec<SigningKey>,
    corpus: Vec<Vec<u8>>,
    seed: u64,
    gas_limit: u64,
    random_txs: bool,
    legacy_txs: bool,
    legacy_creation_txs: bool,
    empty_al_txs: bool,
    empty_al_creation_txs: bool,
    eip1559_txs: bool,
    eip1559_creation_txs: bool,
    eip1559_al_txs: bool,
    eip1559_al_creation_txs: bool,
    blob_txs: bool,
    blob_creation_txs: bool,
    blob_al_txs: bool,
    blob_al_creation_txs: bool,
    auth_txs: bool,
    auth_creation_txs: bool,
    auth_al_txs: bool,
    auth_al_creation_txs: bool,
    auth_blob_txs: bool,
    auth_blob_creation_txs: bool,
    auth_blob_al_txs: bool,
    auth_blob_al_creation_txs: bool,
}

impl Engine {}
