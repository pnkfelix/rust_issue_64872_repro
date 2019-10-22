//! Character conversions.

use crate::convert::TryFrom;
use crate::fmt;
use crate::str::FromStr;

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
pub fn from_u32(i: u32) -> Option<char> { loop { } }

#[inline]
#[stable(feature = "char_from_unchecked", since = "1.5.0")]
pub unsafe fn from_u32_unchecked(i: u32) -> char { loop { } }

#[stable(feature = "char_convert", since = "1.13.0")]
impl From<char> for u32 {
    #[inline]
    fn from(c: char) -> Self { loop { } }
}

#[stable(feature = "char_convert", since = "1.13.0")]
impl From<u8> for char {
    #[inline]
    fn from(i: u8) -> Self { loop { } }
}


#[stable(feature = "char_from_str", since = "1.20.0")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseCharError {
    kind: CharErrorKind,
}

impl ParseCharError {
    #[unstable(feature = "char_error_internals",
               reason = "this method should not be available publicly",
               issue = "0")]
    #[doc(hidden)]
    pub fn __description(&self) -> &str { loop { } }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CharErrorKind {
    EmptyString,
    TooManyChars,
}

#[stable(feature = "char_from_str", since = "1.20.0")]
impl fmt::Display for ParseCharError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}


#[stable(feature = "char_from_str", since = "1.20.0")]
impl FromStr for char {
    type Err = ParseCharError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> { loop { } }
}


#[stable(feature = "try_from", since = "1.34.0")]
impl TryFrom<u32> for char {
    type Error = CharTryFromError;

    #[inline]
    fn try_from(i: u32) -> Result<Self, Self::Error> { loop { } }
}

#[stable(feature = "try_from", since = "1.34.0")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CharTryFromError(());

#[stable(feature = "try_from", since = "1.34.0")]
impl fmt::Display for CharTryFromError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
pub fn from_digit(num: u32, radix: u32) -> Option<char> { loop { } }
