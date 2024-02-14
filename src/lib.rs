pub mod params;
mod type_support;

pub use self::type_support::*;

use std::iter::Iterator;
use std::ops::{AddAssign, BitAnd, BitXor, BitXorAssign, Mul, Shl, Shr, Sub};
use std::str::FromStr;
use std::fmt::Display;

extern crate num_traits;

use num_traits::{Bounded, One, Zero, Unsigned, PrimInt};


/**
 * A low-discrepancy Sobol sequence generator
 */
#[derive(Clone)]
pub struct Sobol<T: SobolType> {
    pub dims: usize,
    pub resolution: usize,
    dir_vals: Vec<Vec<T::IT>>,
    previous: Option<Vec<T::IT>>,
    pub count: T::IT,
    pub max_len: T::IT
}

impl<T: SobolType> Sobol<T> {

    /**
     * Constructs a new sequence
     **/
    pub fn new<P>(dims: usize, params: &dyn SobolParams<P>) -> Self
        where T::IT: LossyFrom<P> {

       Self::new_with_resolution::<P>(dims, params, None)
    }

    /**
     * Constructs a new sequence of given resolution. Resolution is the number of bits used in the
     * computation of the sequence and by default is the size of the underlying type. This
     * constructor is useful for reducing the number of cycles necessary to generate each point when the
     * length of the sequence is not expected to approach it's theoretically maximum (2^res).
     **/
    pub fn new_with_resolution<P>(dims: usize, params: &dyn SobolParams<P>, resolution: Option<usize>) -> Self
        where T::IT: LossyFrom<P> {

        let res = resolution
            .filter(|res| *res <= T::MAX_RESOLUTION)
            .unwrap_or(T::MAX_RESOLUTION);

        assert!(dims <= params.max_dims(), "Parameters for this Sobol sequence support values with a maximum of \
                                            {} dimensions but was configured for {}.", params.max_dims(), dims);
        
        let dir_values = Self::init_direction_vals::<P>(dims, res, params);
        // Transpose dir values for better cache locality
        let dir_values = (0..dir_values[0].len())
            .map(|i| dir_values.iter().map(|inner| inner[i].clone()).collect::<Vec<_>>())
            .collect();
        Sobol {
            dims,
            resolution: res,
            dir_vals: dir_values,
            count: T::IT::zero(),
            max_len: T::IT::max_value() >> (T::IT::BITS - res),
            previous: None
        } as Sobol<T>
    }

    /**
     * Initializes per-dimension direction values given sequence parameters
     */
    pub fn init_direction_vals<P>(dims: usize, resolution: usize, params: &dyn SobolParams<P>) -> Vec<Vec<T::IT>>
        where T::IT: LossyFrom<P> {

        let bits = T::IT::BITS;

        (1 ..= dims).map(|dim| match dim {
            1 => (1 ..= resolution).map(|i| T::IT::one() << (bits - i)).collect(),
            _ => {
                /* Import the parameters needed to prepare this dimension's direction vector */
                let p = params.get_dim(dim);
                let s = if resolution >= p.s() { p.s() } else { resolution };

                /* Shift initial directions */
                let mut dirs: Vec<T::IT> = vec![T::IT::zero(); resolution];
                for i in 1 ..= s {
                    let m = T::IT::lossy_from(p.m(i - 1));
                    dirs[i - 1] = m << (bits - i);
                }
    
                /* Compute remaining directions */
                for i in s + 1 ..= resolution {
                    dirs[i - 1] = dirs[i - s - 1] ^ (dirs[i - s - 1] >> s);
    
                    for k in 1 .. s {
                        let a = T::IT::lossy_from(p.coefficient(s - k - 1));
                        let dir = dirs[i - k - 1];
                        dirs[i - 1] ^= a * dir;
                    }
                }

                dirs
            }
        }).collect()
    }


    /** Returns zero-based index of the rightmost binary zero. Used for the Gray code optimization */
    #[inline] pub fn rightmost_zero(n: T::IT) -> usize {
        (n ^ T::IT::max_value()).trailing_zeros() as usize
    }
}

impl<T: SobolType> Iterator for Sobol<T> {

    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max_len {
            let next = match &self.previous {
                None => vec![T::IT::zero(); self.dims],
                Some(previous) => {
                    let a = self.count - T::IT::one();
                    let c = Self::rightmost_zero(a);
                    self.dir_vals[c as usize].iter()
                        .zip(previous)
                        .map(|(p, dir)| *p ^ *dir)
                        .collect::<Vec<T::IT>>()
                }
            };

            let next_render: Vec<T> = next.iter()
                .map(|v| T::render(*v))
                .collect();

            self.count += T::IT::one();
            self.previous = Some(next);

            Some(next_render)
        } else { None }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        for _ in 0..n {
            if self.count < self.max_len {
                let next = match &self.previous {
                    None => vec![T::IT::zero(); self.dims],
                    Some(previous) => {
                        let a = self.count - T::IT::one();
                        let c = Self::rightmost_zero(a);
                        self.dir_vals[c as usize].iter()
                            .zip(previous)
                            .map(|(p, dir)| *p ^ *dir)
                            .collect::<Vec<T::IT>>()
                    }
                };
    
                self.count += T::IT::one();
                self.previous = Some(next);
            } else { break }
        }
        self.next()
    }
}

/**
 * The main type parameter for the `Sobol` iterator. This defines the concrete `InternalType`
 * to be used internally, as well as other properties necessary for sequence generation.
 */
pub trait SobolType: Sized + Display {

    /**
     * The unsigned integer type used internally to compute sequence values.
     */
    type IT: InternalType;

    /**
     * Converts internal values to those expected by the user. This usually
     * involves casting and, for float values, scaling to the range [0,1).
     */
    fn render(val: Self::IT) -> Self;

    /**
     * The maximum number of bits this type can support. By default, this is
     * number of bits in the underlying `InternalType`, but it may be less
     * in some cases (e.g. floats are limited by the size of their significand).
     */
    const MAX_RESOLUTION: usize = Self::IT::BITS;
}

/**
 * Sequences are computed internally using unsigned types with the following capabilities
 */
pub trait InternalType:
    PrimInt +
    Unsigned +
    One +
    Zero +
    AddAssign +
    BitXorAssign +
    Sub<Output = Self> +
    Mul<Output = Self> +
    Shl<usize, Output = Self> +
    Shr<usize, Output = Self> +
    BitAnd<Output = Self> +
    BitXor<Output = Self> +
    Copy +
    PartialEq +
    PartialOrd +
    FromStr +
    Display {

    const BITS: usize;
}

/**
 * Primitive polynomial parameters and initial direction values for all sequence dimensions
 */
pub trait SobolParams<P> {

    /** Parameters for a given dimension */
    fn get_dim(&self, dim: usize) -> &dyn ParamDimension<P>;

    /** Maximum number of dimensions supported by this instance */
    fn max_dims(&self) -> usize;
}

/**
 * Primitive polynomial parameters and initial direction values for a single dimension
 */
pub trait ParamDimension<P> {

    /** The one-based index of this dimension */
    fn d(&self) -> u16;

    /** The degree of the primitive polynomial */
    fn s(&self) -> usize;
    
    /** The binary coefficient for bit `i`, the zero-based index from the right */
    fn coefficient(&self, i: usize) -> P;

    /** The initial direction value for bit `i`, the zero-based index from the right */
    fn m(&self, i: usize) -> P;
}

/**
 * A more permissive `From` trait - suitable for cases where lossy casting is
 * acceptable (i.e. truncation). This is used for casting parameter values to
 * internal values.
 */
pub trait LossyFrom<T>: Sized {
    fn lossy_from(_: T) -> Self;
}
