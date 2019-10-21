#![stable(feature = "core_ascii", since = "1.26.0")]

use crate::fmt;
use crate::ops::Range;
use crate::iter::FusedIterator;
use crate::str::from_utf8_unchecked;

#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Clone)]
pub struct EscapeDefault {
    range: Range<usize>,
    data: [u8; 4],
}

#[stable(feature = "rust1", since = "1.0.0")]
pub fn escape_default(c: u8) -> EscapeDefault { loop { } }

#[stable(feature = "rust1", since = "1.0.0")]
impl Iterator for EscapeDefault {
    type Item = u8;
    fn next(&mut self) -> Option<u8> { loop { } }
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }
    fn last(mut self) -> Option<u8> { loop { } }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl DoubleEndedIterator for EscapeDefault {
    fn next_back(&mut self) -> Option<u8> { loop { } }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl ExactSizeIterator for EscapeDefault {}
#[stable(feature = "fused", since = "1.26.0")]
impl FusedIterator for EscapeDefault {}

#[stable(feature = "ascii_escape_display", since = "1.39.0")]
impl fmt::Display for EscapeDefault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "std_debug", since = "1.16.0")]
impl fmt::Debug for EscapeDefault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}
