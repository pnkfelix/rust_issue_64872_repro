//! Converting decimal strings into IEEE 754 binary floating point numbers.

#![doc(hidden)]
#![unstable(feature = "dec2flt",
            reason = "internal routines only exposed for testing",
            issue = "0")]

use crate::fmt;

use self::parse::{Decimal, Sign};
use self::rawfp::RawFloat;

mod algorithm;
mod table;
mod num;
pub mod rawfp;
pub mod parse;

#[derive(Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct ParseFloatError {
    kind: FloatErrorKind
}

#[derive(Debug)]
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
