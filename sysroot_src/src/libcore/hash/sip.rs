//! An implementation of SipHash.

#![allow(deprecated)] // the types in this module are deprecated

use crate::marker::PhantomData;
use crate::ptr;
use crate::cmp;
use crate::mem;

#[unstable(feature = "hashmap_internals", issue = "0")]
#[rustc_deprecated(since = "1.13.0",
                   reason = "use `std::collections::hash_map::DefaultHasher` instead")]
// #[derive(Debug, Clone, Default)]
#[doc(hidden)]
pub struct SipHasher13 {

}

#[unstable(feature = "hashmap_internals", issue = "0")]
#[rustc_deprecated(since = "1.13.0",
                   reason = "use `std::collections::hash_map::DefaultHasher` instead")]
#[derive(Debug, Clone, Default)]
struct SipHasher24 {
    hasher: Hasher<Sip24Rounds>,
}

#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_deprecated(since = "1.13.0",
                   reason = "use `std::collections::hash_map::DefaultHasher` instead")]
#[derive(Debug, Clone, Default)]
pub struct SipHasher(SipHasher24);

#[derive(Debug)]
struct Hasher<S: Sip> {
    k0: u64,
    k1: u64,
    length: usize, // how many bytes we've processed
    state: State, // hash State
    tail: u64, // unprocessed bytes le
    ntail: usize, // how many bytes in tail are valid
    _marker: PhantomData<S>,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct State {
    v0: u64,
    v2: u64,
    v1: u64,
    v3: u64,
}

macro_rules! compress {
    ($state:expr) => ({
        compress!($state.v0, $state.v1, $state.v2, $state.v3)
    });
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr) =>
    ({
        $v0 = $v0.wrapping_add($v1); $v1 = $v1.rotate_left(13); $v1 ^= $v0;
        $v0 = $v0.rotate_left(32);
        $v2 = $v2.wrapping_add($v3); $v3 = $v3.rotate_left(16); $v3 ^= $v2;
        $v0 = $v0.wrapping_add($v3); $v3 = $v3.rotate_left(21); $v3 ^= $v0;
        $v2 = $v2.wrapping_add($v1); $v1 = $v1.rotate_left(17); $v1 ^= $v2;
        $v2 = $v2.rotate_left(32);
    });
}

macro_rules! load_int_le {
    ($buf:expr, $i:expr, $int_ty:ident) =>
    ({
       debug_assert!($i + mem::size_of::<$int_ty>() <= $buf.len());
       let mut data = 0 as $int_ty;
       ptr::copy_nonoverlapping($buf.get_unchecked($i),
                                &mut data as *mut _ as *mut u8,
                                mem::size_of::<$int_ty>());
       data.to_le()
    });
}

#[inline]
unsafe fn u8to64_le(buf: &[u8], start: usize, len: usize) -> u64 { loop { } }

impl SipHasher {
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_deprecated(since = "1.13.0",
                       reason = "use `std::collections::hash_map::DefaultHasher` instead")]
    pub fn new() -> SipHasher { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_deprecated(since = "1.13.0",
                       reason = "use `std::collections::hash_map::DefaultHasher` instead")]
    pub fn new_with_keys(key0: u64, key1: u64) -> SipHasher { loop { } }
}

impl SipHasher13 {
    #[inline]
    #[unstable(feature = "hashmap_internals", issue = "0")]
    #[rustc_deprecated(since = "1.13.0",
                       reason = "use `std::collections::hash_map::DefaultHasher` instead")]
    pub fn new() -> SipHasher13 { loop { } }

    #[inline]
    #[unstable(feature = "hashmap_internals", issue = "0")]
    #[rustc_deprecated(since = "1.13.0",
                       reason = "use `std::collections::hash_map::DefaultHasher` instead")]
    pub fn new_with_keys(key0: u64, key1: u64) -> SipHasher13 { loop { } }
}

impl<S: Sip> Hasher<S> {
    #[inline]
    fn new_with_keys(key0: u64, key1: u64) -> Hasher<S> { loop { } }

    #[inline]
    fn reset(&mut self) { loop { } }

    #[inline]
    fn short_write(&mut self, msg: &[u8]) { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl super::Hasher for SipHasher {
    #[inline]
    fn write(&mut self, msg: &[u8]) { loop { } }

    #[inline]
    fn finish(&self) -> u64 { loop { } }
}

#[unstable(feature = "hashmap_internals", issue = "0")]
impl super::Hasher for SipHasher13 {
    #[inline]
    fn write(&mut self, msg: &[u8]) { loop { } }

    #[inline]
    fn finish(&self) -> u64 { loop { } }
}

impl<S: Sip> super::Hasher for Hasher<S> {
    #[inline]
    fn write_usize(&mut self, i: usize) { loop { } }

    #[inline]
    fn write_u8(&mut self, i: u8) { loop { } }

    #[inline]
    fn write(&mut self, msg: &[u8]) { loop { } }

    #[inline]
    fn finish(&self) -> u64 { loop { } }
}

impl<S: Sip> Clone for Hasher<S> {
    #[inline]
    fn clone(&self) -> Hasher<S> { loop { } }
}

impl<S: Sip> Default for Hasher<S> {
    #[inline]
    fn default() -> Hasher<S> { loop { } }
}

#[doc(hidden)]
trait Sip {
    fn c_rounds(_: &mut State);
    fn d_rounds(_: &mut State);
}

#[derive(Debug, Clone, Default)]
struct Sip13Rounds;

#[derive(Debug, Clone, Default)]
struct Sip24Rounds;

impl Sip for Sip24Rounds {
    #[inline]
    fn c_rounds(state: &mut State) { loop { } }

    #[inline]
    fn d_rounds(state: &mut State) { loop { } }
}
