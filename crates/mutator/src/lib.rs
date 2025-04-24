use rand::Rng;

#[derive(Default)]
pub struct Mutator {
    rng: rand::rngs::ThreadRng,
}

impl Mutator {
    pub fn new(rng: &mut impl Rng) -> Self {
        Self { rng }
    }
}

