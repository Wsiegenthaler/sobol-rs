extern crate sobol;
extern crate libflate;
#[macro_use] extern crate lazy_static;

use crate::sobol::*;
use crate::sobol::params::JoeKuoD6;

use std::fs::File;
use std::fmt::Display;

use std::io::{BufRead, BufReader};
use libflate::gzip::Decoder;


lazy_static! {
    /** High-dimensional reference sequence for validation */
    static ref REF_SEQ_LO: Vec<Vec<f32>> = load_ref_seq("./tests/data/ref_seq_lo.tsv.gz");
    
    /** Low-dimensional reference sequence for validation */
    static ref REF_SEQ_HI: Vec<Vec<f32>> = load_ref_seq("./tests/data/ref_seq_hi.tsv.gz");
}


/** Validate low-dimensional `f32` sequence */
#[test] fn validate_f32_hires_lodim() {
    validate::<f32>(&REF_SEQ_LO, None);
}

/** Validate low-dimensional `f64` sequence */
#[test] fn validate_f64_hires_lodim() {
    validate::<f64>(&REF_SEQ_LO, None);
}

/** Validate low-dimensional `u8` sequence */
#[test] fn validate_u8_hires_lodim() {
    validate::<u8>(&REF_SEQ_LO, None);
}

/** Validate low-dimensional `u16` sequence */
#[test] fn validate_u16_hires_lodim() {
    validate::<u16>(&REF_SEQ_LO, None);
}

/** Validate low-dimensional `u32` sequence */
#[test] fn validate_u32_hires_lodim() {
    validate::<u32>(&REF_SEQ_LO, None);
}

/** Validate low-dimensional `u64` sequence */
#[test] fn validate_u64_hires_lodim() {
    validate::<u64>(&REF_SEQ_LO, None);
}

/** Validate low-dimensional `u128` sequence */
#[test] fn validate_u128_hires_lodim() {
    validate::<u128>(&REF_SEQ_LO, None);
}

/** Validate low-dimensional `i8` sequence */
#[test] fn validate_i8_hires_lodim() {
    validate::<i8>(&REF_SEQ_LO, None);
}

/** Validate low-dimensional `i16` sequence */
#[test] fn validate_i16_hires_lodim() {
    validate::<i16>(&REF_SEQ_LO, None);
}

/** Validate low-dimensional `i32` sequence */
#[test] fn validate_i32_hires_lodim() {
    validate::<i32>(&REF_SEQ_LO, None);
}

/** Validate low-dimensional `i64` sequence */
#[test] fn validate_i64_hires_lodim() {
    validate::<i64>(&REF_SEQ_LO, None);
}

/** Validate low-dimensional `i128` sequence */
#[test] fn validate_i128_hires_lodim() {
    validate::<i128>(&REF_SEQ_LO, None);
}


/** Validate high-dimensional `f32` sequence */
#[test] fn validate_f32_hires_hidim() {
    validate::<f32>(&REF_SEQ_HI, None);
}

/** Validate high-dimensional `f64` sequence */
#[test] fn validate_f64_hires_hidim() {
    validate::<f64>(&REF_SEQ_HI, None);
}

/** Validate high-dimensional `u8` sequence */
#[test] fn validate_u8_hires_hidim() {
    validate::<u8>(&REF_SEQ_HI, None);
}

/** Validate high-dimensional `u16` sequence */
#[test] fn validate_u16_hires_hidim() {
    validate::<u16>(&REF_SEQ_HI, None);
}

/** Validate high-dimensional `u32` sequence */
#[test] fn validate_u32_hires_hidim() {
    validate::<u32>(&REF_SEQ_HI, None);
}

/** Validate high-dimensional `u64` sequence */
#[test] fn validate_u64_hires_hidim() {
    validate::<u64>(&REF_SEQ_HI, None);
}

/** Validate high-dimensional `u128` sequence */
#[test] fn validate_u128_hires_hidim() {
    validate::<u128>(&REF_SEQ_HI, None);
}

/** Validate high-dimensional `i8` sequence */
#[test] fn validate_i8_hires_hidim() {
    validate::<i8>(&REF_SEQ_HI, None);
}

/** Validate high-dimensional `i16` sequence */
#[test] fn validate_i16_hires_hidim() {
    validate::<i16>(&REF_SEQ_HI, None);
}

/** Validate high-dimensional `i32` sequence */
#[test] fn validate_i32_hires_hidim() {
    validate::<i32>(&REF_SEQ_HI, None);
}

/** Validate high-dimensional `i64` sequence */
#[test] fn validate_i64_hires_hidim() {
    validate::<i64>(&REF_SEQ_HI, None);
}

/** Validate high-dimensional `i128` sequence */
#[test] fn validate_i128_hires_hidim() {
    validate::<i128>(&REF_SEQ_HI, None);
}

/** Validate low-resolution low-dimensional `f32` sequence */
#[test] fn validate_f32_lores_lodim() {
    validate::<f32>(&REF_SEQ_LO, Some(11));
}

/** Validate low-resolution low-dimensional `f64` sequence */
#[test] fn validate_f64_lores_lodim() {
    validate::<f64>(&REF_SEQ_LO, Some(11));
}

/** Validate low-resolution low-dimensional `u8` sequence */
#[test] fn validate_u8_lores_lodim() {
    validate::<u8>(&REF_SEQ_LO, Some(5));
}

/** Validate low-resolution low-dimensional `u16` sequence */
#[test] fn validate_u16_lores_lodim() {
    validate::<u16>(&REF_SEQ_LO, Some(11));
}

/** Validate low-resolution low-dimensional `u32` sequence */
#[test] fn validate_u32_lores_lodim() {
    validate::<u32>(&REF_SEQ_LO, Some(11));
}

/** Validate low-resolution low-dimensional `u64` sequence */
#[test] fn validate_u64_lores_lodim() {
    validate::<u64>(&REF_SEQ_LO, Some(11));
}

/** Validate low-resolution low-dimensional `u128` sequence */
#[test] fn validate_u128_lores_lodim() {
    validate::<u128>(&REF_SEQ_LO, Some(11));
}

/** Validate low-resolution low-dimensional `i8` sequence */
#[test] fn validate_i8_lores_lodim() {
    validate::<i8>(&REF_SEQ_LO, Some(5));
}

/** Validate low-resolution low-dimensional `i16` sequence */
#[test] fn validate_i16_lores_lodim() {
    validate::<i16>(&REF_SEQ_LO, Some(11));
}

/** Validate low-resolution low-dimensional `i32` sequence */
#[test] fn validate_i32_lores_lodim() {
    validate::<i32>(&REF_SEQ_LO, Some(11));
}

/** Validate low-resolution low-dimensional `i64` sequence */
#[test] fn validate_i64_lores_lodim() {
    validate::<i64>(&REF_SEQ_LO, Some(11));
}

/** Validate low-resolution low-dimensional `i128` sequence */
#[test] fn validate_i128_lores_lodim() {
    validate::<i128>(&REF_SEQ_LO, Some(11));
}


/** Validate low-resolution high-dimensional `f32` sequence */
#[test] fn validate_f32_lores_hidim() {
    validate::<f32>(&REF_SEQ_HI, Some(11));
}

/** Validate low-resolution high-dimensional `f64` sequence */
#[test] fn validate_f64_lores_hidim() {
    validate::<f64>(&REF_SEQ_HI, Some(11));
}

/** Validate low-resolution high-dimensional `u8` sequence */
#[test] fn validate_u8_lores_hidim() {
    validate::<u8>(&REF_SEQ_HI, Some(5));
}

/** Validate low-resolution high-dimensional `u16` sequence */
#[test] fn validate_u16_lores_hidim() {
    validate::<u16>(&REF_SEQ_HI, Some(11));
}

/** Validate low-resolution high-dimensional `u32` sequence */
#[test] fn validate_u32_lores_hidim() {
    validate::<u32>(&REF_SEQ_HI, Some(11));
}

/** Validate low-resolution high-dimensional `u64` sequence */
#[test] fn validate_u64_lores_hidim() {
    validate::<u64>(&REF_SEQ_HI, Some(11));
}

/** Validate low-resolution high-dimensional `u128` sequence */
#[test] fn validate_u128_lores_hidim() {
    validate::<u128>(&REF_SEQ_HI, Some(11));
}

/** Validate low-resolution high-dimensional `i8` sequence */
#[test] fn validate_i8_lores_hidim() {
    validate::<i8>(&REF_SEQ_HI, Some(5));
}

/** Validate low-resolution high-dimensional `i16` sequence */
#[test] fn validate_i16_lores_hidim() {
    validate::<i16>(&REF_SEQ_HI, Some(11));
}

/** Validate low-resolution high-dimensional `i32` sequence */
#[test] fn validate_i32_lores_hidim() {
    validate::<i32>(&REF_SEQ_HI, Some(11));
}

/** Validate low-resolution high-dimensional `i64` sequence */
#[test] fn validate_i64_lores_hidim() {
    validate::<i64>(&REF_SEQ_HI, Some(11));
}

/** Validate low-resolution high-dimensional `i128` sequence */
#[test] fn validate_i128_lores_hidim() {
    validate::<i128>(&REF_SEQ_HI, Some(11));
}


/**
 * Generates a sequence of type T and compares values to an externally generated
 * reference sequence (see 'test/data/ref_seq_*.tsv.gz')
 */
fn validate<T>(ref_seq: &Vec<Vec<f32>>, resolution: Option<usize>) 
    where T: SobolType + ToFloat + Display, T::IT: LossyFrom<u32>,
{
    let dims: usize = ref_seq[0].len();
    let params = JoeKuoD6::load_extended();

    Sobol::<T>::new_with_resolution(dims, &params, resolution)
        .map(|s| s.iter().map(|v| v.to_float()).collect::<Vec<_>>())
        .zip(ref_seq.iter().map(|p| p.to_vec()).collect::<Vec<_>>())
        .enumerate()
        .filter(|(_, (s, r))| s != r)
        .take(1)
        .for_each(|(i, (s, r))| {
            panic!("Generated point #{} does not match point from reference sequence!\n  --> generated = {}\n  --> expected =  {}", i, point_str(&s), point_str(&r));
        });
}

/** Generates string representation of a multi-dimensional point for display */
fn point_str<T: Display>(point: &Vec<T>) -> String {
    format!("[{}]", point.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(","))
}

/** Loads reference sequence from gzipped tsv file */
fn load_ref_seq(filename: &str) -> Vec<Vec<f32>> {
    let mut file = File::open(filename)
        .expect(&format!("Can't open reference sequence file: {}", filename));
    let mut decoder = Decoder::new(&mut file).unwrap();
    BufReader::new(&mut decoder)
        .lines()
        .map(|res| res.unwrap().split_whitespace()
             .map(|v| v.parse::<f32>().unwrap())
             .collect())
        .collect()
}

/** Converts a generated value to canonical `f32` for comparison with reference sequence */
pub trait ToFloat {
    fn to_float(&self) -> f32;
}

impl ToFloat for f32 {
    fn to_float(&self) -> f32 {
        *self
    }
}

impl ToFloat for f64 {
    fn to_float(&self) -> f32 {
        *self as f32
    }
}

impl ToFloat for u8 {
    fn to_float(&self) -> f32 {
        (*self as f32) / 2f32.powi(8)
    }

}

impl ToFloat for u16 {
    fn to_float(&self) -> f32 {
        (*self as f32) / 2f32.powi(16)
    }
}

impl ToFloat for u32 {
    fn to_float(&self) -> f32 {
        (*self as f32) / 2f32.powi(32)
    }
}

impl ToFloat for u64 {
    fn to_float(&self) -> f32 {
        (*self as f32) / 2f32.powi(64)
    }
}

impl ToFloat for u128 {
    fn to_float(&self) -> f32 {
        ((*self as f64) / 2f64.powi(128)) as f32
    }
}

impl ToFloat for i8 {
    fn to_float(&self) -> f32 {
        let range = 2f32.powi(8);
        ((*self as f32) + range / 2.0) / range
    }
}

impl ToFloat for i16 {
    fn to_float(&self) -> f32 {
        let range = 2f32.powi(16);
        ((*self as f32) + range / 2.0) / range
    }
}

impl ToFloat for i32 {
    fn to_float(&self) -> f32 {
        let range = 2f32.powi(32);
        ((*self as f32) + range / 2.0) / range
    }
}

impl ToFloat for i64 {
    fn to_float(&self) -> f32 {
        let range = 2f32.powi(64);
        ((*self as f32) + range / 2.0) / range
    }
}

impl ToFloat for i128 {
    fn to_float(&self) -> f32 {
        let range = 2f64.powi(128);
        (((*self as f64) + range / 2.0) / range) as f32
    }
}
