//! Utility functions for bignums that don't make too much sense to turn into methods.


use crate::cmp::Ordering::{self};

pub use crate::num::bignum::Big32x40 as Big;

pub fn compare_with_half_ulp(f: &Big, ones_place: usize) -> Ordering { loop { } }

pub fn from_str_unchecked<'a, T>(bytes: T) -> u64 where T : IntoIterator<Item=&'a u8> { loop { } }

pub fn digits_to_big(integral: &[u8], fractional: &[u8]) -> Big { loop { } }

pub fn to_u64(x: &Big) -> u64 { loop { } }



pub fn get_bits(x: &Big, start: usize, end: usize) -> u64 { loop { } }
