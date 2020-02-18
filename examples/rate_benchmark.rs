extern crate sobol;

use crate::sobol::*;
use crate::sobol::params::JoeKuoD6;

use std::iter::Iterator;
use std::time::Instant;


/** The dimensionality of the sequence to generate */
const DIMS: usize = 25;

/** The number of times to run the benchmark */
const ITERATIONS: usize = 10;

/** The duration of each benchmark */
const MILLIS: u128 = 1000;

/** The type of sequence values */
type ValType = f32;


/**
 * This example benchmarks the average number of points generated
 * in a given duration.
 */
fn main() {
    println!(" [ Dimensions ] = {}d", DIMS);
    println!(" [ Duration   ] = {}ms", MILLIS);
    println!(" [ Iterations ] = {}x", ITERATIONS);

    let avg = (1..=ITERATIONS)
        .map(|i| (i, benchmark()))
        .inspect(|(i, cnt)| println!("> Iter {:<2} => {} points", i, cnt))
        .map(|(_, cnt)| cnt)
        .sum::<usize>() / ITERATIONS;
   
    println!("> AVERAGE => {} points", avg);
}

fn benchmark() -> usize {
    let params = JoeKuoD6::standard();
    let sobol = Sobol::<ValType>::new(DIMS, &params);
    let start = Instant::now();
    sobol.take_while(|_p| start.elapsed().as_millis() < MILLIS).count()
}
