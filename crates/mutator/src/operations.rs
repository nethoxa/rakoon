use rand::{Rng, rngs::StdRng, seq::SliceRandom};

use crate::constants::INTERESTING_8;

pub fn flip_bit(input: &mut [u8], random: &mut StdRng) {
    let bit = random.random_range(0..8);
    let byte = random.random_range(0..input.len());
    input[byte] ^= 1 << bit;
}

pub fn flip_byte(input: &mut [u8], random: &mut StdRng) {
    let byte = random.random_range(0..input.len());
    input[byte] ^= 0xff;
}

pub fn interesting(input: &mut [u8], random: &mut StdRng) {
    let idx = random.random_range(0..INTERESTING_8.len());
    input[random.random_range(0..input.len())] = INTERESTING_8[idx];
}

pub fn interesting_be(input: &mut [u8], random: &mut StdRng) {
    let idx = random.random_range(0..INTERESTING_8.len());
    input[random.random_range(0..input.len())] = INTERESTING_8[idx].reverse_bits();
}

pub fn add(input: &mut [u8], random: &mut StdRng) {
    let num = random.random_range(0..u8::MAX);
    let idx = random.random_range(0..input.len());
    input[idx] = input[idx].saturating_add(num);
}

pub fn add_one(input: &mut [u8], random: &mut StdRng) {
    let idx = random.random_range(0..input.len());
    input[idx] = input[idx].saturating_add(1);
}

pub fn sub(input: &mut [u8], random: &mut StdRng) {
    let num = random.random_range(0..u8::MAX);
    let idx = random.random_range(0..input.len());
    input[idx] = input[idx].saturating_sub(num);
}

pub fn sub_one(input: &mut [u8], random: &mut StdRng) {
    let idx = random.random_range(0..input.len());
    input[idx] = input[idx].saturating_sub(1);
}

pub fn random_byte(input: &mut [u8], random: &mut StdRng) {
    let idx = random.random_range(0..input.len());
    input[idx] ^= random.random_range(0..u8::MAX);
}

pub fn clone_byte(input: &mut [u8], random: &mut StdRng) {
    let src = random.random_range(0..input.len());
    let dst = random.random_range(0..input.len());
    input[dst] = input[src];
}

pub fn swap_byte(input: &mut [u8], random: &mut StdRng) {
    let src = random.random_range(0..input.len());
    let dst = random.random_range(0..input.len());
    input.swap(src, dst);
}

pub fn set_zero_byte(input: &mut [u8], random: &mut StdRng) {
    let idx = random.random_range(0..input.len());
    input[idx] = 0;
}

pub fn set_one_byte(input: &mut [u8], random: &mut StdRng) {
    let idx = random.random_range(0..input.len());
    input[idx] = 1;
}

pub fn set_ff_byte(input: &mut [u8], random: &mut StdRng) {
    let idx = random.random_range(0..input.len());
    input[idx] = 0xff;
}

pub fn shuffle_bytes(input: &mut [u8], random: &mut StdRng) {
    let mut bytes = input.to_vec();
    bytes.shuffle(random);
    for (i, byte) in bytes.iter().enumerate() {
        input[i] = *byte;
    }
}
