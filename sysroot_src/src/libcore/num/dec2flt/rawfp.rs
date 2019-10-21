//! Bit fiddling on positive IEEE 754 floats. Negative numbers aren't and needn't be handled.
use crate::cmp::Ordering::{Less, Equal, Greater};
use crate::convert::{TryFrom, TryInto};
use crate::ops::{Add, Mul, Div, Neg};
use crate::fmt::{Debug, LowerExp};
use crate::num::diy_float::Fp;
use crate::num::FpCategory::{Infinite, Zero, Subnormal, Normal, Nan};
use crate::num::FpCategory;
use crate::num::dec2flt::num::{self, Big};
use crate::num::dec2flt::table;

#[derive(Copy, Clone, Debug)]
pub struct Unpacked {
    pub sig: u64,
    pub k: i16,
}

impl Unpacked {
    pub fn new(sig: u64, k: i16) -> Self { loop { } }
}

pub trait RawFloat
    : Copy
    + Debug
    + LowerExp
    + Mul<Output=Self>
    + Div<Output=Self>
    + Neg<Output=Self>
{
    const INFINITY: Self;
    const NAN: Self;
    const ZERO: Self;

    type Bits: Add<Output = Self::Bits> + From<u8> + TryFrom<u64>;

    fn to_bits(self) -> Self::Bits;

    fn from_bits(v: Self::Bits) -> Self;

    fn classify(self) -> FpCategory;

    fn integer_decode(self) -> (u64, i16, i8);

    fn unpack(self) -> Unpacked;

    fn from_int(x: u64) -> Self;

    fn short_fast_pow10(e: usize) -> Self;

    const CEIL_LOG5_OF_MAX_SIG: i16;

    const MAX_NORMAL_DIGITS: usize;

    const INF_CUTOFF: i64;

    const ZERO_CUTOFF: i64;

    const EXP_BITS: u8;

    const SIG_BITS: u8;

    const EXPLICIT_SIG_BITS: u8;

    const MAX_EXP: i16;

    const MIN_EXP: i16;

    const MAX_EXP_INT: i16;

    const MAX_ENCODED_EXP: i16;

    const MIN_EXP_INT: i16;

    const MAX_SIG: u64;

    const MIN_SIG: u64;
}

macro_rules! other_constants {
    ($type: ident) => {
        const EXPLICIT_SIG_BITS: u8 = Self::SIG_BITS - 1;
        const MAX_EXP: i16 = (1 << (Self::EXP_BITS - 1)) - 1;
        const MIN_EXP: i16 = -Self::MAX_EXP + 1;
        const MAX_EXP_INT: i16 = Self::MAX_EXP - (Self::SIG_BITS as i16 - 1);
        const MAX_ENCODED_EXP: i16 = (1 << Self::EXP_BITS) - 1;
        const MIN_EXP_INT: i16 = Self::MIN_EXP - (Self::SIG_BITS as i16 - 1);
        const MAX_SIG: u64 = (1 << Self::SIG_BITS) - 1;
        const MIN_SIG: u64 = 1 << (Self::SIG_BITS - 1);

        const INFINITY: Self = $crate::$type::INFINITY;
        const NAN: Self = $crate::$type::NAN;
        const ZERO: Self = 0.0;
    }
}

impl RawFloat for f32 {
    type Bits = u32;

    const SIG_BITS: u8 = 24;
    const EXP_BITS: u8 = 8;
    const CEIL_LOG5_OF_MAX_SIG: i16 = 11;
    const MAX_NORMAL_DIGITS: usize = 35;
    const INF_CUTOFF: i64 = 40;
    const ZERO_CUTOFF: i64 = -48;
    other_constants!(f32);

    fn integer_decode(self) -> (u64, i16, i8) { loop { } }

    fn unpack(self) -> Unpacked { loop { } }

    fn from_int(x: u64) -> f32 { loop { } }

    fn short_fast_pow10(e: usize) -> Self { loop { } }

    fn classify(self) -> FpCategory { loop { } }
    fn to_bits(self) -> Self::Bits { loop { } }
    fn from_bits(v: Self::Bits) -> Self { loop { } }
}


impl RawFloat for f64 {
    type Bits = u64;

    const SIG_BITS: u8 = 53;
    const EXP_BITS: u8 = 11;
    const CEIL_LOG5_OF_MAX_SIG: i16 = 23;
    const MAX_NORMAL_DIGITS: usize = 305;
    const INF_CUTOFF: i64 = 310;
    const ZERO_CUTOFF: i64 = -326;
    other_constants!(f64);

    fn integer_decode(self) -> (u64, i16, i8) { loop { } }

    fn unpack(self) -> Unpacked { loop { } }

    fn from_int(x: u64) -> f64 { loop { } }

    fn short_fast_pow10(e: usize) -> Self { loop { } }

    fn classify(self) -> FpCategory { loop { } }
    fn to_bits(self) -> Self::Bits { loop { } }
    fn from_bits(v: Self::Bits) -> Self { loop { } }
}

pub fn fp_to_float<T: RawFloat>(x: Fp) -> T { loop { } }

pub fn round_normal<T: RawFloat>(x: Fp) -> Unpacked { loop { } }

pub fn encode_normal<T: RawFloat>(x: Unpacked) -> T { loop { } }

pub fn encode_subnormal<T: RawFloat>(significand: u64) -> T { loop { } }

pub fn big_to_fp(f: &Big) -> Fp { loop { } }

pub fn prev_float<T: RawFloat>(x: T) -> T { loop { } }

pub fn next_float<T: RawFloat>(x: T) -> T { loop { } }
