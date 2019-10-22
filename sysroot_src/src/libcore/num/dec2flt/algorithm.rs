//! The various algorithms from the paper.

use crate::num::diy_float::Fp;
use crate::num::dec2flt::rawfp::{RawFloat};
use crate::num::dec2flt::num::{Big};

const P: u32 = 64;


fn power_of_ten(e: i16) -> Fp { loop { } }

#[cfg(any(not(target_arch="x86"), target_feature="sse2"))]
mod fpu_precision {
    pub fn set_precision<T>() { }
}

#[cfg(all(target_arch="x86", not(target_feature="sse2")))]
mod fpu_precision {
    use crate::mem::size_of;

    pub struct FPUControlWord(u16);

    fn set_cw(cw: u16) { loop { } }

    pub fn set_precision<T>() -> FPUControlWord { loop { } }

    impl Drop for FPUControlWord {
        fn drop(&mut self) { loop { } }
    }
}

pub fn fast_path<T: RawFloat>(integral: &[u8], fractional: &[u8], e: i64) -> Option<T> { loop { } }

pub fn bellerophon<T: RawFloat>(f: &Big, e: i16) -> T { loop { } }

fn algorithm_r<T: RawFloat>(f: &Big, e: i16, z0: T) -> T { loop { } }

fn make_ratio(x: &mut Big, y: &mut Big, e: i16, k: i16) { loop { } }

pub fn algorithm_m<T: RawFloat>(f: &Big, e: i16) -> T { loop { } }

fn quick_start<T: RawFloat>(u: &mut Big, v: &mut Big, k: &mut i16) { loop { } }

fn underflow<T: RawFloat>(x: Big, v: Big, rem: Big) -> T { loop { } }

fn round_by_remainder<T: RawFloat>(v: Big, r: Big, q: u64, z: T) -> T { loop { } }
