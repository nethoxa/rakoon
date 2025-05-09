use crate::{builder::Builder, cache::BuilderCache, logger::Logger};
use alloy::{
    consensus::TxLegacy,
    primitives::{Address, TxHash},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::{k256::ecdsa::SigningKey, local::PrivateKeySigner},
    transports::http::reqwest::Url,
};
use alloy_rlp::Encodable;
use common::types::Backend;
use mutator::Mutator;
use rand::{SeedableRng, random_bool, rngs::StdRng};

pub struct LegacyTransactionRunner {
    pub sk: SigningKey,
    pub seed: u64,
    pub provider: Backend,
    pub current_tx: Vec<u8>,
    pub mutator: Mutator,
    pub crash_counter: u64,
    pub running: bool,
    pub logger: Logger,
    pub cache: BuilderCache,
    pub sender: Address,
}

impl Builder for LegacyTransactionRunner {
    fn provider(&self) -> &Backend {
        &self.provider
    }

    fn cache(&self) -> &BuilderCache {
        &self.cache
    }

    fn cache_mut(&mut self) -> &mut BuilderCache {
        &mut self.cache
    }
}

impl LegacyTransactionRunner {
    pub async fn new(rpc_url: Url, sk: SigningKey, seed: u64, max_operations_per_mutation: u64) -> Self {
        let provider = ProviderBuilder::new()
            .wallet::<PrivateKeySigner>(sk.clone().into())
            .connect_http(rpc_url);

        let sender = Address::from_private_key(&sk);
        let account = provider.get_account(sender).await.unwrap_or_default();
        let cache = BuilderCache {
            gas_price: provider.get_gas_price().await.unwrap_or_default(),
            max_priority_fee: provider.get_max_priority_fee_per_gas().await.unwrap_or_default(),
            max_fee_per_blob_gas: provider.get_blob_base_fee().await.unwrap_or_default(),
            balance: account.balance,
            nonce: account.nonce,
            chain_id: provider.get_chain_id().await.unwrap_or_default(),
        };

        let mutator = Mutator::new(max_operations_per_mutation, seed);
        let logger = Logger::new("legacy").unwrap();

        Self { sk, seed, current_tx: vec![], provider, mutator, crash_counter: 0, running: false, logger, cache, sender }
    }

    pub async fn run(&mut self) {
        let mut random = StdRng::seed_from_u64(self.seed);
        self.running = true;

        loop {
            // 10% chance to re-generate the transaction
            if random_bool(0.1) || self.current_tx.is_empty() {
                // 50% chance to update the cache
                // This is to try to get further in the execution by bypassing
                // common checks like gas > expected and so on
                if random_bool(0.5) {
                    self.cache.update(&self.provider, self.sender).await;
                }

                let (request, tx) = self.create_legacy_transaction(&mut random).await;
                tx.encode(&mut self.current_tx);

                if let Err(err) = self.provider.send_transaction_unsafe(request).await {
                    if self.logger.is_connection_refused_error(&err) {
                        let current_tx = self.current_tx.clone();
                        let _ = self.logger.generate_crash_report(&current_tx);

                        self.crash_counter += 1;
                        self.running = false;

                        break;
                    }
                }
            } else {
                self.mutator.mutate(&mut self.current_tx);
                if let Err(err) = self.provider.client().request::<_, TxHash>("eth_sendRawTransaction", &self.current_tx).await {
                    if self.logger.is_connection_refused_error(&err) {
                        let current_tx = self.current_tx.clone();
                        let _ = self.logger.generate_crash_report(&current_tx);

                        self.crash_counter += 1;
                        self.running = false;

                        break;
                    }
                }
            }
        }
    }

    pub async fn create_legacy_transaction(
        &self,
        random: &mut StdRng,
    ) -> (TransactionRequest, TxLegacy) {
        // Legacy transaction type
        let transaction_type = 0;

        let to = self.to(random);
        let gas_price = self.gas_price(random).await;
        let gas_limit = self.gas(random);
        let value = self.value(random).await;
        let input = self.input(random);
        let nonce = self.nonce(random).await;
        let chain_id = self.chain_id(random).await;

        let request = TransactionRequest {
            from: Some(self.sender),
            to: Some(to),
            gas_price: Some(gas_price),
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
            max_fee_per_blob_gas: None,
            gas: Some(gas_limit),
            value: Some(value),
            input: input.clone(),
            nonce: Some(nonce),
            chain_id: Some(chain_id),
            access_list: None,
            transaction_type: Some(transaction_type),
            blob_versioned_hashes: None,
            sidecar: None,
            authorization_list: None,
        };

        let tx = TxLegacy {
            to,
            value,
            chain_id: Some(chain_id),
            nonce,
            gas_price,
            gas_limit,
            input: input.into_input().unwrap(),
        };

        (request, tx)
    }
}

#[tokio::test]
async fn test_legacy_transaction_runner() {
    let mut rng = StdRng::seed_from_u64(1);
    let runner = LegacyTransactionRunner::new(
        "http://localhost:8545".parse::<Url>().unwrap(),
        SigningKey::from_slice(
            &alloy::hex::decode(
                "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
            )
            .unwrap(),
        )
        .unwrap(),
        1,
        1000
    ).await;
    let tx = runner.create_legacy_transaction(&mut rng).await;
    println!("tx: {:#?}", &tx);
}
