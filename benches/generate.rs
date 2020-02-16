#![feature(test)]

extern crate sobol;
extern crate test;

use test::{Bencher, black_box};

use crate::sobol::*;
use crate::sobol::params::JoeKuoD6;


/** The number of points to generate per benchmark */
const N: usize = 128;

/** The dimensionality of the sequence to generate */
const DIMS: usize = 10;

/** Generates N `f32` points */
#[bench] fn bench_generate_f32(b: &mut Bencher) {
    generate_points::<f32>(b);
}

/** Generates N `f64` points */
#[bench] fn bench_generate_f64(b: &mut Bencher) {
    generate_points::<f64>(b);
}

/** Generates N `u8` points */
#[bench] fn bench_generate_u8(b: &mut Bencher) {
    generate_points::<u8>(b);
}

/** Generates N `u16` points */
#[bench] fn bench_generate_u16(b: &mut Bencher) {
    generate_points::<u16>(b);
}

/** Generates N `u32` points */
#[bench] fn bench_generate_u32(b: &mut Bencher) {
    generate_points::<u32>(b);
}

/** Generates N `u64` points */
#[bench] fn bench_generate_u64(b: &mut Bencher) {
    generate_points::<u64>(b);
}

/** Generates N `u128` points */
#[bench] fn bench_generate_u128(b: &mut Bencher) {
    generate_points::<u128>(b);
}

/** Generates N `i8` points */
#[bench] fn bench_generate_i8(b: &mut Bencher) {
    generate_points::<i8>(b);
}

/** Generates N `i16` points */
#[bench] fn bench_generate_i16(b: &mut Bencher) {
    generate_points::<i16>(b);
}

/** Generates N `i32` points */
#[bench] fn bench_generate_i32(b: &mut Bencher) {
    generate_points::<i32>(b);
}

/** Generates N `i64` points */
#[bench] fn bench_generate_i64(b: &mut Bencher) {
    generate_points::<i64>(b);
}

/** Generates N `i128` points */
#[bench] fn bench_generate_i128(b: &mut Bencher) {
    generate_points::<i128>(b);
}


/**
 * Generates N points for a given type and consumes the results.
 */
fn generate_points<T>(b: &mut Bencher) 
    where T: SobolType, T::IT: LossyFrom<u32>, Sobol<T>: Clone {

    let sobol = Sobol::<T>::new(DIMS, &JoeKuoD6::load());
    b.iter(|| black_box(sobol.clone().take(N).collect::<Vec<_>>()));
}
