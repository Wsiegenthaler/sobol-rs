#![feature(test)]

extern crate sobol;
extern crate test;

use test::{Bencher, black_box};

use crate::sobol::params::JoeKuoD6;


/** Benchmark loading of parameter data */
#[bench] fn bench_params(b: &mut Bencher) {
    b.iter(|| black_box(JoeKuoD6::standard()));
}
