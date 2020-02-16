use crate::{SobolType, InternalType, LossyFrom};

use std::{u8, u16, u32, u64, u128};
use std::{i8, i16, i32, i64, i128};
use std::{f32, f64};


/** SobolType implementation for 32-bit floating-point values */
impl SobolType for f32 {
    type IT = u32;
    const MAX_RESOLUTION: usize = 24; // IEEE-754 "binary32" significand = 24 bits
    fn render(val: u32) -> f32 {
        (val as f32) / 4_294_967_296_f32
    }
}

/** SobolType implementation for 64-bit floating-point values */
impl SobolType for f64 {
    type IT = u64;
    const MAX_RESOLUTION: usize = 53; // IEEE-754 "binary64" significand = 53 bits
    fn render(val: u64) -> f64 {
        (val as f64) / 18_446_744_073_709_551_616_f64
    }
}

/** SobolType implementation for 8-bit unsigned values */
impl SobolType for u8 {
    type IT = u8;
    fn render(val: u8) -> u8 {
        val
    }
}

/** SobolType implementation for 16-bit unsigned values */
impl SobolType for u16 {
    type IT = u16;
    fn render(val: u16) -> u16 {
        val
    }
}

/** SobolType implementation for 32-bit unsigned values */
impl SobolType for u32 {
    type IT = u32;
    fn render(val: u32) -> u32 {
        val
    }
}

/** SobolType implementation for 64-bit unsigned values */
impl SobolType for u64 {
    type IT = u64;
    fn render(val: u64) -> u64 {
        val
    }
}

/** SobolType implementation for 128-bit unsigned values */
impl SobolType for u128 {
    type IT = u128;
    fn render(val: u128) -> u128 {
        val
    }
}

/** SobolType implementation for 8-bit signed values */
impl SobolType for i8 {
    type IT = u8;
    fn render(val: u8) -> i8 {
        (val ^ 0x80) as i8
    }
}

/** SobolType implementation for 16-bit signed values */
impl SobolType for i16 {
    type IT = u16;
    fn render(val: u16) -> i16 {
        (val ^ 0x8000) as i16
    }
}

/** SobolType implementation for 32-bit signed values */
impl SobolType for i32 {
    type IT = u32;
    fn render(val: u32) -> i32 {
        (val ^ 0x8000_0000) as i32
    }
}

/** SobolType implementation for 64-bit signed values */
impl SobolType for i64 {
    type IT = u64;
    fn render(val: u64) -> i64 {
        (val ^ 0x8000_0000_0000_0000) as i64
    }
}

/** SobolType implementation for 128-bit signed values */
impl SobolType for i128 {
    type IT = u128;
    fn render(val: u128) -> i128 {
        (val ^ 0x8000_0000_0000_0000_0000_0000_0000_0000) as i128
    }
}


/** InternalType implementation for 8-bit values */
impl InternalType for u8 {
    const BITS: usize = 8;
}

/** InternalType implementation for 16-bit values */
impl InternalType for u16 {
    const BITS: usize = 16;
}

/** InternalType implementation for 32-bit values */
impl InternalType for u32 {
    const BITS: usize = 32;
}

/** InternalType implementation for 64-bit values */
impl InternalType for u64 {
    const BITS: usize = 64;
}

/** InternalType implementation for 128-bit values */
impl InternalType for u128 {
    const BITS: usize = 128;
}


/** Reflexive `LossyFrom` */
impl<T> LossyFrom<T> for T {
    fn lossy_from(val: T) -> T {
        val
    }
}

/** LossyFrom `u16` to `u8` */
impl LossyFrom<u16> for u8 {
    fn lossy_from(val: u16) -> u8 {
        val as u8
    }
}

/** LossyFrom `u16` to `u32` */
impl LossyFrom<u16> for u32 {
    fn lossy_from(val: u16) -> u32 {
        u32::from(val)
    }
}

/** LossyFrom `u16` to `u64` */
impl LossyFrom<u16> for u64 {
    fn lossy_from(val: u16) -> u64 {
        u64::from(val)
    }
}

/** LossyFrom `u16` to `u128` */
impl LossyFrom<u16> for u128 {
    fn lossy_from(val: u16) -> u128 {
        u128::from(val)
    }
}

/** LossyFrom `u32` to `u8` */
impl LossyFrom<u32> for u8 {
    fn lossy_from(val: u32) -> u8 {
        val as u8
    }
}

/** LossyFrom `u32` to `u16` */
impl LossyFrom<u32> for u16 {
    fn lossy_from(val: u32) -> u16 {
        val as u16
    }
}

/** LossyFrom `u32` to `u64` */
impl LossyFrom<u32> for u64 {
    fn lossy_from(val: u32) -> u64 {
        u64::from(val)
    }
}

/** LossyFrom `u32` to `u128` */
impl LossyFrom<u32> for u128 {
    fn lossy_from(val: u32) -> u128 {
        u128::from(val)
    }
}

/** LossyFrom `u64` to `u8` */
impl LossyFrom<u64> for u8 {
    fn lossy_from(val: u64) -> u8 {
        val as u8
    }
}

/** LossyFrom `u64` to `u16` */
impl LossyFrom<u64> for u16 {
    fn lossy_from(val: u64) -> u16 {
        val as u16
    }
}

/** LossyFrom `u64` to `u32` */
impl LossyFrom<u64> for u32 {
    fn lossy_from(val: u64) -> u32 {
        val as u32
    }
}

/** LossyFrom `u64` to `u128` */
impl LossyFrom<u64> for u128 {
    fn lossy_from(val: u64) -> u128 {
        u128::from(val)
    }
}

/** LossyFrom `u128` to `u8` */
impl LossyFrom<u128> for u8 {
    fn lossy_from(val: u128) -> u8 {
        val as u8
    }
}

/** LossyFrom `u128` to `u16` */
impl LossyFrom<u128> for u16 {
    fn lossy_from(val: u128) -> u16 {
        val as u16
    }
}

/** LossyFrom `u128` to `u32` */
impl LossyFrom<u128> for u32 {
    fn lossy_from(val: u128) -> u32 {
        val as u32
    }
}

/** LossyFrom `u128` to `u64` */
impl LossyFrom<u128> for u64 {
    fn lossy_from(val: u128) -> u64 {
        val as u64
    }
}
