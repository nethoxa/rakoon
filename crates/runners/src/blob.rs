use crate::builder::Builder;
use alloy::{
    consensus::{TxEip4844, TxEip4844WithSidecar, transaction::RlpEcdsaEncodableTx},
    primitives::{Address, TxHash},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::{k256::ecdsa::SigningKey, local::PrivateKeySigner},
    transports::http::reqwest::Url,
};
use common::types::Backend;
use mutator::Mutator;
use rand::{SeedableRng, random_bool, rngs::StdRng};
use crate::logger::Logger;

pub struct BlobTransactionRunner {
    pub sk: SigningKey,
    pub seed: u64,
    pub tx_sent: u64,
    pub provider: Backend,
    pub current_tx: Vec<u8>,
    pub mutator: Mutator,
    pub crash_counter: u64,
    pub running: bool,
    pub logger: Logger,
}

impl Builder for BlobTransactionRunner {
    fn provider(&self) -> &Backend {
        &self.provider
    }

    fn is_running(&self) -> bool {
        self.running
    }
}

impl BlobTransactionRunner {
    pub fn new(rpc_url: Url, sk: SigningKey, seed: u64, max_operations_per_mutation: u64) -> Self {
        let provider = ProviderBuilder::new()
            .wallet::<PrivateKeySigner>(sk.clone().into())
            .connect_http(rpc_url);

        let mutator = Mutator::new(max_operations_per_mutation, seed);
        let logger = Logger::new("blob").unwrap();

        Self { 
            sk, 
            seed, 
            tx_sent: 0, 
            provider, 
            current_tx: vec![], 
            mutator, 
            crash_counter: 0,
            running: false,
            logger 
        }
    }

    pub async fn run(&mut self) {
        let mut random = StdRng::seed_from_u64(self.seed);
        let sender = Address::from_private_key(&self.sk);
        self.running = true;

        loop {
            // 10% chance to re-generate the transaction
            if random_bool(0.1) || self.current_tx.is_empty() {
                let (request, tx) = self.create_blob_transaction(&mut random, sender).await;
                tx.rlp_encode(&mut self.current_tx);

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

    pub async fn create_blob_transaction(
        &self,
        random: &mut StdRng,
        sender: Address,
    ) -> (TransactionRequest, TxEip4844WithSidecar) {
        // EIP-4844 transaction type
        let transaction_type = 3;

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
        let blob_versioned_hashes = self.blob_versioned_hashes(random);
        let sidecar = self.sidecar(random);

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
            authorization_list: None,
        };

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

        (request, tx_with_sidecar)
    }
}

#[tokio::test]
async fn test_blob_transaction_runner() {
    let mut rng = StdRng::seed_from_u64(1);
    let runner = BlobTransactionRunner::new(
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
    let tx = runner.create_blob_transaction(&mut rng, Address::ZERO).await;
    println!("tx: {:#?}", &tx);
}
