use rand::{Rng, SeedableRng, rngs::StdRng};
mod constants;
mod operations;

use operations::*;

#[derive(Clone)]
pub struct Mutator {
    operations: Vec<fn(&mut [u8], &mut StdRng)>,
    max_operations_per_mutation: usize,
    seed: u64,
}

impl Mutator {
    pub fn new(max_operations_per_mutation: usize, seed: u64) -> Self {
        Self {
            operations: vec![
                flip_bit,
                flip_byte,
                interesting,
                interesting_be,
                add,
                add_one,
                sub,
                sub_one,
                random_byte,
                clone_byte,
                swap_byte,
                set_zero_byte,
                set_one_byte,
                set_ff_byte,
                shuffle_bytes,
            ],
            max_operations_per_mutation,
            seed,
        }
    }
    pub fn mutate(&self, input: &mut [u8]) {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let mut operations = vec![];

        for _ in 0..rng.random_range(0..self.max_operations_per_mutation) {
            operations.push(self.operations[rng.random_range(0..self.operations.len())]);
        }

        for operation in operations {
            operation(input, &mut rng);
        }
    }
}
