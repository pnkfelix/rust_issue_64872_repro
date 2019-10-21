//! Generic hashing support.

#![stable(feature = "rust1", since = "1.0.0")]

use crate::fmt;
use crate::marker;

#[stable(feature = "rust1", since = "1.0.0")]
#[allow(deprecated)]
pub use self::sip::SipHasher;

#[unstable(feature = "hashmap_internals", issue = "0")]
#[allow(deprecated)]
#[doc(hidden)]
pub use self::sip::SipHasher13;

mod sip;

#[stable(feature = "rust1", since = "1.0.0")]
pub trait Hash {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn hash<H: Hasher>(&self, state: &mut H);

    #[stable(feature = "hash_slice", since = "1.3.0")]
    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
        where Self: Sized
    { loop { } }
}

pub(crate) mod macros {
    #[rustc_builtin_macro]
    #[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
    #[allow_internal_unstable(core_intrinsics)]
    pub macro Hash($item:item) { /* compiler built-in */ }
}
#[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
#[doc(inline)]
pub use macros::Hash;

#[stable(feature = "rust1", since = "1.0.0")]
pub trait Hasher {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn finish(&self) -> u64;

    #[stable(feature = "rust1", since = "1.0.0")]
    fn write(&mut self, bytes: &[u8]);

    #[inline]
    #[stable(feature = "hasher_write", since = "1.3.0")]
    fn write_u8(&mut self, i: u8) { loop { } }
    #[inline]
    #[stable(feature = "hasher_write", since = "1.3.0")]
    fn write_u16(&mut self, i: u16) { loop { } }
    #[inline]
    #[stable(feature = "hasher_write", since = "1.3.0")]
    fn write_u32(&mut self, i: u32) { loop { } }
    #[inline]
    #[stable(feature = "hasher_write", since = "1.3.0")]
    fn write_u64(&mut self, i: u64) { loop { } }
    #[inline]
    #[stable(feature = "i128", since = "1.26.0")]
    fn write_u128(&mut self, i: u128) { loop { } }
    #[inline]
    #[stable(feature = "hasher_write", since = "1.3.0")]
    fn write_usize(&mut self, i: usize) { loop { } }

    #[inline]
    #[stable(feature = "hasher_write", since = "1.3.0")]
    fn write_i8(&mut self, i: i8) { loop { } }
    #[inline]
    #[stable(feature = "hasher_write", since = "1.3.0")]
    fn write_i16(&mut self, i: i16) { loop { } }
    #[inline]
    #[stable(feature = "hasher_write", since = "1.3.0")]
    fn write_i32(&mut self, i: i32) { loop { } }
    #[inline]
    #[stable(feature = "hasher_write", since = "1.3.0")]
    fn write_i64(&mut self, i: i64) { loop { } }
    #[inline]
    #[stable(feature = "i128", since = "1.26.0")]
    fn write_i128(&mut self, i: i128) { loop { } }
    #[inline]
    #[stable(feature = "hasher_write", since = "1.3.0")]
    fn write_isize(&mut self, i: isize) { loop { } }
}

#[stable(feature = "indirect_hasher_impl", since = "1.22.0")]
impl<H: Hasher + ?Sized> Hasher for &mut H {
    fn finish(&self) -> u64 { loop { } }
    fn write(&mut self, bytes: &[u8]) { loop { } }
    fn write_u8(&mut self, i: u8) { loop { } }
    fn write_u16(&mut self, i: u16) { loop { } }
    fn write_u32(&mut self, i: u32) { loop { } }
    fn write_u64(&mut self, i: u64) { loop { } }
    fn write_u128(&mut self, i: u128) { loop { } }
    fn write_usize(&mut self, i: usize) { loop { } }
    fn write_i8(&mut self, i: i8) { loop { } }
    fn write_i16(&mut self, i: i16) { loop { } }
    fn write_i32(&mut self, i: i32) { loop { } }
    fn write_i64(&mut self, i: i64) { loop { } }
    fn write_i128(&mut self, i: i128) { loop { } }
    fn write_isize(&mut self, i: isize) { loop { } }
}

#[stable(since = "1.7.0", feature = "build_hasher")]
pub trait BuildHasher {
    #[stable(since = "1.7.0", feature = "build_hasher")]
    type Hasher: Hasher;

    #[stable(since = "1.7.0", feature = "build_hasher")]
    fn build_hasher(&self) -> Self::Hasher;
}

#[stable(since = "1.7.0", feature = "build_hasher")]
pub struct BuildHasherDefault<H>(marker::PhantomData<H>);

#[stable(since = "1.9.0", feature = "core_impl_debug")]
impl<H> fmt::Debug for BuildHasherDefault<H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(since = "1.7.0", feature = "build_hasher")]
impl<H: Default + Hasher> BuildHasher for BuildHasherDefault<H> {
    type Hasher = H;

    fn build_hasher(&self) -> H { loop { } }
}

#[stable(since = "1.7.0", feature = "build_hasher")]
impl<H> Clone for BuildHasherDefault<H> {
    fn clone(&self) -> BuildHasherDefault<H> { loop { } }
}

#[stable(since = "1.7.0", feature = "build_hasher")]
impl<H> Default for BuildHasherDefault<H> {
    fn default() -> BuildHasherDefault<H> { loop { } }
}

#[stable(since = "1.29.0", feature = "build_hasher_eq")]
impl<H> PartialEq for BuildHasherDefault<H> {
    fn eq(&self, _other: &BuildHasherDefault<H>) -> bool { loop { } }
}

#[stable(since = "1.29.0", feature = "build_hasher_eq")]
impl<H> Eq for BuildHasherDefault<H> {}

mod impls {
    use super::*;

    macro_rules! impl_write {
        ($(($ty:ident, $meth:ident),)*) => {$(
            #[stable(feature = "rust1", since = "1.0.0")]
            impl Hash for $ty {
                fn hash<H: Hasher>(&self, state: &mut H) { loop { } }

                fn hash_slice<H: Hasher>(data: &[$ty], state: &mut H) { loop { } }
            }
        )*}
    }

    impl_write! {
        (u8, write_u8),
        (u16, write_u16),
        (u32, write_u32),
        (u64, write_u64),
        (usize, write_usize),
        (i8, write_i8),
        (i16, write_i16),
        (i32, write_i32),
        (i64, write_i64),
        (isize, write_isize),
        (u128, write_u128),
        (i128, write_i128),
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl Hash for bool {
        fn hash<H: Hasher>(&self, state: &mut H) { loop { } }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl Hash for char {
        fn hash<H: Hasher>(&self, state: &mut H) { loop { } }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl Hash for str {
        fn hash<H: Hasher>(&self, state: &mut H) { loop { } }
    }

    #[stable(feature = "never_hash", since = "1.29.0")]
    impl Hash for ! {
        fn hash<H: Hasher>(&self, _: &mut H) { loop { } }
    }

    macro_rules! impl_hash_tuple {
        () => (
            #[stable(feature = "rust1", since = "1.0.0")]
            impl Hash for () {
                fn hash<H: Hasher>(&self, _state: &mut H) { loop { } }
            }
        );

        ( $($name:ident)+) => (
            #[stable(feature = "rust1", since = "1.0.0")]
            impl<$($name: Hash),+> Hash for ($($name,)+) where last_type!($($name,)+): ?Sized {
                #[allow(non_snake_case)]
                fn hash<S: Hasher>(&self, state: &mut S) { loop { } }
            }
        );
    }

    macro_rules! last_type {
        ($a:ident,) => { $a };
        ($a:ident, $($rest_a:ident,)+) => { last_type!($($rest_a,)+) };
    }

    impl_hash_tuple! {}
    impl_hash_tuple! { A }
    impl_hash_tuple! { A B }
    impl_hash_tuple! { A B C }
    impl_hash_tuple! { A B C D }
    impl_hash_tuple! { A B C D E }
    impl_hash_tuple! { A B C D E F }
    impl_hash_tuple! { A B C D E F G }
    impl_hash_tuple! { A B C D E F G H }
    impl_hash_tuple! { A B C D E F G H I }
    impl_hash_tuple! { A B C D E F G H I J }
    impl_hash_tuple! { A B C D E F G H I J K }
    impl_hash_tuple! { A B C D E F G H I J K L }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl<T: Hash> Hash for [T] {
        fn hash<H: Hasher>(&self, state: &mut H) { loop { } }
    }


    #[stable(feature = "rust1", since = "1.0.0")]
    impl<T: ?Sized + Hash> Hash for &T {
        fn hash<H: Hasher>(&self, state: &mut H) { loop { } }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl<T: ?Sized + Hash> Hash for &mut T {
        fn hash<H: Hasher>(&self, state: &mut H) { loop { } }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl<T: ?Sized> Hash for *const T {
        fn hash<H: Hasher>(&self, state: &mut H) { loop { } }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl<T: ?Sized> Hash for *mut T {
        fn hash<H: Hasher>(&self, state: &mut H) { loop { } }
    }
}
