pub struct TxConf {
    pub rpc: String,
    pub nonce: u64,
    pub sender: Address,
    pub to: Address,
    pub value: U256,
    pub gas_limit: u64,
    pub gas_price: U256,
    pub chain_id: u64,
    pub code: Vec<u8>,
}

impl TxConf {
    pub fn new(rpc: String, nonce: u64, sender: Address, gas_price: U256, chain_id: u64) -> Self {
        Self { rpc, nonce, sender, to, value, gas_limit, gas_price, chain_id, code }
    }
}