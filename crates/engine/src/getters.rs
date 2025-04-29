use crate::{Backend, Engine, EngineStatus};
use alloy::{primitives::U256, rpc::types::TransactionRequest, signers::k256::ecdsa::SigningKey};

impl Engine {
    // Status getters
    pub fn get_status(&self) -> &EngineStatus {
        &self.status
    }

    // Configuration getters
    pub fn get_random_txs(&self) -> bool {
        self.random_txs
    }

    pub fn get_legacy_txs(&self) -> bool {
        self.legacy_txs
    }

    pub fn get_legacy_creation_txs(&self) -> bool {
        self.legacy_creation_txs
    }

    pub fn get_empty_al_txs(&self) -> bool {
        self.empty_al_txs
    }

    pub fn get_empty_al_creation_txs(&self) -> bool {
        self.empty_al_creation_txs
    }

    pub fn get_eip1559_txs(&self) -> bool {
        self.eip1559_txs
    }

    pub fn get_eip1559_creation_txs(&self) -> bool {
        self.eip1559_creation_txs
    }

    pub fn get_eip1559_al_txs(&self) -> bool {
        self.eip1559_al_txs
    }

    pub fn get_eip1559_al_creation_txs(&self) -> bool {
        self.eip1559_al_creation_txs
    }

    pub fn get_blob_txs(&self) -> bool {
        self.blob_txs
    }

    pub fn get_blob_creation_txs(&self) -> bool {
        self.blob_creation_txs
    }

    pub fn get_blob_al_txs(&self) -> bool {
        self.blob_al_txs
    }

    pub fn get_blob_al_creation_txs(&self) -> bool {
        self.blob_al_creation_txs
    }

    pub fn get_auth_txs(&self) -> bool {
        self.auth_txs
    }

    pub fn get_auth_creation_txs(&self) -> bool {
        self.auth_creation_txs
    }

    pub fn get_auth_al_txs(&self) -> bool {
        self.auth_al_txs
    }

    pub fn get_auth_al_creation_txs(&self) -> bool {
        self.auth_al_creation_txs
    }

    pub fn get_auth_blob_txs(&self) -> bool {
        self.auth_blob_txs
    }

    pub fn get_auth_blob_creation_txs(&self) -> bool {
        self.auth_blob_creation_txs
    }

    pub fn get_auth_blob_al_txs(&self) -> bool {
        self.auth_blob_al_txs
    }

    pub fn get_auth_blob_al_creation_txs(&self) -> bool {
        self.auth_blob_al_creation_txs
    }

    // Transaction and key getters
    pub fn get_current_tx(&self) -> &TransactionRequest {
        &self.current_tx
    }

    pub fn get_sk(&self) -> &Option<SigningKey> {
        &self.sk
    }

    pub fn get_keys(&self) -> &Vec<SigningKey> {
        &self.keys
    }

    pub fn get_corpus(&self) -> &Vec<Vec<u8>> {
        &self.corpus
    }

    pub fn get_seed(&self) -> u64 {
        self.seed
    }

    pub fn get_backend(&self) -> &Option<Backend> {
        &self.backend
    }

    // Limit getters
    pub fn get_max_operations_per_mutation(&self) -> u64 {
        self.max_operations_per_mutation
    }

    pub fn get_max_input_length(&self) -> usize {
        self.max_input_length
    }

    pub fn get_max_access_list_length(&self) -> usize {
        self.max_access_list_length
    }

    pub fn get_max_accessed_keys_length(&self) -> usize {
        self.max_accessed_keys_length
    }

    pub fn get_max_transaction_type(&self) -> u8 {
        self.max_transaction_type
    }

    pub fn get_max_blob_versioned_hashes_length(&self) -> usize {
        self.max_blob_versioned_hashes_length
    }

    pub fn get_max_blob_sidecar_length(&self) -> usize {
        self.max_blob_sidecar_length
    }

    pub fn get_max_authorization_list_length(&self) -> usize {
        self.max_authorization_list_length
    }

    pub fn get_max_balance_divisor(&self) -> &U256 {
        &self.max_balance_divisor
    }

    // Format all engine settings as a string
    pub fn format_settings(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!("Engine Status: {:?}\n", self.status));
        output.push_str(&format!("Seed: {}\n", self.seed));
        output.push_str("\nTransaction Types:\n");
        output.push_str(&format!("  Random Transactions: {}\n", self.random_txs));
        output.push_str(&format!("  Legacy Transactions: {}\n", self.legacy_txs));
        output.push_str(&format!("  Legacy Creation Transactions: {}\n", self.legacy_creation_txs));
        output.push_str(&format!("  Empty Access List Transactions: {}\n", self.empty_al_txs));
        output.push_str(&format!(
            "  Empty Access List Creation Transactions: {}\n",
            self.empty_al_creation_txs
        ));
        output.push_str(&format!("  EIP1559 Transactions: {}\n", self.eip1559_txs));
        output
            .push_str(&format!("  EIP1559 Creation Transactions: {}\n", self.eip1559_creation_txs));
        output.push_str(&format!("  EIP1559 Access List Transactions: {}\n", self.eip1559_al_txs));
        output.push_str(&format!(
            "  EIP1559 Access List Creation Transactions: {}\n",
            self.eip1559_al_creation_txs
        ));
        output.push_str(&format!("  Blob Transactions: {}\n", self.blob_txs));
        output.push_str(&format!("  Blob Creation Transactions: {}\n", self.blob_creation_txs));
        output.push_str(&format!("  Blob Access List Transactions: {}\n", self.blob_al_txs));
        output.push_str(&format!(
            "  Blob Access List Creation Transactions: {}\n",
            self.blob_al_creation_txs
        ));
        output.push_str(&format!("  Auth Transactions: {}\n", self.auth_txs));
        output.push_str(&format!("  Auth Creation Transactions: {}\n", self.auth_creation_txs));
        output.push_str(&format!("  Auth Access List Transactions: {}\n", self.auth_al_txs));
        output.push_str(&format!(
            "  Auth Access List Creation Transactions: {}\n",
            self.auth_al_creation_txs
        ));
        output.push_str(&format!("  Auth Blob Transactions: {}\n", self.auth_blob_txs));
        output.push_str(&format!(
            "  Auth Blob Creation Transactions: {}\n",
            self.auth_blob_creation_txs
        ));
        output.push_str(&format!(
            "  Auth Blob Access List Transactions: {}\n",
            self.auth_blob_al_txs
        ));
        output.push_str(&format!(
            "  Auth Blob Access List Creation Transactions: {}\n",
            self.auth_blob_al_creation_txs
        ));

        output.push_str("\nLimits:\n");
        output.push_str(&format!(
            "  Max Operations Per Mutation: {}\n",
            self.max_operations_per_mutation
        ));
        output.push_str(&format!("  Max Input Length: {}\n", self.max_input_length));
        output.push_str(&format!("  Max Access List Length: {}\n", self.max_access_list_length));
        output
            .push_str(&format!("  Max Accessed Keys Length: {}\n", self.max_accessed_keys_length));
        output.push_str(&format!("  Max Transaction Type: {}\n", self.max_transaction_type));
        output.push_str(&format!(
            "  Max Blob Versioned Hashes Length: {}\n",
            self.max_blob_versioned_hashes_length
        ));
        output.push_str(&format!("  Max Blob Sidecar Length: {}\n", self.max_blob_sidecar_length));
        output.push_str(&format!(
            "  Max Authorization List Length: {}\n",
            self.max_authorization_list_length
        ));
        output.push_str(&format!("  Max Balance Divisor: {}\n", self.max_balance_divisor));

        output.push_str("\nKeys and Corpus:\n");
        output.push_str(&format!("  Has Signing Key: {}\n", self.sk.is_some()));
        output.push_str(&format!("  Number of Keys: {}\n", self.keys.len()));
        output.push_str(&format!("  Corpus Size: {}\n", self.corpus.len()));
        output.push_str(&format!(
            "  Backend: {}\n",
            if self.backend.is_some() { "Set" } else { "None" }
        ));

        output
    }
}
