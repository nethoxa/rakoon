use alloy::{
    primitives::{Address, U256}, providers::Provider, rpc::types::TransactionRequest
};
use spammer::{Spammer, config::Config, errors::SpammerError};

pub struct Engine {
    sk: String,
    seed: u64,
    no_al: bool,
    corpus: String,
    rpc: String,
    tx_count: u64,
    gas_limit: u64,
    slot_time: u64,
    airdrop_value: u64,
    max_operations_per_mutation: usize,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            sk: String::new(),
            seed: 0,
            no_al: false,
            corpus: String::new(),
            rpc: String::new(),
            tx_count: 0,
            gas_limit: 0,
            slot_time: 0,
            airdrop_value: 0,
            max_operations_per_mutation: 0,
        }
    }

    pub fn set_sk(&mut self, sk: String) {
        self.sk = sk;
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.seed = seed;
    }

    pub fn set_no_al(&mut self, no_al: bool) {
        self.no_al = no_al;
    }

    pub fn set_corpus(&mut self, corpus: String) {
        self.corpus = corpus;
    }

    pub fn set_rpc(&mut self, rpc: String) {
        self.rpc = rpc;
    }

    pub fn set_tx_count(&mut self, tx_count: u64) {
        self.tx_count = tx_count;
    }

    pub fn set_gas_limit(&mut self, gas_limit: u64) {
        self.gas_limit = gas_limit;
    }

    pub fn set_slot_time(&mut self, slot_time: u64) {
        self.slot_time = slot_time;
    }

    pub fn set_airdrop_value(&mut self, airdrop_value: u64) {
        self.airdrop_value = airdrop_value;
    }

    pub fn set_max_operations_per_mutation(&mut self, max_operations_per_mutation: usize) {
        self.max_operations_per_mutation = max_operations_per_mutation;
    }

    async fn setup_config(&self) -> Result<Config, SpammerError> {
        if self.sk.is_empty() || self.gas_limit == 0 || self.corpus.is_empty() || self.seed == 0 {
            Config::default(
                self.rpc.clone(),
                self.tx_count,
                self.no_al,
                self.max_operations_per_mutation,
            )
        } else {
            Config::new(
                self.rpc.clone(),
                self.sk.clone(),
                self.gas_limit,
                self.tx_count,
                self.corpus.clone(),
                self.seed,
                self.no_al,
                self.max_operations_per_mutation,
            )
            .await
        }
    }

    pub async fn run_airdrop(&self) -> Result<(), String> {
        let config = self.setup_config().await.unwrap();

        for key in config.keys {
            let address = Address::from_public_key(key.verifying_key());

            let tx = TransactionRequest::default()
                .to(address)
                .value(U256::from(self.airdrop_value));

            let balance_before = config.backend.get_balance(address).await.unwrap();
            let pending: alloy::providers::PendingTransactionBuilder<alloy::network::Ethereum> = config.backend.send_transaction(tx).await.map_err(|e| SpammerError::ProviderError(e.to_string())).unwrap();
            let receipt = pending.get_receipt().await.map_err(|e| SpammerError::ProviderError(e.to_string())).unwrap();
            if receipt.status() {
                println!("Airdrop to {} sent successfully", address);
                let balance_after = config.backend.get_balance(address).await.unwrap();
                if balance_after - balance_before != U256::from(self.airdrop_value) {
                    println!("Airdropped value to {} is not correct, expected {} but got {}", address, self.airdrop_value, balance_after - balance_before);
                }
            } else {
                println!("Airdrop to {} failed", address);
            }
        }

        Ok(())
    }

    pub fn run_spam(&self) -> Result<(), String> {
        todo!()
    }

    pub fn run_blob_spam(&self) -> Result<(), String> {
        todo!()
    }

    pub fn run_7702_spam(&self) -> Result<(), String> {
        todo!()
    }

    pub fn run_create(&self) -> Result<(), String> {
        todo!()
    }
}
