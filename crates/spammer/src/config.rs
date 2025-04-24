use crate::{errors::SpammerError, types::Backend};
use alloy::{
    eips::eip1898::BlockNumberOrTag,
    providers::{Provider, ProviderBuilder, WalletProvider},
    signers::{k256::ecdsa::SigningKey, local::PrivateKeySigner},
};
use common::{SK, STATIC_KEYS};
use mutator::Mutator;

pub struct Config {
    pub backend: Backend,

    pub n: u64,
    pub faucet: SigningKey,
    pub keys: Vec<SigningKey>,
    pub corpus: Vec<Vec<u8>>,
    pub access_list: bool,
    pub gas_limit: u64,
    pub slot_time: u64,

    pub seed: u64,
    pub mutator: Mutator,
}

impl Config {
    pub fn default(
        rpc: String,
        n: u64,
        access_list: bool,
        max_operations_per_mutation: usize,
    ) -> Result<Self, SpammerError> {
        let faucet = SigningKey::from_bytes(SK.as_bytes().into()).unwrap();
        let rpc_url = rpc.parse().unwrap();
        let backend = ProviderBuilder::new()
            .wallet::<PrivateKeySigner>(faucet.clone().into())
            .connect_http(rpc_url);

        let mut keys = Vec::new();
        for i in 0..STATIC_KEYS.len() {
            let key = SigningKey::from_bytes(STATIC_KEYS[i].as_bytes().into()).unwrap();
            keys.push(key);
        }

        Ok(Self {
            backend,
            n,
            faucet,
            keys,
            corpus: Vec::new(),
            access_list,
            gas_limit: 30_000_000,
            slot_time: 12,
            seed: 0,
            mutator: Mutator::new(max_operations_per_mutation, 0),
        })
    }

    pub async fn new(
        rpc: String,
        sk: String,
        gas_limit: u64,
        n: u64,
        corpus_file: String,
        seed: u64,
        access_list: bool,
        max_operations_per_mutation: usize,
    ) -> Result<Self, SpammerError> {
        let faucet = SigningKey::from_bytes(SK.as_bytes().into()).unwrap();
        let rpc_url = rpc.parse().unwrap();
        let backend = ProviderBuilder::new()
            .wallet::<PrivateKeySigner>(faucet.clone().into())
            .connect_http(rpc_url);

        let faucet = if sk.is_empty() {
            SigningKey::from_bytes(SK.as_bytes().into()).unwrap()
        } else {
            SigningKey::from_bytes(sk.as_bytes().into()).unwrap()
        };

        let mut keys = Vec::new();
        for i in 0..STATIC_KEYS.len() {
            let key = SigningKey::from_bytes(STATIC_KEYS[i].as_bytes().into()).unwrap();
            keys.push(key);
        }

        let gas_limit = if gas_limit == 0 {
            backend
                .get_block_by_number(BlockNumberOrTag::Latest)
                .await
                .unwrap()
                .unwrap()
                .header
                .gas_limit
        } else {
            gas_limit
        };

        let n = if n == 0 {
            Self::setup_n(&backend, keys.len() as u64, gas_limit)
                .await
                .unwrap()
        } else {
            n
        };

        let corpus = if corpus_file.is_empty() {
            Vec::new()
        } else {
            Self::read_corpus(corpus_file).unwrap()
        };

        Ok(Self {
            backend,
            n,
            faucet,
            keys,
            corpus,
            access_list,
            gas_limit,
            slot_time: 12,
            seed,
            mutator: Mutator::new(max_operations_per_mutation, seed),
        })
    }

    async fn setup_n(backend: &Backend, keys: u64, gas_limit: u64) -> Result<u64, SpammerError> {
        let header = backend
            .get_block_by_number(BlockNumberOrTag::Latest)
            .await
            .unwrap()
            .unwrap()
            .header;
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
            let file = std::fs::read(stat.unwrap().path())
                .map_err(|e| SpammerError::FailedToReadCorpus(e.to_string()))?;
            corpus.push(file);
        }
        Ok(corpus)
    }
}
