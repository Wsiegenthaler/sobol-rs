extern crate libflate;

use crate::{SobolParams, ParamDimension};

use std::io::{BufRead, BufReader, Cursor};
use libflate::gzip::Decoder;


pub struct JoeKuoD6 {
    pub dim_params: Vec<JoeKuoD6Dim>,
    pub max_dims: usize
}

impl JoeKuoD6 {

    /**
     * Load parameter values supporting up to 1000 dimensions
     */
    pub fn standard() -> Self {
        JoeKuoD6::load_gz_bytes(include_bytes!("data/new-joe-kuo-6.1000.gz"))
    }

    /**
     * Load parameter values supporting up to 100 dimensions
     */
    pub fn minimal() -> Self {
        JoeKuoD6::load_gz_bytes(include_bytes!("data/new-joe-kuo-6.100.gz"))
    }

    /**
     * Load parameter values supporting up to 21,201 dimensions
     */
    pub fn extended() -> Self {
        JoeKuoD6::load_gz_bytes(include_bytes!("data/new-joe-kuo-6.21201.gz"))
    }

    /** Instantiates parameter struct from gz sequence of bytes */
    fn load_gz_bytes(bytes: &[u8]) -> JoeKuoD6 {
        let mut byte_cursor = Cursor::new(bytes);
        let gz_decoder = Decoder::new(&mut byte_cursor).unwrap();
        let dim_params: Vec<JoeKuoD6Dim> = BufReader::new(gz_decoder)
            .lines()
            .skip(1)
            .map(|l| JoeKuoD6Dim::parse(&l.unwrap()))
            .collect();
        let max_dims = dim_params.len() + 1;
        JoeKuoD6 { dim_params, max_dims }
    }
}

impl SobolParams<u32> for JoeKuoD6 {
    #[inline]
    fn get_dim(&self, dim: usize) -> &dyn ParamDimension<u32> {
        &self.dim_params[dim - 2]
    }

    #[inline]
    fn max_dims(&self) -> usize {
        self.max_dims
    }
}

/** Parameters for a single dimension */
pub struct JoeKuoD6Dim {
    pub d: u16,
    pub a: u32,
    pub m: Vec<u32>
}

impl JoeKuoD6Dim {

    /** Parses the dimensional parameters from string according to the format provided by Joe/Kuo */
    pub fn parse(s: &str) -> Self {
        let mut tokens = s.split_whitespace();
        let d = tokens.next().unwrap().parse::<u16>().unwrap();
        tokens.next();
        let a = tokens.next().unwrap().parse::<u32>().unwrap();
        let m = tokens.map(|t| t.parse::<u32>().unwrap()).collect();
        JoeKuoD6Dim { d, a, m }
    }
}

impl ParamDimension<u32> for JoeKuoD6Dim {

    #[inline]
    fn d(&self) -> u16 {
        self.d
    }

    #[inline]
    fn s(&self) -> usize {
        self.m.len()
    }
    
    #[inline]
    fn coefficient(&self, i: usize) -> u32 {
        (self.a >> i) & 1
    }
    
    #[inline]
    fn m(&self, i: usize) -> u32 {
        self.m[i]
    }
}
