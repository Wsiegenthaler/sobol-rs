extern crate sobol;

use crate::sobol::*;
use crate::sobol::params::JoeKuoD6;


/** The dimensionality of the sequence to generate */
const DIMS: usize = 10;

/** Number of points to generate */
const N: usize = 128;

/** The type of sequence values */
type ValType = f32;


/**
 * Prints first N points from an example sequence
 */
fn main() {
    println!(" [ Dimensions ] = {}", DIMS);
    println!(" [ Count      ] = {}", N);

    let params = JoeKuoD6::load();
    let sobol = Sobol::<ValType>::new(DIMS, &params);
    
    sobol
        .take(N)
        .map(|p| p.iter().map(|v| format!("{:<12}", v)).collect::<Vec<_>>().join(" "))
        .for_each(|p| println!("{}", p));

    println!("> DONE.");
}
