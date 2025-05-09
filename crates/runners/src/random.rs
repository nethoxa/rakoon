use crate::{builder::Builder, logger::Logger};
use alloy::{
    consensus::{
        TxEip1559, TxEip2930, TxEip4844, TxEip4844WithSidecar, TxEip7702, TxLegacy,
        transaction::RlpEcdsaEncodableTx,
    },
    primitives::{Address, TxHash},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::{k256::ecdsa::SigningKey, local::PrivateKeySigner},
    transports::http::reqwest::Url,
};
use alloy_rlp::Encodable;
use common::{constants::MAX_TRANSACTION_LENGTH, types::Backend};
use mutator::Mutator;
use rand::{Rng, SeedableRng, random_bool, rngs::StdRng};
pub struct RandomTransactionRunner {
    pub sk: SigningKey,
    pub seed: u64,
    pub provider: Backend,
    pub current_tx: Vec<u8>,
    pub mutator: Mutator,
    pub running: bool,
    pub crash_counter: u64,
    pub logger: Logger,
}

impl Builder for RandomTransactionRunner {
    fn provider(&self) -> &Backend {
        &self.provider
    }
}

impl RandomTransactionRunner {
    pub fn new(rpc_url: Url, sk: SigningKey, seed: u64, max_operations_per_mutation: u64) -> Self {
        let provider = ProviderBuilder::new()
            .wallet::<PrivateKeySigner>(sk.clone().into())
            .connect_http(rpc_url);

        let mutator = Mutator::new(max_operations_per_mutation, seed);
        let logger = Logger::new("random").unwrap();

        Self { sk, seed, current_tx: vec![], provider, mutator, running: false, crash_counter: 0, logger }
    }

    pub async fn run(&mut self) {
        let mut random = StdRng::seed_from_u64(self.seed);
        let sender = Address::from_private_key(&self.sk);
        self.running = true;

        loop {
            // 10% chance to re-generate the transaction
            if random_bool(0.1) || self.current_tx.is_empty() {
                let (request, tx) = self.create_random_transaction(&mut random, sender).await;
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

    pub async fn create_random_transaction(
        &self,
        random: &mut StdRng,
        sender: Address,
    ) -> (TransactionRequest, Vec<u8>) {
        let to = self.to(random);
        let gas_price = self.gas_price(random).await;
        let max_fee_per_gas = self.max_fee_per_gas(random);
        let max_priority_fee_per_gas = self.max_priority_fee_per_gas(random).await;
        let max_fee_per_blob_gas = self.max_fee_per_blob_gas(random).await;
        let gas = self.gas(random);
        let value = self.value(random, sender).await;
        let input = self.input(random);
        let nonce = self.nonce(random, sender).await;
        let chain_id = self.chain_id(random).await;
        let access_list = self.access_list(random);
        let transaction_type = self.transaction_type(random);
        let blob_versioned_hashes = self.blob_versioned_hashes(random);
        let sidecar = self.sidecar(random);
        let authorization_list = self.authorization_list(random);

        let request = TransactionRequest {
            from: Some(sender),
            to: Some(to),
            gas_price: Some(gas_price),
            max_fee_per_gas: Some(max_fee_per_gas),
            max_priority_fee_per_gas: Some(max_priority_fee_per_gas),
            max_fee_per_blob_gas: Some(max_fee_per_blob_gas),
            gas: Some(gas),
            value: Some(value),
            input: input.clone(),
            nonce: Some(nonce),
            chain_id: Some(chain_id),
            access_list: Some(access_list.clone()),
            transaction_type: Some(transaction_type),
            blob_versioned_hashes: Some(blob_versioned_hashes.clone()),
            sidecar: Some(sidecar.clone()),
            authorization_list: Some(authorization_list.clone()),
        };

        let mut encoded = vec![];
        match transaction_type {
            0 => {
                let tx = TxLegacy {
                    to,
                    value,
                    chain_id: Some(chain_id),
                    nonce,
                    gas_price,
                    gas_limit: gas,
                    input: input.into_input().unwrap(),
                };

                tx.encode(&mut encoded);
            }
            1 => {
                let tx = TxEip2930 {
                    to,
                    gas_price,
                    gas_limit: gas,
                    value,
                    chain_id,
                    nonce,
                    access_list,
                    input: input.into_input().unwrap(),
                };

                tx.encode(&mut encoded);
            }
            2 => {
                let tx = TxEip1559 {
                    to,
                    gas_limit: gas,
                    value,
                    chain_id,
                    nonce,
                    access_list,
                    input: input.into_input().unwrap(),
                    max_fee_per_gas,
                    max_priority_fee_per_gas,
                };

                tx.encode(&mut encoded);
            }
            3 => {
                let tx = TxEip4844 {
                    to: to.into_to().unwrap_or_else(|| Address::ZERO),
                    chain_id,
                    nonce,
                    max_fee_per_gas,
                    max_priority_fee_per_gas,
                    value,
                    access_list,
                    blob_versioned_hashes,
                    max_fee_per_blob_gas,
                    input: input.into_input().unwrap(),
                    gas_limit: gas,
                };

                let tx_with_sidecar = TxEip4844WithSidecar { tx, sidecar };

                tx_with_sidecar.rlp_encode(&mut encoded);
            }
            4 => {
                let tx = TxEip7702 {
                    to: to.into_to().unwrap_or_else(|| Address::ZERO),
                    gas_limit: gas,
                    value,
                    chain_id,
                    nonce,
                    max_fee_per_gas,
                    max_priority_fee_per_gas,
                    access_list,
                    authorization_list,
                    input: input.into_input().unwrap(),
                };

                tx.encode(&mut encoded);
            }
            _ => {
                // Fill with random bytes for any other transaction type
                let length = random.random_range(0..=MAX_TRANSACTION_LENGTH);
                let random_bytes = (0..length)
                    .map(|_| random.random_range(0..=u8::MAX) as u8)
                    .collect::<Vec<u8>>();

                encoded.extend_from_slice(&random_bytes);
            }
        };

        (request, encoded)
    }
}

#[tokio::test]
async fn test_random_transaction_runner() {
    let mut rng = StdRng::seed_from_u64(1);
    let runner = RandomTransactionRunner::new(
        "http://localhost:8545".parse::<Url>().unwrap(),
        SigningKey::from_slice(
            &alloy::hex::decode(
                "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
            )
            .unwrap(),
        )
        .unwrap(),
        1,
        1000,
    );
    let tx = runner.create_random_transaction(&mut rng, Address::ZERO).await;
    println!("tx: {:#?}", &tx);
}
