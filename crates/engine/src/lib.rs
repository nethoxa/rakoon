pub struct Engine {
    sk: String,
    seed: String,
    no_al: bool,
    corpus: String,
    rpc: String,
    tx_count: u64,
    gas_limit: u64,
    slot_time: u64,
    airdrop_value: u64,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            sk: String::new(),
            seed: String::new(),
            no_al: false,
            corpus: String::new(),
            rpc: String::new(),
            tx_count: 0,
            gas_limit: 0,
            slot_time: 0,
            airdrop_value: 0,
        }
    }

    pub fn set_sk(&mut self, sk: String) {
        self.sk = sk;
    }

    pub fn set_seed(&mut self, seed: String) {
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

    pub fn run_airdrop(&self) -> Result<(), String> {
        todo!()
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

    pub fn run_unstuck(&self) -> Result<(), String> {
        todo!()
    }
}
