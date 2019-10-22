//! Implementations of things like `Eq` for fixed-length arrays

#![stable(feature = "core_array", since = "1.36.0")]

use crate::borrow::{Borrow, BorrowMut};
use crate::cmp::Ordering;
use crate::convert::{Infallible, TryFrom};
use crate::fmt;
use crate::marker::Unsize;
use crate::slice::{Iter, IterMut};

#[unstable(feature = "fixed_size_array", issue = "27778")]
pub unsafe trait FixedSizeArray<T> {
    #[unstable(feature = "fixed_size_array", issue = "27778")]
    fn as_slice(&self) -> &[T];
    #[unstable(feature = "fixed_size_array", issue = "27778")]
    fn as_mut_slice(&mut self) -> &mut [T];
}

#[unstable(feature = "fixed_size_array", issue = "27778")]
unsafe impl<T, A: Unsize<[T]>> FixedSizeArray<T> for A {
    #[inline]
    fn as_slice(&self) -> &[T] { loop { } }
    #[inline]
    fn as_mut_slice(&mut self) -> &mut [T] { loop { } }
}

#[stable(feature = "try_from", since = "1.34.0")]
#[derive(Debug, Copy, Clone)]
pub struct TryFromSliceError(());

#[stable(feature = "core_array", since = "1.36.0")]
impl fmt::Display for TryFromSliceError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

impl TryFromSliceError {
    #[unstable(feature = "array_error_internals",
           reason = "available through Error trait and this method should not \
                     be exposed publicly",
           issue = "0")]
    #[inline]
    #[doc(hidden)]
    pub fn __description(&self) -> &str { loop { } }
}

#[stable(feature = "try_from_slice_error", since = "1.36.0")]
impl From<Infallible> for TryFromSliceError {
    fn from(x: Infallible) -> TryFromSliceError { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T, const N: usize> AsRef<[T]> for [T; N]
where
    [T; N]: LengthAtMost32,
{
    #[inline]
    fn as_ref(&self) -> &[T] { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T, const N: usize> AsMut<[T]> for [T; N]
where
    [T; N]: LengthAtMost32,
{
    #[inline]
    fn as_mut(&mut self) -> &mut [T] { loop { } }
}

#[stable(feature = "array_borrow", since = "1.4.0")]
impl<T, const N: usize> Borrow<[T]> for [T; N]
where
    [T; N]: LengthAtMost32,
{
    fn borrow(&self) -> &[T] { loop { } }
}

#[stable(feature = "array_borrow", since = "1.4.0")]
impl<T, const N: usize> BorrowMut<[T]> for [T; N]
where
    [T; N]: LengthAtMost32,
{
    fn borrow_mut(&mut self) -> &mut [T] { loop { } }
}

#[stable(feature = "try_from", since = "1.34.0")]
impl<T, const N: usize> TryFrom<&[T]> for [T; N]
where
    T: Copy,
    [T; N]: LengthAtMost32,
{
    type Error = TryFromSliceError;

    fn try_from(slice: &[T]) -> Result<[T; N], TryFromSliceError> { loop { } }
}

#[stable(feature = "try_from", since = "1.34.0")]
impl<'a, T, const N: usize> TryFrom<&'a [T]> for &'a [T; N]
where
    [T; N]: LengthAtMost32,
{
    type Error = TryFromSliceError;

    fn try_from(slice: &[T]) -> Result<&[T; N], TryFromSliceError> { loop { } }
}

#[stable(feature = "try_from", since = "1.34.0")]
impl<'a, T, const N: usize> TryFrom<&'a mut [T]> for &'a mut [T; N]
where
    [T; N]: LengthAtMost32,
{
    type Error = TryFromSliceError;

    fn try_from(slice: &mut [T]) -> Result<&mut [T; N], TryFromSliceError> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: fmt::Debug, const N: usize> fmt::Debug for [T; N]
where
    [T; N]: LengthAtMost32,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T, const N: usize> IntoIterator for &'a [T; N]
where
    [T; N]: LengthAtMost32,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T, const N: usize> IntoIterator for &'a mut [T; N]
where
    [T; N]: LengthAtMost32,
{
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<A, B, const N: usize> PartialEq<[B; N]> for [A; N]
where
    A: PartialEq<B>,
    [A; N]: LengthAtMost32,
    [B; N]: LengthAtMost32,
{
    #[inline]
    fn eq(&self, other: &[B; N]) -> bool { loop { } }
    #[inline]
    fn ne(&self, other: &[B; N]) -> bool { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<A, B, const N: usize> PartialEq<[B]> for [A; N]
where
    A: PartialEq<B>,
    [A; N]: LengthAtMost32,
{
    #[inline]
    fn eq(&self, other: &[B]) -> bool { loop { } }
    #[inline]
    fn ne(&self, other: &[B]) -> bool { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<A, B, const N: usize> PartialEq<[A; N]> for [B]
where
    B: PartialEq<A>,
    [A; N]: LengthAtMost32,
{
    #[inline]
    fn eq(&self, other: &[A; N]) -> bool { loop { } }
    #[inline]
    fn ne(&self, other: &[A; N]) -> bool { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'b, A, B, const N: usize> PartialEq<&'b [B]> for [A; N]
where
    A: PartialEq<B>,
    [A; N]: LengthAtMost32,
{
    #[inline]
    fn eq(&self, other: &&'b [B]) -> bool { loop { } }
    #[inline]
    fn ne(&self, other: &&'b [B]) -> bool { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'b, A, B, const N: usize> PartialEq<[A; N]> for &'b [B]
where
    B: PartialEq<A>,
    [A; N]: LengthAtMost32,
{
    #[inline]
    fn eq(&self, other: &[A; N]) -> bool { loop { } }
    #[inline]
    fn ne(&self, other: &[A; N]) -> bool { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'b, A, B, const N: usize> PartialEq<&'b mut [B]> for [A; N]
where
    A: PartialEq<B>,
    [A; N]: LengthAtMost32,
{
    #[inline]
    fn eq(&self, other: &&'b mut [B]) -> bool { loop { } }
    #[inline]
    fn ne(&self, other: &&'b mut [B]) -> bool { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'b, A, B, const N: usize> PartialEq<[A; N]> for &'b mut [B]
where
    B: PartialEq<A>,
    [A; N]: LengthAtMost32,
{
    #[inline]
    fn eq(&self, other: &[A; N]) -> bool { loop { } }
    #[inline]
    fn ne(&self, other: &[A; N]) -> bool { loop { } }
}


#[stable(feature = "rust1", since = "1.0.0")]
impl<T: Eq, const N: usize> Eq for [T; N] where [T; N]: LengthAtMost32 {}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: PartialOrd, const N: usize> PartialOrd for [T; N]
where
    [T; N]: LengthAtMost32,
{
    #[inline]
    fn partial_cmp(&self, other: &[T; N]) -> Option<Ordering> { loop { } }
    #[inline]
    fn lt(&self, other: &[T; N]) -> bool { loop { } }
    #[inline]
    fn le(&self, other: &[T; N]) -> bool { loop { } }
    #[inline]
    fn ge(&self, other: &[T; N]) -> bool { loop { } }
    #[inline]
    fn gt(&self, other: &[T; N]) -> bool { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: Ord, const N: usize> Ord for [T; N]
where
    [T; N]: LengthAtMost32,
{
    #[inline]
    fn cmp(&self, other: &[T; N]) -> Ordering { loop { } }
}

#[rustc_on_unimplemented(
    message="arrays only have std trait implementations for lengths 0..=32",
)]
#[unstable(feature = "const_generic_impls_guard", issue = "0",
    reason = "will never be stable, just a temporary step until const generics are stable")]
pub trait LengthAtMost32 {}

macro_rules! array_impls {
    ($($N:literal)+) => {
        $(
            #[unstable(feature = "const_generic_impls_guard", issue = "0")]
            impl<T> LengthAtMost32 for [T; $N] {}
        )+
    }
}

array_impls! {
     0  1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
}


macro_rules! array_impl_default {
    {$n:expr, $t:ident $($ts:ident)*} => {
        #[stable(since = "1.4.0", feature = "array_default")]
        impl<T> Default for [T; $n] where T: Default {
            fn default() -> [T; $n] { loop { } }
        }
        array_impl_default!{($n - 1), $($ts)*}
    };
    {$n:expr,} => {
        #[stable(since = "1.4.0", feature = "array_default")]
        impl<T> Default for [T; $n] {
            fn default() -> [T; $n] { loop { } }
        }
    };
}

array_impl_default!{32, T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T}
