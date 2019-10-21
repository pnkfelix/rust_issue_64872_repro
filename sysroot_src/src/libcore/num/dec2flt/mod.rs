//! Converting decimal strings into IEEE 754 binary floating point numbers.

#![doc(hidden)]
#![unstable(feature = "dec2flt",
            reason = "internal routines only exposed for testing",
            issue = "0")]

use crate::fmt;
use crate::str::FromStr;

use self::parse::{parse_decimal, Decimal, Sign, ParseResult};
use self::num::digits_to_big;
use self::rawfp::RawFloat;

mod algorithm;
mod table;
mod num;
pub mod rawfp;
pub mod parse;

macro_rules! from_str_float_impl {
    ($t:ty) => {
        #[stable(feature = "rust1", since = "1.0.0")]
        impl FromStr for $t {
            type Err = ParseFloatError;

            #[inline]
            fn from_str(src: &str) -> Result<Self, ParseFloatError> { loop { } }
        }
    }
}
from_str_float_impl!(f32);
from_str_float_impl!(f64);

#[derive(Debug, Clone, PartialEq, Eq)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct ParseFloatError {
    kind: FloatErrorKind
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FloatErrorKind {
    Empty,
    Invalid,
}

impl ParseFloatError {
    #[unstable(feature = "int_error_internals",
               reason = "available through Error trait and this method should \
                         not be exposed publicly",
               issue = "0")]
    #[doc(hidden)]
    pub fn __description(&self) -> &str { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl fmt::Display for ParseFloatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

fn pfe_empty() -> ParseFloatError { loop { } }

fn pfe_invalid() -> ParseFloatError { loop { } }

fn extract_sign(s: &str) -> (Sign, &str) { loop { } }

fn dec2flt<T: RawFloat>(s: &str) -> Result<T, ParseFloatError> { loop { } }

fn convert<T: RawFloat>(mut decimal: Decimal<'_>) -> Result<T, ParseFloatError> {
    loop { }
}


#[inline(always)]
fn simplify(decimal: &mut Decimal<'_>) { loop { } }

fn bound_intermediate_digits(decimal: &Decimal<'_>, e: i64) -> u64 { loop { } }

fn trivial_cases<T: RawFloat>(decimal: &Decimal<'_>) -> Option<T> { loop { } }
