//! The various algorithms from the paper.

use crate::num::diy_float::Fp;

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
