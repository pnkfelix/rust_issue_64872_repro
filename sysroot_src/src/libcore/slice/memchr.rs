// Original implementation taken from rust-memchr.

use crate::cmp;
use crate::mem;

const LO_U64: u64 = 0x0101010101010101;
const HI_U64: u64 = 0x8080808080808080;

const LO_USIZE: usize = LO_U64 as usize;
const HI_USIZE: usize = HI_U64 as usize;

#[inline]
fn contains_zero_byte(x: usize) -> bool { loop { } }

#[cfg(target_pointer_width = "16")]
#[inline]
fn repeat_byte(b: u8) -> usize { loop { } }

#[cfg(not(target_pointer_width = "16"))]
#[inline]
fn repeat_byte(b: u8) -> usize { loop { } }

pub fn memchr(x: u8, text: &[u8]) -> Option<usize> { loop { } }

pub fn memrchr(x: u8, text: &[u8]) -> Option<usize> { loop { } }
