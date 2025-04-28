use rand::{Rng, SeedableRng, rngs::StdRng};
mod constants;
mod operations;

use operations::*;

#[derive(Clone)]
/// Mutator is a struct that contains the operations to mutate the input and the maximum number of
/// operations per mutation.
pub struct Mutator {
    /// The operations to mutate the input
    operations: Vec<fn(&mut [u8], &mut StdRng)>,
    /// The maximum number of operations per mutation
    max_operations_per_mutation: u64,
    /// The seed for the random number generator
    seed: u64,
}

impl Mutator {
    /// Creates a new `Mutator` with the given maximum number of operations per mutation and seed
    /// for the random number generator.
    pub fn new(max_operations_per_mutation: u64, seed: u64) -> Self {
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

    /// Mutate the input
    pub fn mutate(&self, input: &mut [u8]) {
        let mut rng = StdRng::seed_from_u64(self.seed);

        for _ in 0..rng.random_range(0..self.max_operations_per_mutation) {
            self.operations[rng.random_range(0..self.operations.len())](input, &mut rng);
        }
    }
}
