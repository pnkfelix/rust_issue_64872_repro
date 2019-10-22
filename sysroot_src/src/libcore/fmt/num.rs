//! Integer and floating-point number formatting


use crate::fmt;
use crate::ops::{Div, Rem, Sub};

#[doc(hidden)]
trait Int: PartialEq + PartialOrd + Div<Output=Self> + Rem<Output=Self> +
           Sub<Output=Self> + Sized {
    fn zero() -> Self;
    fn from_u8(u: u8) -> Self;
    fn to_u8(&self) -> u8;
    fn to_u16(&self) -> u16;
    fn to_u32(&self) -> u32;
    fn to_u64(&self) -> u64;
    fn to_u128(&self) -> u128;
}

macro_rules! doit {
    ($($t:ident)*) => ($(impl Int for $t {
        fn zero() -> $t { loop { } }
        fn from_u8(u: u8) -> $t { loop { } }
        fn to_u8(&self) -> u8 { loop { } }
        fn to_u16(&self) -> u16 { loop { } }
        fn to_u32(&self) -> u32 { loop { } }
        fn to_u64(&self) -> u64 { loop { } }
        fn to_u128(&self) -> u128 { loop { } }
    })*)
}
doit! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }

#[doc(hidden)]
trait GenericRadix {
    const BASE: u8;

    const PREFIX: &'static str;

    fn digit(x: u8) -> u8;

    fn fmt_int<T: Int>(&self, mut x: T, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[derive(Clone, PartialEq)]
struct Binary;

#[derive(Clone, PartialEq)]
struct Octal;

#[derive(Clone, PartialEq)]
struct LowerHex;

#[derive(Clone, PartialEq)]
struct UpperHex;

macro_rules! radix {
    ($T:ident, $base:expr, $prefix:expr, $($x:pat => $conv:expr),+) => {
        impl GenericRadix for $T {
            const BASE: u8 = $base;
            const PREFIX: &'static str = $prefix;
            fn digit(x: u8) -> u8 { loop { } }
        }
    }
}

radix! { Binary,    2, "0b", x @  0 ..=  1 => b'0' + x }
radix! { Octal,     8, "0o", x @  0 ..=  7 => b'0' + x }
radix! { LowerHex, 16, "0x", x @  0 ..=  9 => b'0' + x,
                             x @ 10 ..= 15 => b'a' + (x - 10) }
radix! { UpperHex, 16, "0x", x @  0 ..=  9 => b'0' + x,
                             x @ 10 ..= 15 => b'A' + (x - 10) }

macro_rules! int_base {
    ($Trait:ident for $T:ident as $U:ident -> $Radix:ident) => {
        #[stable(feature = "rust1", since = "1.0.0")]
        impl fmt::$Trait for $T {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
        }
    }
}

macro_rules! debug {
    ($T:ident) => {
        #[stable(feature = "rust1", since = "1.0.0")]
        impl fmt::Debug for $T {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
        }
    }
}

macro_rules! integer {
    ($Int:ident, $Uint:ident) => {
        int_base! { Binary   for $Int as $Uint  -> Binary }
        int_base! { Octal    for $Int as $Uint  -> Octal }
        int_base! { LowerHex for $Int as $Uint  -> LowerHex }
        int_base! { UpperHex for $Int as $Uint  -> UpperHex }
        debug! { $Int }

        int_base! { Binary   for $Uint as $Uint -> Binary }
        int_base! { Octal    for $Uint as $Uint -> Octal }
        int_base! { LowerHex for $Uint as $Uint -> LowerHex }
        int_base! { UpperHex for $Uint as $Uint -> UpperHex }
        debug! { $Uint }
    }
}
integer! { isize, usize }
integer! { i8, u8 }
integer! { i16, u16 }
integer! { i32, u32 }
integer! { i64, u64 }
integer! { i128, u128 }


static DEC_DIGITS_LUT: &[u8; 200] =
    b"0001020304050607080910111213141516171819\
      2021222324252627282930313233343536373839\
      4041424344454647484950515253545556575859\
      6061626364656667686970717273747576777879\
      8081828384858687888990919293949596979899";

macro_rules! impl_Display {
    ($($t:ident),* as $u:ident via $conv_fn:ident named $name:ident) => {
        fn $name(mut n: $u, is_nonnegative: bool, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }

        $(
            #[stable(feature = "rust1", since = "1.0.0")]
            impl fmt::Display for $t {
                #[allow(unused_comparisons)]
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
            })*
    };
}

#[cfg(any(target_pointer_width = "64", target_arch = "wasm32"))]
mod imp {
    use super::*;
    impl_Display!(
        i8, u8, i16, u16, i32, u32, i64, u64, usize, isize
            as u64 via to_u64 named fmt_u64
    );
}

#[cfg(not(any(target_pointer_width = "64", target_arch = "wasm32")))]
mod imp {
    use super::*;
    impl_Display!(i8, u8, i16, u16, i32, u32, isize, usize as u32 via to_u32 named fmt_u32);
    impl_Display!(i64, u64 as u64 via to_u64 named fmt_u64);
}

impl_Display!(i128, u128 as u128 via to_u128 named fmt_u128);
