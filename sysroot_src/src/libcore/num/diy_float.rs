//! Extended precision "soft float", for internal use only.

#![doc(hidden)]
#![unstable(feature = "core_private_diy_float",
            reason = "internal routines only exposed for testing",
            issue = "0")]

#[derive(Debug)]
#[doc(hidden)]
pub struct Fp {
    pub f: u64,
    pub e: i16,
}

impl Fp {
    pub fn mul(&self, other: &Fp) -> Fp { loop { } }

    pub fn normalize(&self) -> Fp { loop { } }

    pub fn normalize_to(&self, e: i16) -> Fp { loop { } }
}
