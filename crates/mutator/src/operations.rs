use crate::constants::INTERESTING_8;
use rand::{Rng, rngs::StdRng, seq::SliceRandom};

/// Flip a bit in the input
pub fn flip_bit(input: &mut [u8], random: &mut StdRng) {
    let bit = random.random_range(0..8);
    let byte = random.random_range(0..input.len());
    input[byte] ^= 1 << bit;
}

/// Flip a byte in the input
pub fn flip_byte(input: &mut [u8], random: &mut StdRng) {
    let byte = random.random_range(0..input.len());
    input[byte] ^= 0xff;
}

/// Replace a random byte in the input with an interesting value
pub fn interesting(input: &mut [u8], random: &mut StdRng) {
    let idx = random.random_range(0..INTERESTING_8.len());
    input[random.random_range(0..input.len())] = INTERESTING_8[idx];
}

/// Replace a random byte in the input with an interesting value in big endian
pub fn interesting_be(input: &mut [u8], random: &mut StdRng) {
    let idx = random.random_range(0..INTERESTING_8.len());
    input[random.random_range(0..input.len())] = INTERESTING_8[idx].reverse_bits();
}

/// Add a random byte to a random byte in the input
pub fn add(input: &mut [u8], random: &mut StdRng) {
    let num = random.random_range(0..u8::MAX);
    let idx = random.random_range(0..input.len());
    input[idx] = input[idx].saturating_add(num);
}

/// Add 1 to a random byte in the input
pub fn add_one(input: &mut [u8], random: &mut StdRng) {
    let idx = random.random_range(0..input.len());
    input[idx] = input[idx].saturating_add(1);
}

/// Subtract a random byte from a random byte in the input
pub fn sub(input: &mut [u8], random: &mut StdRng) {
    let num = random.random_range(0..u8::MAX);
    let idx = random.random_range(0..input.len());
    input[idx] = input[idx].saturating_sub(num);
}

/// Subtract 1 from a random byte in the input
pub fn sub_one(input: &mut [u8], random: &mut StdRng) {
    let idx = random.random_range(0..input.len());
    input[idx] = input[idx].saturating_sub(1);
}

/// Replace a random byte in the input with a random byte
pub fn random_byte(input: &mut [u8], random: &mut StdRng) {
    let idx = random.random_range(0..input.len());
    input[idx] ^= random.random_range(0..u8::MAX);
}

/// Clone a random byte in the input to a random byte in the input
pub fn clone_byte(input: &mut [u8], random: &mut StdRng) {
    let src = random.random_range(0..input.len());
    let dst = random.random_range(0..input.len());
    input[dst] = input[src];
}

/// Swap a random byte in the input with a random byte in the input
pub fn swap_byte(input: &mut [u8], random: &mut StdRng) {
    let src = random.random_range(0..input.len());
    let dst = random.random_range(0..input.len());
    input.swap(src, dst);
}

/// Set a random byte in the input to 0
pub fn set_zero_byte(input: &mut [u8], random: &mut StdRng) {
    let idx = random.random_range(0..input.len());
    input[idx] = 0;
}

/// Set a random byte in the input to 1
pub fn set_one_byte(input: &mut [u8], random: &mut StdRng) {
    let idx = random.random_range(0..input.len());
    input[idx] = 1;
}

/// Set a random byte in the input to 0xff
pub fn set_ff_byte(input: &mut [u8], random: &mut StdRng) {
    let idx = random.random_range(0..input.len());
    input[idx] = 0xff;
}

/// Shuffle the bytes in the input
pub fn shuffle_bytes(input: &mut [u8], random: &mut StdRng) {
    let mut bytes = input.to_vec();
    bytes.shuffle(random);
    for (i, byte) in bytes.iter().enumerate() {
        input[i] = *byte;
    }
}
