use alloy::{
    primitives::{Address, U256},
    providers::Provider,
    rpc::types::TransactionRequest,
};
use spammer::{Spammer, config::Config, errors::SpammerError};
use common::PendingTransaction;
use std::{collections::HashMap, thread::sleep, time::Duration};

/// Engine is the main struct that holds the state of the engine
pub struct Engine {
    /// Private key of the account to send ETH from
    sk: String,
    /// Seed for the random number generator
    seed: u64,
    /// Whether to use access list or not
    no_al: bool,
    /// Path to the corpus file
    corpus: String,
    /// RPC endpoint to connect to
    rpc: String,
    /// Number of transactions to send
    tx_count: u64,
    /// Gas limit for each transaction
    gas_limit: u64,
    /// Slot time in seconds
    slot_time: u64,
    /// Value of the airdrop
    airdrop_value: u64,
    /// Maximum number of operations per mutation
    max_operations_per_mutation: u64,
}

impl Engine {
    /// Create a new Engine with default values
    pub fn default() -> Self {
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

    /// Set the private key of the account to send ETH from
    pub fn set_sk(&mut self, sk: String) {
        self.sk = sk;
    }

    /// Set the seed for the random number generator
    pub fn set_seed(&mut self, seed: u64) {
        self.seed = seed;
    }

    /// Set whether to use access list or not
    pub fn set_no_al(&mut self, no_al: bool) {
        self.no_al = no_al;
    }

    /// Set the path to the corpus file
    pub fn set_corpus(&mut self, corpus: String) {
        self.corpus = corpus;
    }

    /// Set the RPC endpoint to connect to
    pub fn set_rpc(&mut self, rpc: String) {
        self.rpc = rpc;
    }

    /// Set the number of transactions to send
    pub fn set_tx_count(&mut self, tx_count: u64) {
        self.tx_count = tx_count;
    }

    /// Set the gas limit for each transaction
    pub fn set_gas_limit(&mut self, gas_limit: u64) {
        self.gas_limit = gas_limit;
    }

    /// Set the slot time in seconds
    pub fn set_slot_time(&mut self, slot_time: u64) {
        self.slot_time = slot_time;
    }

    /// Set the value of the airdrop
    pub fn set_airdrop_value(&mut self, airdrop_value: u64) {
        self.airdrop_value = airdrop_value;
    }

    /// Set the maximum number of operations per mutation
    pub fn set_max_operations_per_mutation(&mut self, max_operations_per_mutation: u64) {
        self.max_operations_per_mutation = max_operations_per_mutation;
    }

    /// Setup the config for the spammer. Depending on the values of the engine, it will either create a new config or use the default one.
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

    /// Airdrop ETH to the accounts specified in the config
    pub async fn run_airdrop(&self) -> Result<(), String> {
        let config = self.setup_config().await.unwrap();
        let mut pending_txs = vec![];
        let mut balances_before: HashMap<Address, U256> = HashMap::new();

        for key in config.keys {
            let address = Address::from_public_key(key.verifying_key());

            // Automatically signs the transaction with the signing key specified when creating the `ProviderBuilder` in `setup_config`
            let tx = TransactionRequest::default()
                .to(address)
                .value(U256::from(self.airdrop_value));

            // Store the balance before the airdrop
            balances_before.insert(address, config.backend.get_balance(address).await.unwrap());

            // Send the transaction
            let pending: PendingTransaction =
                config
                    .backend
                    .send_transaction(tx)
                    .await
                    .map_err(|e| SpammerError::ProviderError(e.to_string()))
                    .unwrap();

            // Add the pending transaction to the list
            pending_txs.push(pending);
        }

        // Wait for all the transactions to be mined
        for pending in pending_txs {
            let receipt = pending
                .get_receipt()
                .await
                .map_err(|e| SpammerError::ProviderError(e.to_string()))
                .unwrap();

            let address = receipt.to.unwrap();
            let balance_before = balances_before[&address];
            
            if receipt.status() {
                println!("Airdrop to {} sent successfully", address);
                let balance_after = config.backend.get_balance(address).await.unwrap();
                
                // Sanity check to make sure the balance is correct before and after the airdrop
                if balance_after - balance_before != U256::from(self.airdrop_value) {
                    println!(
                        "Airdropped value to {} is not correct, expected {} but got {}",
                        address,
                        self.airdrop_value,
                        balance_after - balance_before
                    );
                }
            } else {
                println!("Airdrop to {} failed", address);
            }
        }

        Ok(())
    }

    pub async fn run_spam(&self) -> Result<(), String> {
        loop {
            self.run_airdrop().await.unwrap();
            let config = self.setup_config().await.unwrap();
            let keys = config.keys.clone();
            let spammer = Spammer::new(config);
            for key in keys {
                spammer.send_legacy_txs(&key).await.unwrap();
            }
            sleep(Duration::from_secs(self.slot_time));
        }
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
