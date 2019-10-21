#![doc(hidden)]
#![unstable(feature = "flt2dec",
            reason = "internal routines only exposed for testing",
            issue = "0")]

use crate::i16;
pub use self::decoder::{decode, DecodableFloat, FullDecoded, Decoded};

pub mod estimator;
pub mod decoder;

pub mod strategy {
    pub mod dragon;
    pub mod grisu;
}

pub const MAX_SIG_DIGITS: usize = 17;

#[doc(hidden)]
pub fn round_up(d: &mut [u8], n: usize) -> Option<u8> { loop { } }

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Part<'a> {
    Zero(usize),
    Num(u16),
    Copy(&'a [u8]),
}

impl<'a> Part<'a> {
    pub fn len(&self) -> usize { loop { } }

    pub fn write(&self, out: &mut [u8]) -> Option<usize> { loop { } }
}

#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct Formatted<'a> {
    pub sign: &'static [u8],
    pub parts: &'a [Part<'a>],
}

impl<'a> Formatted<'a> {
    pub fn len(&self) -> usize { loop { } }

    pub fn write(&self, out: &mut [u8]) -> Option<usize> { loop { } }
}

fn digits_to_dec_str<'a>(buf: &'a [u8], exp: i16, frac_digits: usize,
                         parts: &'a mut [Part<'a>]) -> &'a [Part<'a>] { loop { } }

fn digits_to_exp_str<'a>(buf: &'a [u8], exp: i16, min_ndigits: usize, upper: bool,
                         parts: &'a mut [Part<'a>]) -> &'a [Part<'a>] { loop { } }

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Sign {
    Minus,        // -inf -1  0  0  1  inf nan
    MinusRaw,     // -inf -1 -0  0  1  inf nan
    MinusPlus,    // -inf -1 +0 +0 +1 +inf nan
    MinusPlusRaw, // -inf -1 -0 +0 +1 +inf nan
}

fn determine_sign(sign: Sign, decoded: &FullDecoded, negative: bool) -> &'static [u8] { loop { } }

pub fn to_shortest_str<'a, T, F>(mut format_shortest: F, v: T,
                                 sign: Sign, frac_digits: usize, _upper: bool,
                                 buf: &'a mut [u8], parts: &'a mut [Part<'a>]) -> Formatted<'a>
where T: DecodableFloat, F: FnMut(&Decoded, &mut [u8]) -> (usize, i16) { loop { } }

pub fn to_shortest_exp_str<'a, T, F>(mut format_shortest: F, v: T,
                                     sign: Sign, dec_bounds: (i16, i16), upper: bool,
                                     buf: &'a mut [u8], parts: &'a mut [Part<'a>]) -> Formatted<'a>
where T: DecodableFloat, F: FnMut(&Decoded, &mut [u8]) -> (usize, i16) { loop { } }

fn estimate_max_buf_len(exp: i16) -> usize { loop { } }

pub fn to_exact_exp_str<'a, T, F>(mut format_exact: F, v: T,
                                  sign: Sign, ndigits: usize, upper: bool,
                                  buf: &'a mut [u8], parts: &'a mut [Part<'a>]) -> Formatted<'a>
where T: DecodableFloat, F: FnMut(&Decoded, &mut [u8], i16) -> (usize, i16) { loop { } }

pub fn to_exact_fixed_str<'a, T, F>(mut format_exact: F, v: T,
                                    sign: Sign, frac_digits: usize, _upper: bool,
                                    buf: &'a mut [u8], parts: &'a mut [Part<'a>]) -> Formatted<'a>
where T: DecodableFloat, F: FnMut(&Decoded, &mut [u8], i16) -> (usize, i16) { loop { } }
