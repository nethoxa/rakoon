use alloy::{
    eips::eip1898::BlockNumberOrTag, primitives::{Address, Sign}, providers::{Provider, ProviderBuilder}, signers::k256::ecdsa::SigningKey
};
use mutator::Mutator;
use rand::Rng;

use crate::{errors::SpammerError, types::Backend};

use common::{STATIC_KEYS, SK};

pub struct Config {
    backend: Backend,

    n: u64,
    faucet: SigningKey,
    keys: Vec<SigningKey>,
    corpus: Vec<Vec<u8>>,
    access_list: bool,
    gas_limit: u64,
    slot_time: u64,

    seed: i64,
    mutator: Mutator,
}

impl Config {
    pub fn default(rpc: String, n: u64, access_list: bool, rng: &mut impl Rng) -> Result<Self, SpammerError> {
        let rpc_url = rpc.parse().unwrap();
        let backend = ProviderBuilder::new().connect_http(rpc_url);

        let mut keys = Vec::new();
        for i in 0..STATIC_KEYS.len() {
            let key = SigningKey::from_bytes(STATIC_KEYS[i].as_bytes().into()).unwrap();
            keys.push(key);
        }

        Ok(
            Self { 
                backend, 
                n, 
                faucet: SigningKey::from_bytes(SK.as_bytes().into()).unwrap(),
                keys,
                corpus: Vec::new(),
                access_list,
                gas_limit: 30_000_000,
                slot_time: 12,
                seed: 0,
                mutator: Mutator::new(rng),
            }
        )
    }

    async fn setup_n(backend: &Backend, keys: u64, gas_limit: u64) -> Result<u64, SpammerError> {
        let header = backend.get_block_by_number(BlockNumberOrTag::Latest).await.unwrap().unwrap().header;
        let tx_per_block = header.gas_limit / gas_limit;
        let tx_per_account = tx_per_block / keys;
        if tx_per_account == 0 {
            return Ok(1);
        }
        Ok(tx_per_account)
    }

    fn read_corpus(path: String) -> Result<Vec<Vec<u8>>, SpammerError> {
        let stats = std::fs::read_dir(path).unwrap();
        let mut corpus = Vec::new();
        for stat in stats {
            let file = std::fs::read(stat.unwrap().path()).unwrap();
            corpus.push(file);
        }
        Ok(corpus)
    }
}
