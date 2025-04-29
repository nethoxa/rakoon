pub mod errors;
pub mod setters;
pub mod txs;

use alloy::{primitives::U256, rpc::types::TransactionRequest, signers::k256::ecdsa::SigningKey};
use common::Backend;
use txs::random_sender::RandomSender;

#[derive(PartialEq, Eq)]
pub enum EngineStatus {
    Stopped,
    Running,
}

pub struct Engine {
    status: EngineStatus,

    sk: SigningKey,
    keys: Vec<SigningKey>,
    corpus: Vec<Vec<u8>>,
    seed: u64,

    backend: Backend,
    current_tx: TransactionRequest,
    max_operations_per_mutation: u64,
    max_input_length: usize,
    max_access_list_length: usize,
    max_accessed_keys_length: usize,
    max_transaction_type: u8,
    max_blob_versioned_hashes_length: usize,
    max_blob_sidecar_length: usize,
    max_authorization_list_length: usize,
    max_balance_divisor: U256,

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

impl RandomSender for Engine {
    fn get_backend(&self) -> &Backend {
        &self.backend
    }

    fn current_tx(&self) -> &TransactionRequest {
        &self.current_tx
    }

    fn seed(&self) -> u64 {
        self.seed
    }

    fn max_operations_per_mutation(&self) -> u64 {
        self.max_operations_per_mutation
    }

    fn max_input_length(&self) -> usize {
        self.max_input_length
    }

    fn max_access_list_length(&self) -> usize {
        self.max_access_list_length
    }

    fn max_accessed_keys_length(&self) -> usize {
        self.max_accessed_keys_length
    }

    fn max_transaction_type(&self) -> u8 {
        self.max_transaction_type
    }

    fn max_blob_versioned_hashes_length(&self) -> usize {
        self.max_blob_versioned_hashes_length
    }

    fn max_blob_sidecar_length(&self) -> usize {
        self.max_blob_sidecar_length
    }

    fn max_authorization_list_length(&self) -> usize {
        self.max_authorization_list_length
    }

    fn max_balance_divisor(&self) -> U256 {
        self.max_balance_divisor
    }
}
