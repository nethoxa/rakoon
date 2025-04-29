use crate::{Engine, EngineStatus, errors::EngineError};
use alloy::{
    primitives::U256,
    rpc::types::TransactionRequest,
    signers::k256::{ecdsa::SigningKey, elliptic_curve::rand_core::OsRng},
};

impl Engine {
    pub fn new(
        rpc: String,
        max_operation_per_mutation: u64,
        max_input_length: usize,
        max_access_list_length: usize,
        max_accessed_keys_length: usize,
        max_transaction_type: u8,
        max_blob_versioned_hashes_length: usize,
        max_blob_sidecar_length: usize,
        max_authorization_list_length: usize,
        max_balance_divisor: U256,
    ) -> Self {
        Self {
            status: EngineStatus::Stopped,

            sk: SigningKey::random(&mut OsRng),
            keys: Vec::new(),
            corpus: Vec::new(),
            seed: 0,

            backend: todo!(),
            current_tx: TransactionRequest::default(),
            max_operations_per_mutation: todo!(),
            max_input_length: todo!(),
            max_access_list_length: todo!(),
            max_accessed_keys_length: todo!(),
            max_transaction_type: todo!(),
            max_blob_versioned_hashes_length: todo!(),
            max_blob_sidecar_length: todo!(),
            max_authorization_list_length: todo!(),
            max_balance_divisor: todo!(),

            random_txs: false,
            legacy_txs: false,
            legacy_creation_txs: false,
            empty_al_txs: false,
            empty_al_creation_txs: false,
            eip1559_txs: false,
            eip1559_creation_txs: false,
            eip1559_al_txs: false,
            eip1559_al_creation_txs: false,
            blob_txs: false,
            blob_creation_txs: false,
            blob_al_txs: false,
            blob_al_creation_txs: false,
            auth_txs: false,
            auth_creation_txs: false,
            auth_al_txs: false,
            auth_al_creation_txs: false,
            auth_blob_txs: false,
            auth_blob_creation_txs: false,
            auth_blob_al_txs: false,
            auth_blob_al_creation_txs: false,
        }
    }

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
