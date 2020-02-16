#![feature(test)]

extern crate sobol;
extern crate test;

use test::{Bencher, black_box};

use crate::sobol::Sobol;
use crate::sobol::params::JoeKuoD6;


/** The dimensionality of the sequence to generate */
const DIMS: usize = 512;


/** Generates initialization values for an 8-bit sequence */
#[bench] fn bench_initialize_u8(b: &mut Bencher) {
    let params = JoeKuoD6::load();
    b.iter(|| black_box(Sobol::<u8>::new(DIMS, &params)));
}

/** Generates initialization values for an 16-bit sequence */
#[bench] fn bench_initialize_u16(b: &mut Bencher) {
    let params = JoeKuoD6::load();
    b.iter(|| black_box(Sobol::<u16>::new(DIMS, &params)));
}

/** Generates initialization values for an 32-bit sequence */
#[bench] fn bench_initialize_u32(b: &mut Bencher) {
    let params = JoeKuoD6::load();
    b.iter(|| black_box(Sobol::<u32>::new(DIMS, &params)));
}

/** Generates initialization values for an 64-bit sequence */
#[bench] fn bench_initialize_u64(b: &mut Bencher) {
    let params = JoeKuoD6::load();
    b.iter(|| black_box(Sobol::<u64>::new(DIMS, &params)));
}

/** Generates initialization values for an 128-bit sequence */
#[bench] fn bench_initialize_u128(b: &mut Bencher) {
    let params = JoeKuoD6::load();
    b.iter(|| black_box(Sobol::<u128>::new(DIMS, &params)));
}
