// ignore-tidy-filelength


#![stable(feature = "rust1", since = "1.0.0")]

use crate::fmt;
use crate::intrinsics;
use crate::mem;

macro_rules! doc_comment {
    ($x:expr, $($tt:tt)*) => {
        #[doc = $x]
        $($tt)*
    };
}

#[stable(feature = "rust1", since = "1.0.0")]
#[repr(transparent)]
pub struct Wrapping<T>(#[stable(feature = "rust1", since = "1.0.0")]
                       pub T);

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: fmt::Debug> fmt::Debug for Wrapping<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "wrapping_display", since = "1.10.0")]
impl<T: fmt::Display> fmt::Display for Wrapping<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "wrapping_fmt", since = "1.11.0")]
impl<T: fmt::Binary> fmt::Binary for Wrapping<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "wrapping_fmt", since = "1.11.0")]
impl<T: fmt::Octal> fmt::Octal for Wrapping<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "wrapping_fmt", since = "1.11.0")]
impl<T: fmt::LowerHex> fmt::LowerHex for Wrapping<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "wrapping_fmt", since = "1.11.0")]
impl<T: fmt::UpperHex> fmt::UpperHex for Wrapping<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

// pub mod flt2dec;
pub mod dec2flt;
pub mod bignum;
pub mod diy_float;

mod wrapping;

macro_rules! usize_isize_to_xe_bytes_doc {
    () => {"

**Note**: This function returns an array of length 2, 4 or 8 bytes
depending on the target pointer size.

"}
}


macro_rules! usize_isize_from_xe_bytes_doc {
    () => {"

**Note**: This function takes an array of length 2, 4 or 8 bytes
depending on the target pointer size.

"}
}

macro_rules! int_impl {
    ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
     $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
     $reversed:expr, $le_bytes:expr, $be_bytes:expr,
     $to_xe_bytes_doc:expr, $from_xe_bytes_doc:expr) => {
        doc_comment! {
            concat!("Returns the smallest value that can be represented by this integer type.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(", stringify!($SelfT), "::min_value(), ", stringify!($Min), ");",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline(always)]
            #[rustc_promotable]
            pub const fn min_value() -> Self {
                !0 ^ ((!0 as $UnsignedT) >> 1) as Self
            }
        }

        doc_comment! {
            concat!("Returns the largest value that can be represented by this integer type.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(", stringify!($SelfT), "::max_value(), ", stringify!($Max), ");",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline(always)]
            #[rustc_promotable]
            pub const fn max_value() -> Self {
                !Self::min_value()
            }
        }

        doc_comment! {
            concat!("Converts a string slice in a given base to an integer.

The string is expected to be an optional `+` or `-` sign followed by digits.
Leading and trailing whitespace represent an error. Digits are a subset of these characters,
depending on `radix`:

 * `0-9`
 * `a-z`
 * `A-Z`

# Panics

This function panics if `radix` is not in the range from 2 to 36.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(", stringify!($SelfT), "::from_str_radix(\"A\", 16), Ok(10));",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> { loop { } }
        }

        doc_comment! {
            concat!("Returns the number of ones in the binary representation of `self`.

# Examples

Basic usage:

```
", $Feature, "let n = 0b100_0000", stringify!($SelfT), ";

assert_eq!(n.count_ones(), 1);",
$EndFeature, "
```
"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline]
            pub const fn count_ones(self) -> u32 { (self as $UnsignedT).count_ones() }
        }

        doc_comment! {
            concat!("Returns the number of zeros in the binary representation of `self`.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(", stringify!($SelfT), "::max_value().count_zeros(), 1);", $EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline]
            pub const fn count_zeros(self) -> u32 {
                (!self).count_ones()
            }
        }

        doc_comment! {
            concat!("Returns the number of leading zeros in the binary representation of `self`.

# Examples

Basic usage:

```
", $Feature, "let n = -1", stringify!($SelfT), ";

assert_eq!(n.leading_zeros(), 0);",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline]
            pub const fn leading_zeros(self) -> u32 {
                (self as $UnsignedT).leading_zeros()
            }
        }

        doc_comment! {
            concat!("Returns the number of trailing zeros in the binary representation of `self`.

# Examples

Basic usage:

```
", $Feature, "let n = -4", stringify!($SelfT), ";

assert_eq!(n.trailing_zeros(), 2);",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline]
            pub const fn trailing_zeros(self) -> u32 {
                (self as $UnsignedT).trailing_zeros()
            }
        }

        doc_comment! {
            concat!("Shifts the bits to the left by a specified amount, `n`,
wrapping the truncated bits to the end of the resulting integer.

Please note this isn't the same operation as the `<<` shifting operator!

# Examples

Basic usage:

```
let n = ", $rot_op, stringify!($SelfT), ";
let m = ", $rot_result, ";

assert_eq!(n.rotate_left(", $rot, "), m);
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub const fn rotate_left(self, n: u32) -> Self {
                (self as $UnsignedT).rotate_left(n) as Self
            }
        }

        doc_comment! {
            concat!("Shifts the bits to the right by a specified amount, `n`,
wrapping the truncated bits to the beginning of the resulting
integer.

Please note this isn't the same operation as the `>>` shifting operator!

# Examples

Basic usage:

```
let n = ", $rot_result, stringify!($SelfT), ";
let m = ", $rot_op, ";

assert_eq!(n.rotate_right(", $rot, "), m);
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub const fn rotate_right(self, n: u32) -> Self {
                (self as $UnsignedT).rotate_right(n) as Self
            }
        }

        doc_comment! {
            concat!("Reverses the byte order of the integer.

# Examples

Basic usage:

```
let n = ", $swap_op, stringify!($SelfT), ";

let m = n.swap_bytes();

assert_eq!(m, ", $swapped, ");
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline]
            pub const fn swap_bytes(self) -> Self {
                (self as $UnsignedT).swap_bytes() as Self
            }
        }

        doc_comment! {
            concat!("Reverses the bit pattern of the integer.

# Examples

Basic usage:

```
let n = ", $swap_op, stringify!($SelfT), ";
let m = n.reverse_bits();

assert_eq!(m, ", $reversed, ");
```"),
            #[stable(feature = "reverse_bits", since = "1.37.0")]
            #[inline]
            #[must_use]
            pub const fn reverse_bits(self) -> Self {
                (self as $UnsignedT).reverse_bits() as Self
            }
        }

        doc_comment! {
            concat!("Converts an integer from big endian to the target's endianness.

On big endian this is a no-op. On little endian the bytes are swapped.

# Examples

Basic usage:

```
", $Feature, "let n = 0x1A", stringify!($SelfT), ";

if cfg!(target_endian = \"big\") {
    assert_eq!(", stringify!($SelfT), "::from_be(n), n)
} else {
    assert_eq!(", stringify!($SelfT), "::from_be(n), n.swap_bytes())
}",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline]
            pub const fn from_be(x: Self) -> Self {
                #[cfg(target_endian = "big")]
                {
                    x
                }
                #[cfg(not(target_endian = "big"))]
                {
                    x.swap_bytes()
                }
            }
        }

        doc_comment! {
            concat!("Converts an integer from little endian to the target's endianness.

On little endian this is a no-op. On big endian the bytes are swapped.

# Examples

Basic usage:

```
", $Feature, "let n = 0x1A", stringify!($SelfT), ";

if cfg!(target_endian = \"little\") {
    assert_eq!(", stringify!($SelfT), "::from_le(n), n)
} else {
    assert_eq!(", stringify!($SelfT), "::from_le(n), n.swap_bytes())
}",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline]
            pub const fn from_le(x: Self) -> Self {
                #[cfg(target_endian = "little")]
                {
                    x
                }
                #[cfg(not(target_endian = "little"))]
                {
                    x.swap_bytes()
                }
            }
        }

        doc_comment! {
            concat!("Converts `self` to big endian from the target's endianness.

On big endian this is a no-op. On little endian the bytes are swapped.

# Examples

Basic usage:

```
", $Feature, "let n = 0x1A", stringify!($SelfT), ";

if cfg!(target_endian = \"big\") {
    assert_eq!(n.to_be(), n)
} else {
    assert_eq!(n.to_be(), n.swap_bytes())
}",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline]
            pub const fn to_be(self) -> Self { // or not to be?
                #[cfg(target_endian = "big")]
                {
                    self
                }
                #[cfg(not(target_endian = "big"))]
                {
                    self.swap_bytes()
                }
            }
        }

        doc_comment! {
            concat!("Converts `self` to little endian from the target's endianness.

On little endian this is a no-op. On big endian the bytes are swapped.

# Examples

Basic usage:

```
", $Feature, "let n = 0x1A", stringify!($SelfT), ";

if cfg!(target_endian = \"little\") {
    assert_eq!(n.to_le(), n)
} else {
    assert_eq!(n.to_le(), n.swap_bytes())
}",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline]
            pub const fn to_le(self) -> Self {
                #[cfg(target_endian = "little")]
                {
                    self
                }
                #[cfg(not(target_endian = "little"))]
                {
                    self.swap_bytes()
                }
            }
        }

        doc_comment! {
            concat!("Checked integer addition. Computes `self + rhs`, returning `None`
if overflow occurred.

# Examples

Basic usage:

```
", $Feature, "assert_eq!((", stringify!($SelfT),
"::max_value() - 2).checked_add(1), Some(", stringify!($SelfT), "::max_value() - 1));
assert_eq!((", stringify!($SelfT), "::max_value() - 2).checked_add(3), None);",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn checked_add(self, rhs: Self) -> Option<Self> {
                let (a, b) = self.overflowing_add(rhs);
                if b {None} else {Some(a)}
            }
        }

        doc_comment! {
            concat!("Checked integer subtraction. Computes `self - rhs`, returning `None` if
overflow occurred.

# Examples

Basic usage:

```
", $Feature, "assert_eq!((", stringify!($SelfT),
"::min_value() + 2).checked_sub(1), Some(", stringify!($SelfT), "::min_value() + 1));
assert_eq!((", stringify!($SelfT), "::min_value() + 2).checked_sub(3), None);",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn checked_sub(self, rhs: Self) -> Option<Self> { loop { } }
        }

        doc_comment! {
            concat!("Checked integer multiplication. Computes `self * rhs`, returning `None` if
overflow occurred.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(", stringify!($SelfT),
"::max_value().checked_mul(1), Some(", stringify!($SelfT), "::max_value()));
assert_eq!(", stringify!($SelfT), "::max_value().checked_mul(2), None);",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn checked_mul(self, rhs: Self) -> Option<Self> { loop { } }
        }

        doc_comment! {
            concat!("Checked integer division. Computes `self / rhs`, returning `None` if `rhs == 0`
or the division results in overflow.

# Examples

Basic usage:

```
", $Feature, "assert_eq!((", stringify!($SelfT),
"::min_value() + 1).checked_div(-1), Some(", stringify!($Max), "));
assert_eq!(", stringify!($SelfT), "::min_value().checked_div(-1), None);
assert_eq!((1", stringify!($SelfT), ").checked_div(0), None);",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn checked_div(self, rhs: Self) -> Option<Self> { loop { } }
        }

        doc_comment! {
            concat!("Checked Euclidean division. Computes `self.div_euclid(rhs)`,
returning `None` if `rhs == 0` or the division results in overflow.

# Examples

Basic usage:

```
assert_eq!((", stringify!($SelfT),
"::min_value() + 1).checked_div_euclid(-1), Some(", stringify!($Max), "));
assert_eq!(", stringify!($SelfT), "::min_value().checked_div_euclid(-1), None);
assert_eq!((1", stringify!($SelfT), ").checked_div_euclid(0), None);
```"),
            #[stable(feature = "euclidean_division", since = "1.38.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn checked_div_euclid(self, rhs: Self) -> Option<Self> { loop { } }
        }

        doc_comment! {
            concat!("Checked integer remainder. Computes `self % rhs`, returning `None` if
`rhs == 0` or the division results in overflow.

# Examples

Basic usage:

```
", $Feature, "use std::", stringify!($SelfT), ";

assert_eq!(5", stringify!($SelfT), ".checked_rem(2), Some(1));
assert_eq!(5", stringify!($SelfT), ".checked_rem(0), None);
assert_eq!(", stringify!($SelfT), "::MIN.checked_rem(-1), None);",
$EndFeature, "
```"),
            #[stable(feature = "wrapping", since = "1.7.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn checked_rem(self, rhs: Self) -> Option<Self> { loop { } }
        }

        doc_comment! {
            concat!("Checked Euclidean remainder. Computes `self.rem_euclid(rhs)`, returning `None`
if `rhs == 0` or the division results in overflow.

# Examples

Basic usage:

```
use std::", stringify!($SelfT), ";

assert_eq!(5", stringify!($SelfT), ".checked_rem_euclid(2), Some(1));
assert_eq!(5", stringify!($SelfT), ".checked_rem_euclid(0), None);
assert_eq!(", stringify!($SelfT), "::MIN.checked_rem_euclid(-1), None);
```"),
            #[stable(feature = "euclidean_division", since = "1.38.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn checked_rem_euclid(self, rhs: Self) -> Option<Self> { loop { } }
        }

        doc_comment! {
            concat!("Checked negation. Computes `-self`, returning `None` if `self == MIN`.

# Examples

Basic usage:

```
", $Feature, "use std::", stringify!($SelfT), ";

assert_eq!(5", stringify!($SelfT), ".checked_neg(), Some(-5));
assert_eq!(", stringify!($SelfT), "::MIN.checked_neg(), None);",
$EndFeature, "
```"),
            #[stable(feature = "wrapping", since = "1.7.0")]
            #[inline]
            pub fn checked_neg(self) -> Option<Self> { loop { } }
        }

        doc_comment! {
            concat!("Checked shift left. Computes `self << rhs`, returning `None` if `rhs` is larger
than or equal to the number of bits in `self`.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(0x1", stringify!($SelfT), ".checked_shl(4), Some(0x10));
assert_eq!(0x1", stringify!($SelfT), ".checked_shl(129), None);",
$EndFeature, "
```"),
            #[stable(feature = "wrapping", since = "1.7.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn checked_shl(self, rhs: u32) -> Option<Self> { loop { } }
        }

        doc_comment! {
            concat!("Checked shift right. Computes `self >> rhs`, returning `None` if `rhs` is
larger than or equal to the number of bits in `self`.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(0x10", stringify!($SelfT), ".checked_shr(4), Some(0x1));
assert_eq!(0x10", stringify!($SelfT), ".checked_shr(128), None);",
$EndFeature, "
```"),
            #[stable(feature = "wrapping", since = "1.7.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn checked_shr(self, rhs: u32) -> Option<Self> { loop { } }
        }

        doc_comment! {
            concat!("Checked absolute value. Computes `self.abs()`, returning `None` if
`self == MIN`.

# Examples

Basic usage:

```
", $Feature, "use std::", stringify!($SelfT), ";

assert_eq!((-5", stringify!($SelfT), ").checked_abs(), Some(5));
assert_eq!(", stringify!($SelfT), "::MIN.checked_abs(), None);",
$EndFeature, "
```"),
            #[stable(feature = "no_panic_abs", since = "1.13.0")]
            #[inline]
            pub fn checked_abs(self) -> Option<Self> { loop { } }
        }

        doc_comment! {
            concat!("Checked exponentiation. Computes `self.pow(exp)`, returning `None` if
overflow occurred.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(8", stringify!($SelfT), ".checked_pow(2), Some(64));
assert_eq!(", stringify!($SelfT), "::max_value().checked_pow(2), None);",
$EndFeature, "
```"),

            #[stable(feature = "no_panic_pow", since = "1.34.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn checked_pow(self, exp: u32) -> Option<Self> { loop { } }
        }

        doc_comment! {
            concat!("Saturating integer addition. Computes `self + rhs`, saturating at the numeric
bounds instead of overflowing.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(100", stringify!($SelfT), ".saturating_add(1), 101);
assert_eq!(", stringify!($SelfT), "::max_value().saturating_add(100), ", stringify!($SelfT),
"::max_value());
assert_eq!(", stringify!($SelfT), "::min_value().saturating_add(-1), ", stringify!($SelfT),
"::min_value());",
$EndFeature, "
```"),

            #[stable(feature = "rust1", since = "1.0.0")]
            #[rustc_const_unstable(feature = "const_saturating_int_methods")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub const fn saturating_add(self, rhs: Self) -> Self {
                intrinsics::saturating_add(self, rhs)
            }
        }

        doc_comment! {
            concat!("Saturating integer subtraction. Computes `self - rhs`, saturating at the
numeric bounds instead of overflowing.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(100", stringify!($SelfT), ".saturating_sub(127), -27);
assert_eq!(", stringify!($SelfT), "::min_value().saturating_sub(100), ", stringify!($SelfT),
"::min_value());
assert_eq!(", stringify!($SelfT), "::max_value().saturating_sub(-1), ", stringify!($SelfT),
"::max_value());",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[rustc_const_unstable(feature = "const_saturating_int_methods")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub const fn saturating_sub(self, rhs: Self) -> Self {
                intrinsics::saturating_sub(self, rhs)
            }
        }

        doc_comment! {
            concat!("Saturating integer negation. Computes `-self`, returning `MAX` if `self == MIN`
instead of overflowing.

# Examples

Basic usage:

```
", $Feature, "#![feature(saturating_neg)]
assert_eq!(100", stringify!($SelfT), ".saturating_neg(), -100);
assert_eq!((-100", stringify!($SelfT), ").saturating_neg(), 100);
assert_eq!(", stringify!($SelfT), "::min_value().saturating_neg(), ", stringify!($SelfT),
"::max_value());
assert_eq!(", stringify!($SelfT), "::max_value().saturating_neg(), ", stringify!($SelfT),
"::min_value() + 1);",
$EndFeature, "
```"),

            #[unstable(feature = "saturating_neg", issue = "59983")]
            #[inline]
            pub fn saturating_neg(self) -> Self { loop { } }
        }

        doc_comment! {
            concat!("Saturating absolute value. Computes `self.abs()`, returning `MAX` if `self ==
MIN` instead of overflowing.

# Examples

Basic usage:

```
", $Feature, "#![feature(saturating_neg)]
assert_eq!(100", stringify!($SelfT), ".saturating_abs(), 100);
assert_eq!((-100", stringify!($SelfT), ").saturating_abs(), 100);
assert_eq!(", stringify!($SelfT), "::min_value().saturating_abs(), ", stringify!($SelfT),
"::max_value());
assert_eq!((", stringify!($SelfT), "::min_value() + 1).saturating_abs(), ", stringify!($SelfT),
"::max_value());",
$EndFeature, "
```"),

            #[unstable(feature = "saturating_neg", issue = "59983")]
            #[inline]
            pub fn saturating_abs(self) -> Self { loop { } }
        }

        doc_comment! {
            concat!("Saturating integer multiplication. Computes `self * rhs`, saturating at the
numeric bounds instead of overflowing.

# Examples

Basic usage:

```
", $Feature, "use std::", stringify!($SelfT), ";

assert_eq!(10", stringify!($SelfT), ".saturating_mul(12), 120);
assert_eq!(", stringify!($SelfT), "::MAX.saturating_mul(10), ", stringify!($SelfT), "::MAX);
assert_eq!(", stringify!($SelfT), "::MIN.saturating_mul(10), ", stringify!($SelfT), "::MIN);",
$EndFeature, "
```"),
            #[stable(feature = "wrapping", since = "1.7.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn saturating_mul(self, rhs: Self) -> Self { loop { } }
        }

        doc_comment! {
            concat!("Saturating integer exponentiation. Computes `self.pow(exp)`,
saturating at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
", $Feature, "use std::", stringify!($SelfT), ";

assert_eq!((-4", stringify!($SelfT), ").saturating_pow(3), -64);
assert_eq!(", stringify!($SelfT), "::MIN.saturating_pow(2), ", stringify!($SelfT), "::MAX);
assert_eq!(", stringify!($SelfT), "::MIN.saturating_pow(3), ", stringify!($SelfT), "::MIN);",
$EndFeature, "
```"),
            #[stable(feature = "no_panic_pow", since = "1.34.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn saturating_pow(self, exp: u32) -> Self { loop { } }
        }

        doc_comment! {
            concat!("Wrapping (modular) addition. Computes `self + rhs`, wrapping around at the
boundary of the type.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(100", stringify!($SelfT), ".wrapping_add(27), 127);
assert_eq!(", stringify!($SelfT), "::max_value().wrapping_add(2), ", stringify!($SelfT),
"::min_value() + 1);",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub const fn wrapping_add(self, rhs: Self) -> Self { self }
        }

        doc_comment! {
            concat!("Wrapping (modular) subtraction. Computes `self - rhs`, wrapping around at the
boundary of the type.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(0", stringify!($SelfT), ".wrapping_sub(127), -127);
assert_eq!((-2", stringify!($SelfT), ").wrapping_sub(", stringify!($SelfT), "::max_value()), ",
stringify!($SelfT), "::max_value());",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub const fn wrapping_sub(self, rhs: Self) -> Self { self }
        }

        doc_comment! {
            concat!("Wrapping (modular) multiplication. Computes `self * rhs`, wrapping around at
the boundary of the type.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(10", stringify!($SelfT), ".wrapping_mul(12), 120);
assert_eq!(11i8.wrapping_mul(12), -124);",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub const fn wrapping_mul(self, rhs: Self) -> Self { self }
        }

        doc_comment! {
            concat!("Wrapping (modular) division. Computes `self / rhs`, wrapping around at the
boundary of the type.

The only case where such wrapping can occur is when one divides `MIN / -1` on a signed type (where
`MIN` is the negative minimal value for the type); this is equivalent to `-MIN`, a positive value
that is too large to represent in the type. In such a case, this function returns `MIN` itself.

# Panics

This function will panic if `rhs` is 0.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(100", stringify!($SelfT), ".wrapping_div(10), 10);
assert_eq!((-128i8).wrapping_div(-1), -128);",
$EndFeature, "
```"),
            #[stable(feature = "num_wrapping", since = "1.2.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn wrapping_div(self, rhs: Self) -> Self { loop { } }
        }

        doc_comment! {
            concat!("Wrapping Euclidean division. Computes `self.div_euclid(rhs)`,
wrapping around at the boundary of the type.

Wrapping will only occur in `MIN / -1` on a signed type (where `MIN` is the negative minimal value
for the type). This is equivalent to `-MIN`, a positive value that is too large to represent in the
type. In this case, this method returns `MIN` itself.

# Panics

This function will panic if `rhs` is 0.

# Examples

Basic usage:

```
assert_eq!(100", stringify!($SelfT), ".wrapping_div_euclid(10), 10);
assert_eq!((-128i8).wrapping_div_euclid(-1), -128);
```"),
            #[stable(feature = "euclidean_division", since = "1.38.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn wrapping_div_euclid(self, rhs: Self) -> Self { loop { } }
        }

        doc_comment! {
            concat!("Wrapping (modular) remainder. Computes `self % rhs`, wrapping around at the
boundary of the type.

Such wrap-around never actually occurs mathematically; implementation artifacts make `x % y`
invalid for `MIN / -1` on a signed type (where `MIN` is the negative minimal value). In such a case,
this function returns `0`.

# Panics

This function will panic if `rhs` is 0.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(100", stringify!($SelfT), ".wrapping_rem(10), 0);
assert_eq!((-128i8).wrapping_rem(-1), 0);",
$EndFeature, "
```"),
            #[stable(feature = "num_wrapping", since = "1.2.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn wrapping_rem(self, rhs: Self) -> Self { loop { } }
        }

        doc_comment! {
            concat!("Wrapping Euclidean remainder. Computes `self.rem_euclid(rhs)`, wrapping around
at the boundary of the type.

Wrapping will only occur in `MIN % -1` on a signed type (where `MIN` is the negative minimal value
for the type). In this case, this method returns 0.

# Panics

This function will panic if `rhs` is 0.

# Examples

Basic usage:

```
assert_eq!(100", stringify!($SelfT), ".wrapping_rem_euclid(10), 0);
assert_eq!((-128i8).wrapping_rem_euclid(-1), 0);
```"),
            #[stable(feature = "euclidean_division", since = "1.38.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn wrapping_rem_euclid(self, rhs: Self) -> Self { loop { } }
        }

        doc_comment! {
            concat!("Wrapping (modular) negation. Computes `-self`, wrapping around at the boundary
of the type.

The only case where such wrapping can occur is when one negates `MIN` on a signed type (where `MIN`
is the negative minimal value for the type); this is a positive value that is too large to represent
in the type. In such a case, this function returns `MIN` itself.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(100", stringify!($SelfT), ".wrapping_neg(), -100);
assert_eq!(", stringify!($SelfT), "::min_value().wrapping_neg(), ", stringify!($SelfT),
"::min_value());",
$EndFeature, "
```"),
            #[stable(feature = "num_wrapping", since = "1.2.0")]
            #[inline]
            pub const fn wrapping_neg(self) -> Self {
                self.overflowing_neg().0
            }
        }

        doc_comment! {
            concat!("Panic-free bitwise shift-left; yields `self << mask(rhs)`, where `mask` removes
any high-order bits of `rhs` that would cause the shift to exceed the bitwidth of the type.

Note that this is *not* the same as a rotate-left; the RHS of a wrapping shift-left is restricted to
the range of the type, rather than the bits shifted out of the LHS being returned to the other end.
The primitive integer types all implement a `rotate_left` function, which may be what you want
instead.

# Examples

Basic usage:

```
", $Feature, "assert_eq!((-1", stringify!($SelfT), ").wrapping_shl(7), -128);
assert_eq!((-1", stringify!($SelfT), ").wrapping_shl(128), -1);",
$EndFeature, "
```"),
            #[stable(feature = "num_wrapping", since = "1.2.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub const fn wrapping_shl(self, rhs: u32) -> Self {
                unsafe {
                    intrinsics::unchecked_shl(self, (rhs & ($BITS - 1)) as $SelfT)
                }
            }
        }

        doc_comment! {
            concat!("Panic-free bitwise shift-right; yields `self >> mask(rhs)`, where `mask`
removes any high-order bits of `rhs` that would cause the shift to exceed the bitwidth of the type.

Note that this is *not* the same as a rotate-right; the RHS of a wrapping shift-right is restricted
to the range of the type, rather than the bits shifted out of the LHS being returned to the other
end. The primitive integer types all implement a `rotate_right` function, which may be what you want
instead.

# Examples

Basic usage:

```
", $Feature, "assert_eq!((-128", stringify!($SelfT), ").wrapping_shr(7), -1);
assert_eq!((-128i16).wrapping_shr(64), -128);",
$EndFeature, "
```"),
            #[stable(feature = "num_wrapping", since = "1.2.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub const fn wrapping_shr(self, rhs: u32) -> Self {
                unsafe {
                    intrinsics::unchecked_shr(self, (rhs & ($BITS - 1)) as $SelfT)
                }
            }
        }

        doc_comment! {
            concat!("Wrapping (modular) absolute value. Computes `self.abs()`, wrapping around at
the boundary of the type.

The only case where such wrapping can occur is when one takes the absolute value of the negative
minimal value for the type this is a positive value that is too large to represent in the type. In
such a case, this function returns `MIN` itself.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(100", stringify!($SelfT), ".wrapping_abs(), 100);
assert_eq!((-100", stringify!($SelfT), ").wrapping_abs(), 100);
assert_eq!(", stringify!($SelfT), "::min_value().wrapping_abs(), ", stringify!($SelfT),
"::min_value());
assert_eq!((-128i8).wrapping_abs() as u8, 128);",
$EndFeature, "
```"),
            #[stable(feature = "no_panic_abs", since = "1.13.0")]
            #[inline]
            pub const fn wrapping_abs(self) -> Self {
                self
            }
        }

        doc_comment! {
            concat!("Wrapping (modular) exponentiation. Computes `self.pow(exp)`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(3", stringify!($SelfT), ".wrapping_pow(4), 81);
assert_eq!(3i8.wrapping_pow(5), -13);
assert_eq!(3i8.wrapping_pow(6), -39);",
$EndFeature, "
```"),
            #[stable(feature = "no_panic_pow", since = "1.34.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn wrapping_pow(self, exp: u32) -> Self { loop { } }
        }

        doc_comment! {
            concat!("Calculates `self` + `rhs`

Returns a tuple of the addition along with a boolean indicating whether an arithmetic overflow would
occur. If an overflow would have occurred then the wrapped value is returned.

# Examples

Basic usage:

```
", $Feature, "use std::", stringify!($SelfT), ";

assert_eq!(5", stringify!($SelfT), ".overflowing_add(2), (7, false));
assert_eq!(", stringify!($SelfT), "::MAX.overflowing_add(1), (", stringify!($SelfT),
"::MIN, true));", $EndFeature, "
```"),
            #[stable(feature = "wrapping", since = "1.7.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
                let (a, b) = intrinsics::add_with_overflow(self as $ActualT, rhs as $ActualT);
                (a as Self, b)
            }
        }

        doc_comment! {
            concat!("Calculates `self` - `rhs`

Returns a tuple of the subtraction along with a boolean indicating whether an arithmetic overflow
would occur. If an overflow would have occurred then the wrapped value is returned.

# Examples

Basic usage:

```
", $Feature, "use std::", stringify!($SelfT), ";

assert_eq!(5", stringify!($SelfT), ".overflowing_sub(2), (3, false));
assert_eq!(", stringify!($SelfT), "::MIN.overflowing_sub(1), (", stringify!($SelfT),
"::MAX, true));", $EndFeature, "
```"),
            #[stable(feature = "wrapping", since = "1.7.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
                let (a, b) = intrinsics::sub_with_overflow(self as $ActualT, rhs as $ActualT);
                (a as Self, b)
            }
        }

        doc_comment! {
            concat!("Calculates the multiplication of `self` and `rhs`.

Returns a tuple of the multiplication along with a boolean indicating whether an arithmetic overflow
would occur. If an overflow would have occurred then the wrapped value is returned.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(5", stringify!($SelfT), ".overflowing_mul(2), (10, false));
assert_eq!(1_000_000_000i32.overflowing_mul(10), (1410065408, true));",
$EndFeature, "
```"),
            #[stable(feature = "wrapping", since = "1.7.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
                let (a, b) = intrinsics::mul_with_overflow(self as $ActualT, rhs as $ActualT);
                (a as Self, b)
            }
        }

        doc_comment! {
            concat!("Calculates the divisor when `self` is divided by `rhs`.

Returns a tuple of the divisor along with a boolean indicating whether an arithmetic overflow would
occur. If an overflow would occur then self is returned.

# Panics

This function will panic if `rhs` is 0.

# Examples

Basic usage:

```
", $Feature, "use std::", stringify!($SelfT), ";

assert_eq!(5", stringify!($SelfT), ".overflowing_div(2), (2, false));
assert_eq!(", stringify!($SelfT), "::MIN.overflowing_div(-1), (", stringify!($SelfT),
"::MIN, true));",
$EndFeature, "
```"),
            #[inline]
            #[stable(feature = "wrapping", since = "1.7.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub fn overflowing_div(self, rhs: Self) -> (Self, bool) { loop { } }
        }

        doc_comment! {
            concat!("Calculates the quotient of Euclidean division `self.div_euclid(rhs)`.

Returns a tuple of the divisor along with a boolean indicating whether an arithmetic overflow would
occur. If an overflow would occur then `self` is returned.

# Panics

This function will panic if `rhs` is 0.

# Examples

Basic usage:

```
use std::", stringify!($SelfT), ";

assert_eq!(5", stringify!($SelfT), ".overflowing_div_euclid(2), (2, false));
assert_eq!(", stringify!($SelfT), "::MIN.overflowing_div_euclid(-1), (", stringify!($SelfT),
"::MIN, true));
```"),
            #[inline]
            #[stable(feature = "euclidean_division", since = "1.38.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) { loop { } }
        }

        doc_comment! {
            concat!("Calculates the remainder when `self` is divided by `rhs`.

Returns a tuple of the remainder after dividing along with a boolean indicating whether an
arithmetic overflow would occur. If an overflow would occur then 0 is returned.

# Panics

This function will panic if `rhs` is 0.

# Examples

Basic usage:

```
", $Feature, "use std::", stringify!($SelfT), ";

assert_eq!(5", stringify!($SelfT), ".overflowing_rem(2), (1, false));
assert_eq!(", stringify!($SelfT), "::MIN.overflowing_rem(-1), (0, true));",
$EndFeature, "
```"),
            #[inline]
            #[stable(feature = "wrapping", since = "1.7.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub fn overflowing_rem(self, rhs: Self) -> (Self, bool) { loop { } }
        }


        doc_comment! {
            concat!("Overflowing Euclidean remainder. Calculates `self.rem_euclid(rhs)`.

Returns a tuple of the remainder after dividing along with a boolean indicating whether an
arithmetic overflow would occur. If an overflow would occur then 0 is returned.

# Panics

This function will panic if `rhs` is 0.

# Examples

Basic usage:

```
use std::", stringify!($SelfT), ";

assert_eq!(5", stringify!($SelfT), ".overflowing_rem_euclid(2), (1, false));
assert_eq!(", stringify!($SelfT), "::MIN.overflowing_rem_euclid(-1), (0, true));
```"),
            #[stable(feature = "euclidean_division", since = "1.38.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) { loop { } }
        }


        doc_comment! {
            concat!("Negates self, overflowing if this is equal to the minimum value.

Returns a tuple of the negated version of self along with a boolean indicating whether an overflow
happened. If `self` is the minimum value (e.g., `i32::MIN` for values of type `i32`), then the
minimum value will be returned again and `true` will be returned for an overflow happening.

# Examples

Basic usage:

```
", $Feature, "use std::", stringify!($SelfT), ";

assert_eq!(2", stringify!($SelfT), ".overflowing_neg(), (-2, false));
assert_eq!(", stringify!($SelfT), "::MIN.overflowing_neg(), (", stringify!($SelfT),
"::MIN, true));", $EndFeature, "
```"),
            #[inline]
            #[stable(feature = "wrapping", since = "1.7.0")]
            pub const fn overflowing_neg(self) -> (Self, bool) {
                (self, false)
            }
        }

        doc_comment! {
            concat!("Shifts self left by `rhs` bits.

Returns a tuple of the shifted version of self along with a boolean indicating whether the shift
value was larger than or equal to the number of bits. If the shift value is too large, then value is
masked (N-1) where N is the number of bits, and this value is then used to perform the shift.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(0x1", stringify!($SelfT),".overflowing_shl(4), (0x10, false));
assert_eq!(0x1i32.overflowing_shl(36), (0x10, true));",
$EndFeature, "
```"),
            #[stable(feature = "wrapping", since = "1.7.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
                (self, false)
            }
        }

        doc_comment! {
            concat!("Shifts self right by `rhs` bits.

Returns a tuple of the shifted version of self along with a boolean indicating whether the shift
value was larger than or equal to the number of bits. If the shift value is too large, then value is
masked (N-1) where N is the number of bits, and this value is then used to perform the shift.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(0x10", stringify!($SelfT), ".overflowing_shr(4), (0x1, false));
assert_eq!(0x10i32.overflowing_shr(36), (0x1, true));",
$EndFeature, "
```"),
            #[stable(feature = "wrapping", since = "1.7.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
                (self, false)
            }
        }

        doc_comment! {
            concat!("Computes the absolute value of `self`.

Returns a tuple of the absolute version of self along with a boolean indicating whether an overflow
happened. If self is the minimum value (e.g., ", stringify!($SelfT), "::MIN for values of type
 ", stringify!($SelfT), "), then the minimum value will be returned again and true will be returned
for an overflow happening.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(10", stringify!($SelfT), ".overflowing_abs(), (10, false));
assert_eq!((-10", stringify!($SelfT), ").overflowing_abs(), (10, false));
assert_eq!((", stringify!($SelfT), "::min_value()).overflowing_abs(), (", stringify!($SelfT),
"::min_value(), true));",
$EndFeature, "
```"),
            #[stable(feature = "no_panic_abs", since = "1.13.0")]
            #[inline]
            pub const fn overflowing_abs(self) -> (Self, bool) {
                (self, false)
            }
        }

        doc_comment! {
            concat!("Raises self to the power of `exp`, using exponentiation by squaring.

Returns a tuple of the exponentiation along with a bool indicating
whether an overflow happened.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(3", stringify!($SelfT), ".overflowing_pow(4), (81, false));
assert_eq!(3i8.overflowing_pow(5), (-13, true));",
$EndFeature, "
```"),
            #[stable(feature = "no_panic_pow", since = "1.34.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn overflowing_pow(self, exp: u32) -> (Self, bool) { loop { } }
        }

        doc_comment! {
            concat!("Raises self to the power of `exp`, using exponentiation by squaring.

# Examples

Basic usage:

```
", $Feature, "let x: ", stringify!($SelfT), " = 2; // or any other integer type

assert_eq!(x.pow(5), 32);",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            #[rustc_inherit_overflow_checks]
            pub fn pow(self, exp: u32) -> Self { loop { } }
        }

        doc_comment! {
            concat!("Calculates the quotient of Euclidean division of `self` by `rhs`.

This computes the integer `n` such that `self = n * rhs + self.rem_euclid(rhs)`,
with `0 <= self.rem_euclid(rhs) < rhs`.

In other words, the result is `self / rhs` rounded to the integer `n`
such that `self >= n * rhs`.
If `self > 0`, this is equal to round towards zero (the default in Rust);
if `self < 0`, this is equal to round towards +/- infinity.

# Panics

This function will panic if `rhs` is 0.

# Examples

Basic usage:

```
let a: ", stringify!($SelfT), " = 7; // or any other integer type
let b = 4;

assert_eq!(a.div_euclid(b), 1); // 7 >= 4 * 1
assert_eq!(a.div_euclid(-b), -1); // 7 >= -4 * -1
assert_eq!((-a).div_euclid(b), -2); // -7 >= 4 * -2
assert_eq!((-a).div_euclid(-b), 2); // -7 >= -4 * 2
```"),
            #[stable(feature = "euclidean_division", since = "1.38.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            #[rustc_inherit_overflow_checks]
            pub fn div_euclid(self, rhs: Self) -> Self { loop { } }
        }


        doc_comment! {
            concat!("Calculates the least nonnegative remainder of `self (mod rhs)`.

This is done as if by the Euclidean division algorithm -- given
`r = self.rem_euclid(rhs)`, `self = rhs * self.div_euclid(rhs) + r`, and
`0 <= r < abs(rhs)`.

# Panics

This function will panic if `rhs` is 0.

# Examples

Basic usage:

```
let a: ", stringify!($SelfT), " = 7; // or any other integer type
let b = 4;

assert_eq!(a.rem_euclid(b), 3);
assert_eq!((-a).rem_euclid(b), 1);
assert_eq!(a.rem_euclid(-b), 3);
assert_eq!((-a).rem_euclid(-b), 1);
```"),
            #[stable(feature = "euclidean_division", since = "1.38.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            #[rustc_inherit_overflow_checks]
            pub fn rem_euclid(self, rhs: Self) -> Self { loop { } }
        }

        doc_comment! {
            concat!("Computes the absolute value of `self`.

# Overflow behavior

The absolute value of `", stringify!($SelfT), "::min_value()` cannot be represented as an
`", stringify!($SelfT), "`, and attempting to calculate it will cause an overflow. This means that
code in debug mode will trigger a panic on this case and optimized code will return `",
stringify!($SelfT), "::min_value()` without a panic.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(10", stringify!($SelfT), ".abs(), 10);
assert_eq!((-10", stringify!($SelfT), ").abs(), 10);",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline]
            #[rustc_inherit_overflow_checks]
            pub const fn abs(self) -> Self {

                self
            }
        }

        doc_comment! {
            concat!("Returns a number representing sign of `self`.

 - `0` if the number is zero
 - `1` if the number is positive
 - `-1` if the number is negative

# Examples

Basic usage:

```
", $Feature, "assert_eq!(10", stringify!($SelfT), ".signum(), 1);
assert_eq!(0", stringify!($SelfT), ".signum(), 0);
assert_eq!((-10", stringify!($SelfT), ").signum(), -1);",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[rustc_const_unstable(feature = "const_int_sign")]
            #[inline]
            pub const fn signum(self) -> Self { self }
        }

        doc_comment! {
            concat!("Returns `true` if `self` is positive and `false` if the number is zero or
negative.

# Examples

Basic usage:

```
", $Feature, "assert!(10", stringify!($SelfT), ".is_positive());
assert!(!(-10", stringify!($SelfT), ").is_positive());",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline]
            pub const fn is_positive(self) -> bool { false }
        }

        doc_comment! {
            concat!("Returns `true` if `self` is negative and `false` if the number is zero or
positive.

# Examples

Basic usage:

```
", $Feature, "assert!((-10", stringify!($SelfT), ").is_negative());
assert!(!10", stringify!($SelfT), ".is_negative());",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline]
            pub const fn is_negative(self) -> bool { false }
        }

        doc_comment! {
            concat!("Return the memory representation of this integer as a byte array in
big-endian (network) byte order.
",
$to_xe_bytes_doc,
"
# Examples

```
let bytes = ", $swap_op, stringify!($SelfT), ".to_be_bytes();
assert_eq!(bytes, ", $be_bytes, ");
```"),
            #[stable(feature = "int_to_from_bytes", since = "1.32.0")]
            #[rustc_const_unstable(feature = "const_int_conversion")]
            #[inline]
            pub const fn to_be_bytes(self) -> [u8; mem::size_of::<Self>()] {
                self.to_be().to_ne_bytes()
            }
        }

doc_comment! {
            concat!("Return the memory representation of this integer as a byte array in
little-endian byte order.
",
$to_xe_bytes_doc,
"
# Examples

```
let bytes = ", $swap_op, stringify!($SelfT), ".to_le_bytes();
assert_eq!(bytes, ", $le_bytes, ");
```"),
            #[stable(feature = "int_to_from_bytes", since = "1.32.0")]
            #[rustc_const_unstable(feature = "const_int_conversion")]
            #[inline]
            pub const fn to_le_bytes(self) -> [u8; mem::size_of::<Self>()] {
                self.to_le().to_ne_bytes()
            }
        }

        doc_comment! {
            concat!("
Return the memory representation of this integer as a byte array in
native byte order.

As the target platform's native endianness is used, portable code
should use [`to_be_bytes`] or [`to_le_bytes`], as appropriate,
instead.
",
$to_xe_bytes_doc,
"
[`to_be_bytes`]: #method.to_be_bytes
[`to_le_bytes`]: #method.to_le_bytes

# Examples

```
let bytes = ", $swap_op, stringify!($SelfT), ".to_ne_bytes();
assert_eq!(
    bytes,
    if cfg!(target_endian = \"big\") {
        ", $be_bytes, "
    } else {
        ", $le_bytes, "
    }
);
```"),
            #[stable(feature = "int_to_from_bytes", since = "1.32.0")]
            #[rustc_const_unstable(feature = "const_int_conversion")]
            #[inline]
            pub const fn to_ne_bytes(self) -> [u8; mem::size_of::<Self>()] {
                unsafe { mem::transmute(self) }
            }
        }

doc_comment! {
            concat!("Create an integer value from its representation as a byte array in
big endian.
",
$from_xe_bytes_doc,
"
# Examples

```
let value = ", stringify!($SelfT), "::from_be_bytes(", $be_bytes, ");
assert_eq!(value, ", $swap_op, ");
```

When starting from a slice rather than an array, fallible conversion APIs can be used:

```
use std::convert::TryInto;

fn read_be_", stringify!($SelfT), "(input: &mut &[u8]) -> ", stringify!($SelfT), " {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<", stringify!($SelfT), ">());
    *input = rest;
    ", stringify!($SelfT), "::from_be_bytes(int_bytes.try_into().unwrap())
}
```"),
            #[stable(feature = "int_to_from_bytes", since = "1.32.0")]
            #[rustc_const_unstable(feature = "const_int_conversion")]
            #[inline]
            pub const fn from_be_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
                Self::from_be(Self::from_ne_bytes(bytes))
            }
        }

doc_comment! {
            concat!("
Create an integer value from its representation as a byte array in
little endian.
",
$from_xe_bytes_doc,
"
# Examples

```
let value = ", stringify!($SelfT), "::from_le_bytes(", $le_bytes, ");
assert_eq!(value, ", $swap_op, ");
```

When starting from a slice rather than an array, fallible conversion APIs can be used:

```
use std::convert::TryInto;

fn read_le_", stringify!($SelfT), "(input: &mut &[u8]) -> ", stringify!($SelfT), " {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<", stringify!($SelfT), ">());
    *input = rest;
    ", stringify!($SelfT), "::from_le_bytes(int_bytes.try_into().unwrap())
}
```"),
            #[stable(feature = "int_to_from_bytes", since = "1.32.0")]
            #[rustc_const_unstable(feature = "const_int_conversion")]
            #[inline]
            pub const fn from_le_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
                Self::from_le(Self::from_ne_bytes(bytes))
            }
        }

        doc_comment! {
            concat!("Create an integer value from its memory representation as a byte
array in native endianness.

As the target platform's native endianness is used, portable code
likely wants to use [`from_be_bytes`] or [`from_le_bytes`], as
appropriate instead.

[`from_be_bytes`]: #method.from_be_bytes
[`from_le_bytes`]: #method.from_le_bytes
",
$from_xe_bytes_doc,
"
# Examples

```
let value = ", stringify!($SelfT), "::from_ne_bytes(if cfg!(target_endian = \"big\") {
    ", $be_bytes, "
} else {
    ", $le_bytes, "
});
assert_eq!(value, ", $swap_op, ");
```

When starting from a slice rather than an array, fallible conversion APIs can be used:

```
use std::convert::TryInto;

fn read_ne_", stringify!($SelfT), "(input: &mut &[u8]) -> ", stringify!($SelfT), " {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<", stringify!($SelfT), ">());
    *input = rest;
    ", stringify!($SelfT), "::from_ne_bytes(int_bytes.try_into().unwrap())
}
```"),
            #[stable(feature = "int_to_from_bytes", since = "1.32.0")]
            #[rustc_const_unstable(feature = "const_int_conversion")]
            #[inline]
            pub const fn from_ne_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
                unsafe { mem::transmute(bytes) }
            }
        }
    }
}

#[lang = "i8"]
impl i8 {
    int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
        "[0x12]", "[0x12]", "", "" }
}

#[lang = "i16"]
impl i16 {
    int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
        "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
}

#[lang = "i32"]
impl i32 {
    int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
        "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
        "[0x12, 0x34, 0x56, 0x78]", "", "" }
}

#[lang = "i64"]
impl i64 {
    int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
         "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
         "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
         "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
}

#[lang = "i128"]
impl i128 {
    int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
        170141183460469231731687303715884105727, "", "", 16,
        "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
        "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
        "[0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, \
          0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
        "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
          0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
}

#[cfg(target_pointer_width = "16")]
#[lang = "isize"]
impl isize {
    int_impl! { isize, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234",
        "0x3412", "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]",
        usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
}

#[cfg(target_pointer_width = "32")]
#[lang = "isize"]
impl isize {
    int_impl! { isize, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
        "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
        "[0x12, 0x34, 0x56, 0x78]",
        usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
}

#[cfg(target_pointer_width = "64")]
#[lang = "isize"]
impl isize {
    int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "",
        12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
         "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
         "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
         usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
}

macro_rules! uint_impl {
    ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
        $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
        $reversed:expr, $le_bytes:expr, $be_bytes:expr,
        $to_xe_bytes_doc:expr, $from_xe_bytes_doc:expr) => {
        doc_comment! {
            concat!("Returns the smallest value that can be represented by this integer type.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(", stringify!($SelfT), "::min_value(), 0);", $EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[rustc_promotable]
            #[inline(always)]
            pub const fn min_value() -> Self { 0 }
        }

        doc_comment! {
            concat!("Returns the largest value that can be represented by this integer type.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(", stringify!($SelfT), "::max_value(), ",
stringify!($MaxV), ");", $EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[rustc_promotable]
            #[inline(always)]
            pub const fn max_value() -> Self { !0 }
        }

        doc_comment! {
            concat!("Converts a string slice in a given base to an integer.

The string is expected to be an optional `+` sign
followed by digits.
Leading and trailing whitespace represent an error.
Digits are a subset of these characters, depending on `radix`:

* `0-9`
* `a-z`
* `A-Z`

# Panics

This function panics if `radix` is not in the range from 2 to 36.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(", stringify!($SelfT), "::from_str_radix(\"A\", 16), Ok(10));",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> { loop { } }
        }

        doc_comment! {
            concat!("Returns the number of ones in the binary representation of `self`.

# Examples

Basic usage:

```
", $Feature, "let n = 0b01001100", stringify!($SelfT), ";

assert_eq!(n.count_ones(), 3);", $EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline]
            pub const fn count_ones(self) -> u32 {
                intrinsics::ctpop(self as $ActualT) as u32
            }
        }

        doc_comment! {
            concat!("Returns the number of zeros in the binary representation of `self`.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(", stringify!($SelfT), "::max_value().count_zeros(), 0);", $EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline]
            pub const fn count_zeros(self) -> u32 {
                (!self).count_ones()
            }
        }

        doc_comment! {
            concat!("Returns the number of leading zeros in the binary representation of `self`.

# Examples

Basic usage:

```
", $Feature, "let n = ", stringify!($SelfT), "::max_value() >> 2;

assert_eq!(n.leading_zeros(), 2);", $EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline]
            pub const fn leading_zeros(self) -> u32 {
                intrinsics::ctlz(self as $ActualT) as u32
            }
        }

        doc_comment! {
            concat!("Returns the number of trailing zeros in the binary representation
of `self`.

# Examples

Basic usage:

```
", $Feature, "let n = 0b0101000", stringify!($SelfT), ";

assert_eq!(n.trailing_zeros(), 3);", $EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline]
            pub const fn trailing_zeros(self) -> u32 {
                intrinsics::cttz(self) as u32
            }
        }

        doc_comment! {
            concat!("Shifts the bits to the left by a specified amount, `n`,
wrapping the truncated bits to the end of the resulting integer.

Please note this isn't the same operation as the `<<` shifting operator!

# Examples

Basic usage:

```
let n = ", $rot_op, stringify!($SelfT), ";
let m = ", $rot_result, ";

assert_eq!(n.rotate_left(", $rot, "), m);
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub const fn rotate_left(self, n: u32) -> Self {
                intrinsics::rotate_left(self, n as $SelfT)
            }
        }

        doc_comment! {
            concat!("Shifts the bits to the right by a specified amount, `n`,
wrapping the truncated bits to the beginning of the resulting
integer.

Please note this isn't the same operation as the `>>` shifting operator!

# Examples

Basic usage:

```
let n = ", $rot_result, stringify!($SelfT), ";
let m = ", $rot_op, ";

assert_eq!(n.rotate_right(", $rot, "), m);
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub const fn rotate_right(self, n: u32) -> Self {
                intrinsics::rotate_right(self, n as $SelfT)
            }
        }

        doc_comment! {
            concat!("
Reverses the byte order of the integer.

# Examples

Basic usage:

```
let n = ", $swap_op, stringify!($SelfT), ";
let m = n.swap_bytes();

assert_eq!(m, ", $swapped, ");
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline]
            pub const fn swap_bytes(self) -> Self {
                intrinsics::bswap(self as $ActualT) as Self
            }
        }

        doc_comment! {
            concat!("Reverses the bit pattern of the integer.

# Examples

Basic usage:

```
let n = ", $swap_op, stringify!($SelfT), ";
let m = n.reverse_bits();

assert_eq!(m, ", $reversed, ");
```"),
            #[stable(feature = "reverse_bits", since = "1.37.0")]
            #[inline]
            #[must_use]
            pub const fn reverse_bits(self) -> Self {
                intrinsics::bitreverse(self as $ActualT) as Self
            }
        }

        doc_comment! {
            concat!("Converts an integer from big endian to the target's endianness.

On big endian this is a no-op. On little endian the bytes are
swapped.

# Examples

Basic usage:

```
", $Feature, "let n = 0x1A", stringify!($SelfT), ";

if cfg!(target_endian = \"big\") {
    assert_eq!(", stringify!($SelfT), "::from_be(n), n)
} else {
    assert_eq!(", stringify!($SelfT), "::from_be(n), n.swap_bytes())
}", $EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline]
            pub const fn from_be(x: Self) -> Self {
                #[cfg(target_endian = "big")]
                {
                    x
                }
                #[cfg(not(target_endian = "big"))]
                {
                    x.swap_bytes()
                }
            }
        }

        doc_comment! {
            concat!("Converts an integer from little endian to the target's endianness.

On little endian this is a no-op. On big endian the bytes are
swapped.

# Examples

Basic usage:

```
", $Feature, "let n = 0x1A", stringify!($SelfT), ";

if cfg!(target_endian = \"little\") {
    assert_eq!(", stringify!($SelfT), "::from_le(n), n)
} else {
    assert_eq!(", stringify!($SelfT), "::from_le(n), n.swap_bytes())
}", $EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline]
            pub const fn from_le(x: Self) -> Self {
                #[cfg(target_endian = "little")]
                {
                    x
                }
                #[cfg(not(target_endian = "little"))]
                {
                    x.swap_bytes()
                }
            }
        }

        doc_comment! {
            concat!("Converts `self` to big endian from the target's endianness.

On big endian this is a no-op. On little endian the bytes are
swapped.

# Examples

Basic usage:

```
", $Feature, "let n = 0x1A", stringify!($SelfT), ";

if cfg!(target_endian = \"big\") {
    assert_eq!(n.to_be(), n)
} else {
    assert_eq!(n.to_be(), n.swap_bytes())
}", $EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline]
            pub const fn to_be(self) -> Self { // or not to be?
                #[cfg(target_endian = "big")]
                {
                    self
                }
                #[cfg(not(target_endian = "big"))]
                {
                    self.swap_bytes()
                }
            }
        }

        doc_comment! {
            concat!("Converts `self` to little endian from the target's endianness.

On little endian this is a no-op. On big endian the bytes are
swapped.

# Examples

Basic usage:

```
", $Feature, "let n = 0x1A", stringify!($SelfT), ";

if cfg!(target_endian = \"little\") {
    assert_eq!(n.to_le(), n)
} else {
    assert_eq!(n.to_le(), n.swap_bytes())
}", $EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline]
            pub const fn to_le(self) -> Self {
                #[cfg(target_endian = "little")]
                {
                    self
                }
                #[cfg(not(target_endian = "little"))]
                {
                    self.swap_bytes()
                }
            }
        }

        doc_comment! {
            concat!("Checked integer addition. Computes `self + rhs`, returning `None`
if overflow occurred.

# Examples

Basic usage:

```
", $Feature, "assert_eq!((", stringify!($SelfT), "::max_value() - 2).checked_add(1), ",
"Some(", stringify!($SelfT), "::max_value() - 1));
assert_eq!((", stringify!($SelfT), "::max_value() - 2).checked_add(3), None);", $EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn checked_add(self, rhs: Self) -> Option<Self> { loop { } }
        }

        doc_comment! {
            concat!("Checked integer subtraction. Computes `self - rhs`, returning
`None` if overflow occurred.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(1", stringify!($SelfT), ".checked_sub(1), Some(0));
assert_eq!(0", stringify!($SelfT), ".checked_sub(1), None);", $EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn checked_sub(self, rhs: Self) -> Option<Self> { loop { } }
        }

        doc_comment! {
            concat!("Checked integer multiplication. Computes `self * rhs`, returning
`None` if overflow occurred.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(5", stringify!($SelfT), ".checked_mul(1), Some(5));
assert_eq!(", stringify!($SelfT), "::max_value().checked_mul(2), None);", $EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn checked_mul(self, rhs: Self) -> Option<Self> { loop { } }
        }

        doc_comment! {
            concat!("Checked integer division. Computes `self / rhs`, returning `None`
if `rhs == 0`.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(128", stringify!($SelfT), ".checked_div(2), Some(64));
assert_eq!(1", stringify!($SelfT), ".checked_div(0), None);", $EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn checked_div(self, rhs: Self) -> Option<Self> { loop { } }
        }

        doc_comment! {
            concat!("Checked Euclidean division. Computes `self.div_euclid(rhs)`, returning `None`
if `rhs == 0`.

# Examples

Basic usage:

```
assert_eq!(128", stringify!($SelfT), ".checked_div_euclid(2), Some(64));
assert_eq!(1", stringify!($SelfT), ".checked_div_euclid(0), None);
```"),
            #[stable(feature = "euclidean_division", since = "1.38.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn checked_div_euclid(self, rhs: Self) -> Option<Self> { loop { } }
        }


        doc_comment! {
            concat!("Checked integer remainder. Computes `self % rhs`, returning `None`
if `rhs == 0`.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(5", stringify!($SelfT), ".checked_rem(2), Some(1));
assert_eq!(5", stringify!($SelfT), ".checked_rem(0), None);", $EndFeature, "
```"),
            #[stable(feature = "wrapping", since = "1.7.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn checked_rem(self, rhs: Self) -> Option<Self> { loop { } }
        }

        doc_comment! {
            concat!("Checked Euclidean modulo. Computes `self.rem_euclid(rhs)`, returning `None`
if `rhs == 0`.

# Examples

Basic usage:

```
assert_eq!(5", stringify!($SelfT), ".checked_rem_euclid(2), Some(1));
assert_eq!(5", stringify!($SelfT), ".checked_rem_euclid(0), None);
```"),
            #[stable(feature = "euclidean_division", since = "1.38.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn checked_rem_euclid(self, rhs: Self) -> Option<Self> { loop { } }
        }

        doc_comment! {
            concat!("Checked negation. Computes `-self`, returning `None` unless `self ==
0`.

Note that negating any positive integer will overflow.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(0", stringify!($SelfT), ".checked_neg(), Some(0));
assert_eq!(1", stringify!($SelfT), ".checked_neg(), None);", $EndFeature, "
```"),
            #[stable(feature = "wrapping", since = "1.7.0")]
            #[inline]
            pub fn checked_neg(self) -> Option<Self> { loop { } }
        }

        doc_comment! {
            concat!("Checked shift left. Computes `self << rhs`, returning `None`
if `rhs` is larger than or equal to the number of bits in `self`.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(0x1", stringify!($SelfT), ".checked_shl(4), Some(0x10));
assert_eq!(0x10", stringify!($SelfT), ".checked_shl(129), None);", $EndFeature, "
```"),
            #[stable(feature = "wrapping", since = "1.7.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn checked_shl(self, rhs: u32) -> Option<Self> { loop { } }
        }

        doc_comment! {
            concat!("Checked shift right. Computes `self >> rhs`, returning `None`
if `rhs` is larger than or equal to the number of bits in `self`.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(0x10", stringify!($SelfT), ".checked_shr(4), Some(0x1));
assert_eq!(0x10", stringify!($SelfT), ".checked_shr(129), None);", $EndFeature, "
```"),
            #[stable(feature = "wrapping", since = "1.7.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn checked_shr(self, rhs: u32) -> Option<Self> { loop { } }
        }

        doc_comment! {
            concat!("Checked exponentiation. Computes `self.pow(exp)`, returning `None` if
overflow occurred.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(2", stringify!($SelfT), ".checked_pow(5), Some(32));
assert_eq!(", stringify!($SelfT), "::max_value().checked_pow(2), None);", $EndFeature, "
```"),
            #[stable(feature = "no_panic_pow", since = "1.34.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn checked_pow(self, exp: u32) -> Option<Self> { loop { } }
        }

        doc_comment! {
            concat!("Saturating integer addition. Computes `self + rhs`, saturating at
the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(100", stringify!($SelfT), ".saturating_add(1), 101);
assert_eq!(200u8.saturating_add(127), 255);", $EndFeature, "
```"),

            #[stable(feature = "rust1", since = "1.0.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[rustc_const_unstable(feature = "const_saturating_int_methods")]
            #[inline]
            pub const fn saturating_add(self, rhs: Self) -> Self {
                intrinsics::saturating_add(self, rhs)
            }
        }

        doc_comment! {
            concat!("Saturating integer subtraction. Computes `self - rhs`, saturating
at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(100", stringify!($SelfT), ".saturating_sub(27), 73);
assert_eq!(13", stringify!($SelfT), ".saturating_sub(127), 0);", $EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[rustc_const_unstable(feature = "const_saturating_int_methods")]
            #[inline]
            pub const fn saturating_sub(self, rhs: Self) -> Self {
                intrinsics::saturating_sub(self, rhs)
            }
        }

        doc_comment! {
            concat!("Saturating integer multiplication. Computes `self * rhs`,
saturating at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
", $Feature, "use std::", stringify!($SelfT), ";

assert_eq!(2", stringify!($SelfT), ".saturating_mul(10), 20);
assert_eq!((", stringify!($SelfT), "::MAX).saturating_mul(10), ", stringify!($SelfT),
"::MAX);", $EndFeature, "
```"),
            #[stable(feature = "wrapping", since = "1.7.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn saturating_mul(self, rhs: Self) -> Self { loop { } }
        }

        doc_comment! {
            concat!("Saturating integer exponentiation. Computes `self.pow(exp)`,
saturating at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
", $Feature, "use std::", stringify!($SelfT), ";

assert_eq!(4", stringify!($SelfT), ".saturating_pow(3), 64);
assert_eq!(", stringify!($SelfT), "::MAX.saturating_pow(2), ", stringify!($SelfT), "::MAX);",
$EndFeature, "
```"),
            #[stable(feature = "no_panic_pow", since = "1.34.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn saturating_pow(self, exp: u32) -> Self { loop { } }
        }

        doc_comment! {
            concat!("Wrapping (modular) addition. Computes `self + rhs`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(200", stringify!($SelfT), ".wrapping_add(55), 255);
assert_eq!(200", stringify!($SelfT), ".wrapping_add(", stringify!($SelfT), "::max_value()), 199);",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub const fn wrapping_add(self, rhs: Self) -> Self {
                intrinsics::wrapping_add(self, rhs)
            }
        }

        doc_comment! {
            concat!("Wrapping (modular) subtraction. Computes `self - rhs`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(100", stringify!($SelfT), ".wrapping_sub(100), 0);
assert_eq!(100", stringify!($SelfT), ".wrapping_sub(", stringify!($SelfT), "::max_value()), 101);",
$EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub const fn wrapping_sub(self, rhs: Self) -> Self {
                intrinsics::wrapping_sub(self, rhs)
            }
        }

        #[stable(feature = "rust1", since = "1.0.0")]
        #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
        #[inline]
        pub const fn wrapping_mul(self, rhs: Self) -> Self {
            intrinsics::wrapping_mul(self, rhs)
        }

        doc_comment! {
            concat!("Wrapping (modular) division. Computes `self / rhs`.
Wrapped division on unsigned types is just normal division.
There's no way wrapping could ever happen.
This function exists, so that all operations
are accounted for in the wrapping operations.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(100", stringify!($SelfT), ".wrapping_div(10), 10);", $EndFeature, "
```"),
            #[stable(feature = "num_wrapping", since = "1.2.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn wrapping_div(self, rhs: Self) -> Self { loop { } }
        }

        doc_comment! {
            concat!("Wrapping Euclidean division. Computes `self.div_euclid(rhs)`.
Wrapped division on unsigned types is just normal division.
There's no way wrapping could ever happen.
This function exists, so that all operations
are accounted for in the wrapping operations.
Since, for the positive integers, all common
definitions of division are equal, this
is exactly equal to `self.wrapping_div(rhs)`.

# Examples

Basic usage:

```
assert_eq!(100", stringify!($SelfT), ".wrapping_div_euclid(10), 10);
```"),
            #[stable(feature = "euclidean_division", since = "1.38.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn wrapping_div_euclid(self, rhs: Self) -> Self { loop { } }
        }

        doc_comment! {
            concat!("Wrapping (modular) remainder. Computes `self % rhs`.
Wrapped remainder calculation on unsigned types is
just the regular remainder calculation.
There's no way wrapping could ever happen.
This function exists, so that all operations
are accounted for in the wrapping operations.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(100", stringify!($SelfT), ".wrapping_rem(10), 0);", $EndFeature, "
```"),
            #[stable(feature = "num_wrapping", since = "1.2.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn wrapping_rem(self, rhs: Self) -> Self { loop { } }
        }

        doc_comment! {
            concat!("Wrapping Euclidean modulo. Computes `self.rem_euclid(rhs)`.
Wrapped modulo calculation on unsigned types is
just the regular remainder calculation.
There's no way wrapping could ever happen.
This function exists, so that all operations
are accounted for in the wrapping operations.
Since, for the positive integers, all common
definitions of division are equal, this
is exactly equal to `self.wrapping_rem(rhs)`.

# Examples

Basic usage:

```
assert_eq!(100", stringify!($SelfT), ".wrapping_rem_euclid(10), 0);
```"),
            #[stable(feature = "euclidean_division", since = "1.38.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn wrapping_rem_euclid(self, rhs: Self) -> Self { loop { } }
        }

        #[stable(feature = "num_wrapping", since = "1.2.0")]
        #[inline]
        pub const fn wrapping_neg(self) -> Self {
            self.overflowing_neg().0
        }

        doc_comment! {
            concat!("Panic-free bitwise shift-left; yields `self << mask(rhs)`,
where `mask` removes any high-order bits of `rhs` that
would cause the shift to exceed the bitwidth of the type.

Note that this is *not* the same as a rotate-left; the
RHS of a wrapping shift-left is restricted to the range
of the type, rather than the bits shifted out of the LHS
being returned to the other end. The primitive integer
types all implement a `rotate_left` function, which may
be what you want instead.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(1", stringify!($SelfT), ".wrapping_shl(7), 128);
assert_eq!(1", stringify!($SelfT), ".wrapping_shl(128), 1);", $EndFeature, "
```"),
            #[stable(feature = "num_wrapping", since = "1.2.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub const fn wrapping_shl(self, rhs: u32) -> Self {
                unsafe {
                    intrinsics::unchecked_shl(self, (rhs & ($BITS - 1)) as $SelfT)
                }
            }
        }

        doc_comment! {
            concat!("Panic-free bitwise shift-right; yields `self >> mask(rhs)`,
where `mask` removes any high-order bits of `rhs` that
would cause the shift to exceed the bitwidth of the type.

Note that this is *not* the same as a rotate-right; the
RHS of a wrapping shift-right is restricted to the range
of the type, rather than the bits shifted out of the LHS
being returned to the other end. The primitive integer
types all implement a `rotate_right` function, which may
be what you want instead.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(128", stringify!($SelfT), ".wrapping_shr(7), 1);
assert_eq!(128", stringify!($SelfT), ".wrapping_shr(128), 128);", $EndFeature, "
```"),
            #[stable(feature = "num_wrapping", since = "1.2.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub const fn wrapping_shr(self, rhs: u32) -> Self {
                unsafe {
                    intrinsics::unchecked_shr(self, (rhs & ($BITS - 1)) as $SelfT)
                }
            }
        }

        doc_comment! {
            concat!("Wrapping (modular) exponentiation. Computes `self.pow(exp)`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(3", stringify!($SelfT), ".wrapping_pow(5), 243);
assert_eq!(3u8.wrapping_pow(6), 217);", $EndFeature, "
```"),
            #[stable(feature = "no_panic_pow", since = "1.34.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn wrapping_pow(self, exp: u32) -> Self { loop { } }
        }

        doc_comment! {
            concat!("Calculates `self` + `rhs`

Returns a tuple of the addition along with a boolean indicating
whether an arithmetic overflow would occur. If an overflow would
have occurred then the wrapped value is returned.

# Examples

Basic usage

```
", $Feature, "use std::", stringify!($SelfT), ";

assert_eq!(5", stringify!($SelfT), ".overflowing_add(2), (7, false));
assert_eq!(", stringify!($SelfT), "::MAX.overflowing_add(1), (0, true));", $EndFeature, "
```"),
            #[stable(feature = "wrapping", since = "1.7.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
                let (a, b) = intrinsics::add_with_overflow(self as $ActualT, rhs as $ActualT);
                (a as Self, b)
            }
        }

        doc_comment! {
            concat!("Calculates `self` - `rhs`

Returns a tuple of the subtraction along with a boolean indicating
whether an arithmetic overflow would occur. If an overflow would
have occurred then the wrapped value is returned.

# Examples

Basic usage

```
", $Feature, "use std::", stringify!($SelfT), ";

assert_eq!(5", stringify!($SelfT), ".overflowing_sub(2), (3, false));
assert_eq!(0", stringify!($SelfT), ".overflowing_sub(1), (", stringify!($SelfT), "::MAX, true));",
$EndFeature, "
```"),
            #[stable(feature = "wrapping", since = "1.7.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
                let (a, b) = intrinsics::sub_with_overflow(self as $ActualT, rhs as $ActualT);
                (a as Self, b)
            }
        }

        #[stable(feature = "wrapping", since = "1.7.0")]
        #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
        #[inline]
        pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
            let (a, b) = intrinsics::mul_with_overflow(self as $ActualT, rhs as $ActualT);
            (a as Self, b)
        }

        doc_comment! {
            concat!("Calculates the divisor when `self` is divided by `rhs`.

Returns a tuple of the divisor along with a boolean indicating
whether an arithmetic overflow would occur. Note that for unsigned
integers overflow never occurs, so the second value is always
`false`.

# Panics

This function will panic if `rhs` is 0.

# Examples

Basic usage

```
", $Feature, "assert_eq!(5", stringify!($SelfT), ".overflowing_div(2), (2, false));", $EndFeature, "
```"),
            #[inline]
            #[stable(feature = "wrapping", since = "1.7.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub fn overflowing_div(self, rhs: Self) -> (Self, bool) { loop { } }
        }

        doc_comment! {
            concat!("Calculates the quotient of Euclidean division `self.div_euclid(rhs)`.

Returns a tuple of the divisor along with a boolean indicating
whether an arithmetic overflow would occur. Note that for unsigned
integers overflow never occurs, so the second value is always
`false`.
Since, for the positive integers, all common
definitions of division are equal, this
is exactly equal to `self.overflowing_div(rhs)`.

# Panics

This function will panic if `rhs` is 0.

# Examples

Basic usage

```
assert_eq!(5", stringify!($SelfT), ".overflowing_div_euclid(2), (2, false));
```"),
            #[inline]
            #[stable(feature = "euclidean_division", since = "1.38.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) { loop { } }
        }

        doc_comment! {
            concat!("Calculates the remainder when `self` is divided by `rhs`.

Returns a tuple of the remainder after dividing along with a boolean
indicating whether an arithmetic overflow would occur. Note that for
unsigned integers overflow never occurs, so the second value is
always `false`.

# Panics

This function will panic if `rhs` is 0.

# Examples

Basic usage

```
", $Feature, "assert_eq!(5", stringify!($SelfT), ".overflowing_rem(2), (1, false));", $EndFeature, "
```"),
            #[inline]
            #[stable(feature = "wrapping", since = "1.7.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub fn overflowing_rem(self, rhs: Self) -> (Self, bool) { loop { } }
        }

        doc_comment! {
            concat!("Calculates the remainder `self.rem_euclid(rhs)` as if by Euclidean division.

Returns a tuple of the modulo after dividing along with a boolean
indicating whether an arithmetic overflow would occur. Note that for
unsigned integers overflow never occurs, so the second value is
always `false`.
Since, for the positive integers, all common
definitions of division are equal, this operation
is exactly equal to `self.overflowing_rem(rhs)`.

# Panics

This function will panic if `rhs` is 0.

# Examples

Basic usage

```
assert_eq!(5", stringify!($SelfT), ".overflowing_rem_euclid(2), (1, false));
```"),
            #[inline]
            #[stable(feature = "euclidean_division", since = "1.38.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) { loop { } }
        }

        doc_comment! {
            concat!("Negates self in an overflowing fashion.

Returns `!self + 1` using wrapping operations to return the value
that represents the negation of this unsigned value. Note that for
positive unsigned values overflow always occurs, but negating 0 does
not overflow.

# Examples

Basic usage

```
", $Feature, "assert_eq!(0", stringify!($SelfT), ".overflowing_neg(), (0, false));
assert_eq!(2", stringify!($SelfT), ".overflowing_neg(), (-2i32 as ", stringify!($SelfT),
", true));", $EndFeature, "
```"),
            #[inline]
            #[stable(feature = "wrapping", since = "1.7.0")]
            pub const fn overflowing_neg(self) -> (Self, bool) {
                (self, false)
            }
        }

        doc_comment! {
            concat!("Shifts self left by `rhs` bits.

Returns a tuple of the shifted version of self along with a boolean
indicating whether the shift value was larger than or equal to the
number of bits. If the shift value is too large, then value is
masked (N-1) where N is the number of bits, and this value is then
used to perform the shift.

# Examples

Basic usage

```
", $Feature, "assert_eq!(0x1", stringify!($SelfT), ".overflowing_shl(4), (0x10, false));
assert_eq!(0x1", stringify!($SelfT), ".overflowing_shl(132), (0x10, true));", $EndFeature, "
```"),
            #[stable(feature = "wrapping", since = "1.7.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
                (self, false)
            }
        }

        doc_comment! {
            concat!("Shifts self right by `rhs` bits.

Returns a tuple of the shifted version of self along with a boolean
indicating whether the shift value was larger than or equal to the
number of bits. If the shift value is too large, then value is
masked (N-1) where N is the number of bits, and this value is then
used to perform the shift.

# Examples

Basic usage

```
", $Feature, "assert_eq!(0x10", stringify!($SelfT), ".overflowing_shr(4), (0x1, false));
assert_eq!(0x10", stringify!($SelfT), ".overflowing_shr(132), (0x1, true));", $EndFeature, "
```"),
            #[stable(feature = "wrapping", since = "1.7.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
                (self, false)
            }
        }

        doc_comment! {
            concat!("Raises self to the power of `exp`, using exponentiation by squaring.

Returns a tuple of the exponentiation along with a bool indicating
whether an overflow happened.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(3", stringify!($SelfT), ".overflowing_pow(5), (243, false));
assert_eq!(3u8.overflowing_pow(6), (217, true));", $EndFeature, "
```"),
            #[stable(feature = "no_panic_pow", since = "1.34.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            pub fn overflowing_pow(self, exp: u32) -> (Self, bool) { loop { } }
        }

        doc_comment! {
            concat!("Raises self to the power of `exp`, using exponentiation by squaring.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(2", stringify!($SelfT), ".pow(5), 32);", $EndFeature, "
```"),
        #[stable(feature = "rust1", since = "1.0.0")]
        #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
        #[inline]
        #[rustc_inherit_overflow_checks]
            pub fn pow(self, exp: u32) -> Self { loop { } }
    }

            doc_comment! {
            concat!("Performs Euclidean division.

Since, for the positive integers, all common
definitions of division are equal, this
is exactly equal to `self / rhs`.

# Examples

Basic usage:

```
assert_eq!(7", stringify!($SelfT), ".div_euclid(4), 1); // or any other integer type
```"),
            #[stable(feature = "euclidean_division", since = "1.38.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            #[rustc_inherit_overflow_checks]
                pub fn div_euclid(self, rhs: Self) -> Self { loop { } }
        }


        doc_comment! {
            concat!("Calculates the least remainder of `self (mod rhs)`.

Since, for the positive integers, all common
definitions of division are equal, this
is exactly equal to `self % rhs`.

# Examples

Basic usage:

```
assert_eq!(7", stringify!($SelfT), ".rem_euclid(4), 3); // or any other integer type
```"),
            #[stable(feature = "euclidean_division", since = "1.38.0")]
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            #[inline]
            #[rustc_inherit_overflow_checks]
            pub fn rem_euclid(self, rhs: Self) -> Self { loop { } }
        }

        doc_comment! {
            concat!("Returns `true` if and only if `self == 2^k` for some `k`.

# Examples

Basic usage:

```
", $Feature, "assert!(16", stringify!($SelfT), ".is_power_of_two());
assert!(!10", stringify!($SelfT), ".is_power_of_two());", $EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline]
            pub fn is_power_of_two(self) -> bool { loop { } }
        }

        #[inline]
        fn one_less_than_next_power_of_two(self) -> Self { loop { } }

        doc_comment! {
            concat!("Returns the smallest power of two greater than or equal to `self`.

When return value overflows (i.e., `self > (1 << (N-1))` for type
`uN`), it panics in debug mode and return value is wrapped to 0 in
release mode (the only situation in which method can return 0).

# Examples

Basic usage:

```
", $Feature, "assert_eq!(2", stringify!($SelfT), ".next_power_of_two(), 2);
assert_eq!(3", stringify!($SelfT), ".next_power_of_two(), 4);", $EndFeature, "
```"),
            #[stable(feature = "rust1", since = "1.0.0")]
            #[inline]
            pub fn next_power_of_two(self) -> Self { loop { } }
        }

        doc_comment! {
            concat!("Returns the smallest power of two greater than or equal to `n`. If
the next power of two is greater than the type's maximum value,
`None` is returned, otherwise the power of two is wrapped in `Some`.

# Examples

Basic usage:

```
", $Feature, "assert_eq!(2", stringify!($SelfT),
".checked_next_power_of_two(), Some(2));
assert_eq!(3", stringify!($SelfT), ".checked_next_power_of_two(), Some(4));
assert_eq!(", stringify!($SelfT), "::max_value().checked_next_power_of_two(), None);",
$EndFeature, "
```"),
            #[inline]
            #[stable(feature = "rust1", since = "1.0.0")]
            pub fn checked_next_power_of_two(self) -> Option<Self> { loop { } }
        }

        doc_comment! {
            concat!("Returns the smallest power of two greater than or equal to `n`. If
the next power of two is greater than the type's maximum value,
the return value is wrapped to `0`.

# Examples

Basic usage:

```
#![feature(wrapping_next_power_of_two)]
", $Feature, "
assert_eq!(2", stringify!($SelfT), ".wrapping_next_power_of_two(), 2);
assert_eq!(3", stringify!($SelfT), ".wrapping_next_power_of_two(), 4);
assert_eq!(", stringify!($SelfT), "::max_value().wrapping_next_power_of_two(), 0);",
$EndFeature, "
```"),
            #[unstable(feature = "wrapping_next_power_of_two", issue = "32463",
                       reason = "needs decision on wrapping behaviour")]
            pub fn wrapping_next_power_of_two(self) -> Self { loop { } }
        }

        doc_comment! {
            concat!("Return the memory representation of this integer as a byte array in
big-endian (network) byte order.
",
$to_xe_bytes_doc,
"
# Examples

```
let bytes = ", $swap_op, stringify!($SelfT), ".to_be_bytes();
assert_eq!(bytes, ", $be_bytes, ");
```"),
            #[stable(feature = "int_to_from_bytes", since = "1.32.0")]
            #[rustc_const_unstable(feature = "const_int_conversion")]
            #[inline]
            pub const fn to_be_bytes(self) -> [u8; mem::size_of::<Self>()] {
                self.to_be().to_ne_bytes()
            }
        }

        doc_comment! {
            concat!("Return the memory representation of this integer as a byte array in
little-endian byte order.
",
$to_xe_bytes_doc,
"
# Examples

```
let bytes = ", $swap_op, stringify!($SelfT), ".to_le_bytes();
assert_eq!(bytes, ", $le_bytes, ");
```"),
            #[stable(feature = "int_to_from_bytes", since = "1.32.0")]
            #[rustc_const_unstable(feature = "const_int_conversion")]
            #[inline]
            pub const fn to_le_bytes(self) -> [u8; mem::size_of::<Self>()] {
                self.to_le().to_ne_bytes()
            }
        }

        doc_comment! {
            concat!("
Return the memory representation of this integer as a byte array in
native byte order.

As the target platform's native endianness is used, portable code
should use [`to_be_bytes`] or [`to_le_bytes`], as appropriate,
instead.
",
$to_xe_bytes_doc,
"
[`to_be_bytes`]: #method.to_be_bytes
[`to_le_bytes`]: #method.to_le_bytes

# Examples

```
let bytes = ", $swap_op, stringify!($SelfT), ".to_ne_bytes();
assert_eq!(
    bytes,
    if cfg!(target_endian = \"big\") {
        ", $be_bytes, "
    } else {
        ", $le_bytes, "
    }
);
```"),
            #[stable(feature = "int_to_from_bytes", since = "1.32.0")]
            #[rustc_const_unstable(feature = "const_int_conversion")]
            #[inline]
            pub const fn to_ne_bytes(self) -> [u8; mem::size_of::<Self>()] {
                unsafe { mem::transmute(self) }
            }
        }

        doc_comment! {
            concat!("Create an integer value from its representation as a byte array in
big endian.
",
$from_xe_bytes_doc,
"
# Examples

```
let value = ", stringify!($SelfT), "::from_be_bytes(", $be_bytes, ");
assert_eq!(value, ", $swap_op, ");
```

When starting from a slice rather than an array, fallible conversion APIs can be used:

```
use std::convert::TryInto;

fn read_be_", stringify!($SelfT), "(input: &mut &[u8]) -> ", stringify!($SelfT), " {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<", stringify!($SelfT), ">());
    *input = rest;
    ", stringify!($SelfT), "::from_be_bytes(int_bytes.try_into().unwrap())
}
```"),
            #[stable(feature = "int_to_from_bytes", since = "1.32.0")]
            #[rustc_const_unstable(feature = "const_int_conversion")]
            #[inline]
            pub const fn from_be_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
                Self::from_be(Self::from_ne_bytes(bytes))
            }
        }

        doc_comment! {
            concat!("
Create an integer value from its representation as a byte array in
little endian.
",
$from_xe_bytes_doc,
"
# Examples

```
let value = ", stringify!($SelfT), "::from_le_bytes(", $le_bytes, ");
assert_eq!(value, ", $swap_op, ");
```

When starting from a slice rather than an array, fallible conversion APIs can be used:

```
use std::convert::TryInto;

fn read_le_", stringify!($SelfT), "(input: &mut &[u8]) -> ", stringify!($SelfT), " {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<", stringify!($SelfT), ">());
    *input = rest;
    ", stringify!($SelfT), "::from_le_bytes(int_bytes.try_into().unwrap())
}
```"),
            #[stable(feature = "int_to_from_bytes", since = "1.32.0")]
            #[rustc_const_unstable(feature = "const_int_conversion")]
            #[inline]
            pub const fn from_le_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
                Self::from_le(Self::from_ne_bytes(bytes))
            }
        }

        doc_comment! {
            concat!("Create an integer value from its memory representation as a byte
array in native endianness.

As the target platform's native endianness is used, portable code
likely wants to use [`from_be_bytes`] or [`from_le_bytes`], as
appropriate instead.

[`from_be_bytes`]: #method.from_be_bytes
[`from_le_bytes`]: #method.from_le_bytes
",
$from_xe_bytes_doc,
"
# Examples

```
let value = ", stringify!($SelfT), "::from_ne_bytes(if cfg!(target_endian = \"big\") {
    ", $be_bytes, "
} else {
    ", $le_bytes, "
});
assert_eq!(value, ", $swap_op, ");
```

When starting from a slice rather than an array, fallible conversion APIs can be used:

```
use std::convert::TryInto;

fn read_ne_", stringify!($SelfT), "(input: &mut &[u8]) -> ", stringify!($SelfT), " {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<", stringify!($SelfT), ">());
    *input = rest;
    ", stringify!($SelfT), "::from_ne_bytes(int_bytes.try_into().unwrap())
}
```"),
            #[stable(feature = "int_to_from_bytes", since = "1.32.0")]
            #[rustc_const_unstable(feature = "const_int_conversion")]
            #[inline]
            pub const fn from_ne_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
                unsafe { mem::transmute(bytes) }
            }
        }
    }
}

#[lang = "u8"]
impl u8 {
    uint_impl! { u8, u8, 8, 255, "", "", 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
        "[0x12]", "", "" }


    #[stable(feature = "ascii_methods_on_intrinsics", since = "1.23.0")]
    #[inline]
    pub fn is_ascii(&self) -> bool { loop { } }

    #[stable(feature = "ascii_methods_on_intrinsics", since = "1.23.0")]
    #[inline]
    pub fn to_ascii_uppercase(&self) -> u8 { loop { } }

    #[stable(feature = "ascii_methods_on_intrinsics", since = "1.23.0")]
    #[inline]
    pub fn to_ascii_lowercase(&self) -> u8 { loop { } }

    #[stable(feature = "ascii_methods_on_intrinsics", since = "1.23.0")]
    #[inline]
    pub fn eq_ignore_ascii_case(&self, other: &u8) -> bool { loop { } }

    #[stable(feature = "ascii_methods_on_intrinsics", since = "1.23.0")]
    #[inline]
    pub fn make_ascii_uppercase(&mut self) { loop { } }

    #[stable(feature = "ascii_methods_on_intrinsics", since = "1.23.0")]
    #[inline]
    pub fn make_ascii_lowercase(&mut self) { loop { } }

    #[stable(feature = "ascii_ctype_on_intrinsics", since = "1.24.0")]
    #[inline]
    pub fn is_ascii_alphabetic(&self) -> bool { loop { } }

    #[stable(feature = "ascii_ctype_on_intrinsics", since = "1.24.0")]
    #[inline]
    pub fn is_ascii_uppercase(&self) -> bool { loop { } }

    #[stable(feature = "ascii_ctype_on_intrinsics", since = "1.24.0")]
    #[inline]
    pub fn is_ascii_lowercase(&self) -> bool { loop { } }

    #[stable(feature = "ascii_ctype_on_intrinsics", since = "1.24.0")]
    #[inline]
    pub fn is_ascii_alphanumeric(&self) -> bool { loop { } }

    #[stable(feature = "ascii_ctype_on_intrinsics", since = "1.24.0")]
    #[inline]
    pub fn is_ascii_digit(&self) -> bool { loop { } }

    #[stable(feature = "ascii_ctype_on_intrinsics", since = "1.24.0")]
    #[inline]
    pub fn is_ascii_hexdigit(&self) -> bool { loop { } }

    #[stable(feature = "ascii_ctype_on_intrinsics", since = "1.24.0")]
    #[inline]
    pub fn is_ascii_punctuation(&self) -> bool { loop { } }

    #[stable(feature = "ascii_ctype_on_intrinsics", since = "1.24.0")]
    #[inline]
    pub fn is_ascii_graphic(&self) -> bool { loop { } }

    #[stable(feature = "ascii_ctype_on_intrinsics", since = "1.24.0")]
    #[inline]
    pub fn is_ascii_whitespace(&self) -> bool { loop { } }

    #[stable(feature = "ascii_ctype_on_intrinsics", since = "1.24.0")]
    #[inline]
    pub fn is_ascii_control(&self) -> bool { loop { } }
}

#[lang = "u16"]
impl u16 {
    uint_impl! { u16, u16, 16, 65535, "", "", 4, "0xa003", "0x3a", "0x1234", "0x3412", "0x2c48",
        "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
}

#[lang = "u32"]
impl u32 {
    uint_impl! { u32, u32, 32, 4294967295, "", "", 8, "0x10000b3", "0xb301", "0x12345678",
        "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]", "[0x12, 0x34, 0x56, 0x78]", "", "" }
}

#[lang = "u64"]
impl u64 {
    uint_impl! { u64, u64, 64, 18446744073709551615, "", "", 12, "0xaa00000000006e1", "0x6e10aa",
        "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
        "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
        "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
        "", ""}
}

#[lang = "u128"]
impl u128 {
    uint_impl! { u128, u128, 128, 340282366920938463463374607431768211455, "", "", 16,
        "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
        "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
        "[0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, \
          0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
        "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
          0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]",
         "", ""}
}

#[cfg(target_pointer_width = "16")]
#[lang = "usize"]
impl usize {
    uint_impl! { usize, u16, 16, 65535, "", "", 4, "0xa003", "0x3a", "0x1234", "0x3412", "0x2c48",
        "[0x34, 0x12]", "[0x12, 0x34]",
        usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
}
#[cfg(target_pointer_width = "32")]
#[lang = "usize"]
impl usize {
    uint_impl! { usize, u32, 32, 4294967295, "", "", 8, "0x10000b3", "0xb301", "0x12345678",
        "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]", "[0x12, 0x34, 0x56, 0x78]",
        usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
}

#[cfg(target_pointer_width = "64")]
#[lang = "usize"]
impl usize {
    uint_impl! { usize, u64, 64, 18446744073709551615, "", "", 12, "0xaa00000000006e1", "0x6e10aa",
        "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
        "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
         "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
        usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
}

#[derive(Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
pub enum FpCategory {
    #[stable(feature = "rust1", since = "1.0.0")]
    Nan,

    #[stable(feature = "rust1", since = "1.0.0")]
    Infinite,

    #[stable(feature = "rust1", since = "1.0.0")]
    Zero,

    #[stable(feature = "rust1", since = "1.0.0")]
    Subnormal,

    #[stable(feature = "rust1", since = "1.0.0")]
    Normal,
}

#[stable(feature = "try_from", since = "1.34.0")]
#[derive(Debug)]
pub struct TryFromIntError(());

impl TryFromIntError {
    #[unstable(feature = "int_error_internals",
               reason = "available through Error trait and this method should \
                         not be exposed publicly",
               issue = "0")]
    #[doc(hidden)]
    pub fn __description(&self) -> &str { loop { } }
}

#[stable(feature = "try_from", since = "1.34.0")]
impl fmt::Display for TryFromIntError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[doc(hidden)]
trait FromStrRadixHelper: Sized {
    fn min_value() -> Self;
    fn max_value() -> Self;
    fn from_u32(u: u32) -> Self;
    fn checked_mul(&self, other: u32) -> Option<Self>;
    fn checked_sub(&self, other: u32) -> Option<Self>;
    fn checked_add(&self, other: u32) -> Option<Self>;
}

macro_rules! doit {
    ($($t:ty)*) => ($(impl FromStrRadixHelper for $t {
        #[inline]
        fn min_value() -> Self { loop { } }
        #[inline]
        fn max_value() -> Self { loop { } }
        #[inline]
        fn from_u32(u: u32) -> Self { loop { } }
        #[inline]
        fn checked_mul(&self, other: u32) -> Option<Self> { loop { } }
        #[inline]
        fn checked_sub(&self, other: u32) -> Option<Self> { loop { } }
        #[inline]
        fn checked_add(&self, other: u32) -> Option<Self> { loop { } }
    })*)
}
doit! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }


#[derive(Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct ParseIntError {
    kind: IntErrorKind,
}

#[unstable(feature = "int_error_matching",
           reason = "it can be useful to match errors when making error messages \
                     for integer parsing",
           issue = "22639")]
#[derive(Debug)]
#[non_exhaustive]
pub enum IntErrorKind {
    Empty,
    InvalidDigit,
    Overflow,
    Underflow,
    Zero,
}

impl ParseIntError {
    #[unstable(feature = "int_error_matching",
               reason = "it can be useful to match errors when making error messages \
                         for integer parsing",
               issue = "22639")]
    pub fn kind(&self) -> &IntErrorKind { loop { } }
    #[unstable(feature = "int_error_internals",
               reason = "available through Error trait and this method should \
                         not be exposed publicly",
               issue = "0")]
    #[doc(hidden)]
    pub fn __description(&self) -> &str { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl fmt::Display for ParseIntError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
pub use crate::num::dec2flt::ParseFloatError;
