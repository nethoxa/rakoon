pub mod config;
pub mod errors;

use std::{thread::sleep, time::Duration};

use alloy::{
    consensus::{SignableTransaction, TxLegacy},
    primitives::{Address, TxKind, U256},
    providers::Provider,
    signers::{Signature, k256::ecdsa::SigningKey},
};
use rand::{Rng, RngCore};

use crate::{config::Config, errors::SpammerError};

pub struct Spammer {
    config: Config,
}

impl Spammer {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn run(&self) -> Result<(), SpammerError> {
        todo!()
    }

    pub async fn send_legacy_txs(&self, key: &SigningKey) -> Result<(), SpammerError> {
        let pk = key.verifying_key();
        let from = Address::from_public_key(pk);
        let chain_id = self
            .config
            .backend
            .get_chain_id()
            .await
            .map_err(|e| SpammerError::ProviderError(e.to_string()))?;
        let gas_price = self
            .config
            .backend
            .get_gas_price()
            .await
            .map_err(|e| SpammerError::ProviderError(e.to_string()))?;
        let code = {
            // [nethoxa] TODO create valid code
            let mut rng = rand::rng();
            let length = rng.random_range(0..=128);
            let mut bytes = vec![0u8; length];
            rng.fill_bytes(&mut bytes);
            bytes
        };

        let to = {
            let mut rng = rand::rng();
            let mut bytes = vec![0u8; 20];
            rng.fill_bytes(&mut bytes);
            Address::from_slice(&bytes)
        };

        for i in 0..self.config.n {
            let account = self
                .config
                .backend
                .get_account(from)
                .await
                .map_err(|e| SpammerError::ProviderError(e.to_string()))?;
            let tx = TxLegacy {
                chain_id: Some(chain_id),
                nonce: account.nonce,
                gas_price,
                gas_limit: 1000000,
                to: TxKind::Call(to),
                value: U256::from(i % 2),
                input: code.clone().into(),
            };

            let tx_encoded = tx.encoded_for_signing();
            let (signature, recovery_id) = key
                .sign_recoverable(tx_encoded.as_slice())
                .map_err(|e| SpammerError::SigningError(e.to_string()))?;
            let tx_signed = tx.into_signed(Signature::from_bytes_and_parity(
                signature.to_bytes().as_slice(),
                recovery_id.to_byte() == 0,
            )); // [nethoxa] TODO: check if this is correct

            let mut buffer = vec![];
            tx_signed.rlp_encode(&mut buffer);

            let pending = self
                .config
                .backend
                .send_raw_transaction(buffer.as_slice())
                .await
                .map_err(|e| SpammerError::ProviderError(e.to_string()))?;
            let receipt = pending
                .get_receipt()
                .await
                .map_err(|e| SpammerError::ProviderError(e.to_string()))?;
            if receipt.status() {
                println!("Transaction sent successfully");
            } else {
                println!("Transaction failed");
            }

            sleep(Duration::from_millis(10));
        }
        Ok(())
    }

    fn send_blob_tx(&self, key: &SigningKey) -> Result<(), SpammerError> {
        todo!()
    }

    fn send_7702_tx(&self, key: &SigningKey) -> Result<(), SpammerError> {
        todo!()
    }

    fn send_access_list_tx(&self, key: &SigningKey) -> Result<(), SpammerError> {
        todo!()
    }

    fn send_1559_tx(&self, key: &SigningKey) -> Result<(), SpammerError> {
        todo!()
    }
}
