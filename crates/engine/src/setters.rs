use crate::{Engine, EngineStatus, errors::EngineError};
use alloy::{
    primitives::U256,
    rpc::types::TransactionRequest,
    signers::k256::{ecdsa::SigningKey, elliptic_curve::rand_core::OsRng},
};
use common::Backend;

impl Engine {
    // Setters para los campos booleanos
    pub fn set_random_txs(&mut self, value: bool) -> &mut Self {
        self.random_txs = value;
        self
    }

    pub fn set_legacy_txs(&mut self, value: bool) -> &mut Self {
        self.legacy_txs = value;
        self
    }

    pub fn set_legacy_creation_txs(&mut self, value: bool) -> &mut Self {
        self.legacy_creation_txs = value;
        self
    }

    pub fn set_empty_al_txs(&mut self, value: bool) -> &mut Self {
        self.empty_al_txs = value;
        self
    }

    pub fn set_empty_al_creation_txs(&mut self, value: bool) -> &mut Self {
        self.empty_al_creation_txs = value;
        self
    }

    pub fn set_eip1559_txs(&mut self, value: bool) -> &mut Self {
        self.eip1559_txs = value;
        self
    }

    pub fn set_eip1559_creation_txs(&mut self, value: bool) -> &mut Self {
        self.eip1559_creation_txs = value;
        self
    }

    pub fn set_eip1559_al_txs(&mut self, value: bool) -> &mut Self {
        self.eip1559_al_txs = value;
        self
    }

    pub fn set_eip1559_al_creation_txs(&mut self, value: bool) -> &mut Self {
        self.eip1559_al_creation_txs = value;
        self
    }

    pub fn set_blob_txs(&mut self, value: bool) -> &mut Self {
        self.blob_txs = value;
        self
    }

    pub fn set_blob_creation_txs(&mut self, value: bool) -> &mut Self {
        self.blob_creation_txs = value;
        self
    }

    pub fn set_blob_al_txs(&mut self, value: bool) -> &mut Self {
        self.blob_al_txs = value;
        self
    }

    pub fn set_blob_al_creation_txs(&mut self, value: bool) -> &mut Self {
        self.blob_al_creation_txs = value;
        self
    }

    pub fn set_auth_txs(&mut self, value: bool) -> &mut Self {
        self.auth_txs = value;
        self
    }

    pub fn set_auth_creation_txs(&mut self, value: bool) -> &mut Self {
        self.auth_creation_txs = value;
        self
    }

    pub fn set_auth_al_txs(&mut self, value: bool) -> &mut Self {
        self.auth_al_txs = value;
        self
    }

    pub fn set_auth_al_creation_txs(&mut self, value: bool) -> &mut Self {
        self.auth_al_creation_txs = value;
        self
    }

    pub fn set_auth_blob_txs(&mut self, value: bool) -> &mut Self {
        self.auth_blob_txs = value;
        self
    }

    pub fn set_auth_blob_creation_txs(&mut self, value: bool) -> &mut Self {
        self.auth_blob_creation_txs = value;
        self
    }

    pub fn set_auth_blob_al_txs(&mut self, value: bool) -> &mut Self {
        self.auth_blob_al_txs = value;
        self
    }

    pub fn set_auth_blob_al_creation_txs(&mut self, value: bool) -> &mut Self {
        self.auth_blob_al_creation_txs = value;
        self
    }

    pub fn set_seed(&mut self, value: u64) -> &mut Self {
        self.seed = value;
        self
    }

    pub fn set_backend(&mut self, value: Option<Backend>) -> &mut Self {
        self.backend = value;
        self
    }

    pub fn set_current_tx(&mut self, value: TransactionRequest) -> &mut Self {
        self.current_tx = value;
        self
    }

    pub fn set_max_operations_per_mutation(&mut self, value: u64) -> &mut Self {
        self.max_operations_per_mutation = value;
        self
    }

    pub fn set_max_input_length(&mut self, value: usize) -> &mut Self {
        self.max_input_length = value;
        self
    }

    pub fn set_max_access_list_length(&mut self, value: usize) -> &mut Self {
        self.max_access_list_length = value;
        self
    }

    pub fn set_max_accessed_keys_length(&mut self, value: usize) -> &mut Self {
        self.max_accessed_keys_length = value;
        self
    }

    pub fn set_max_transaction_type(&mut self, value: u8) -> &mut Self {
        self.max_transaction_type = value;
        self
    }

    pub fn set_max_blob_versioned_hashes_length(&mut self, value: usize) -> &mut Self {
        self.max_blob_versioned_hashes_length = value;
        self
    }

    pub fn set_max_blob_sidecar_length(&mut self, value: usize) -> &mut Self {
        self.max_blob_sidecar_length = value;
        self
    }

    pub fn set_max_authorization_list_length(&mut self, value: usize) -> &mut Self {
        self.max_authorization_list_length = value;
        self
    }

    pub fn set_max_balance_divisor(&mut self, value: U256) -> &mut Self {
        self.max_balance_divisor = value;
        self
    }

    pub fn set_sk(&mut self, value: Option<SigningKey>) -> &mut Self {
        self.sk = value;
        self
    }

    pub fn set_keys(&mut self, value: Vec<SigningKey>) -> &mut Self {
        self.keys = value;
        self
    }

    pub fn set_corpus(&mut self, value: Vec<Vec<u8>>) -> &mut Self {
        self.corpus = value;
        self
    }

    pub fn set_status(&mut self, value: EngineStatus) -> &mut Self {
        self.status = value;
        self
    }

    pub fn is_running(&self) -> bool {
        self.status == EngineStatus::Running
    }

    pub fn is_stopped(&self) -> bool {
        self.status == EngineStatus::Stopped
    }

    pub fn start(&mut self) -> Result<String, EngineError> {
        if self.is_running() {
            return Err(EngineError::AlreadyRunning);
        }
        self.status = EngineStatus::Running;
        Ok("Engine started".to_string())
    }

    pub fn stop(&mut self) -> Result<String, EngineError> {
        if self.is_stopped() {
            return Err(EngineError::AlreadyStopped);
        }
        self.status = EngineStatus::Stopped;
        Ok("Engine stopped".to_string())
    }
}
