use rand::{Rng, RngCore};

#[derive(Default)]
pub struct Mutator {
    seed: u64,
}

impl Mutator {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }
}
