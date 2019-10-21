#![stable(feature = "rust1", since = "1.0.0")]

use crate::fmt;
use crate::iter::{FromIterator, FusedIterator, TrustedLen};
use crate::ops::{self, Deref, DerefMut};

#[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[must_use = "this `Result` may be an `Err` variant, which should be handled"]
#[stable(feature = "rust1", since = "1.0.0")]
pub enum Result<T, E> {
    #[stable(feature = "rust1", since = "1.0.0")]
    Ok(#[stable(feature = "rust1", since = "1.0.0")] T),

    #[stable(feature = "rust1", since = "1.0.0")]
    Err(#[stable(feature = "rust1", since = "1.0.0")] E),
}


impl<T, E> Result<T, E> {

    #[must_use = "if you intended to assert that this is ok, consider `.unwrap()` instead"]
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn is_ok(&self) -> bool { loop { } }

    #[must_use = "if you intended to assert that this is err, consider `.unwrap_err()` instead"]
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn is_err(&self) -> bool { loop { } }

    #[must_use]
    #[inline]
    #[unstable(feature = "option_result_contains", issue = "62358")]
    pub fn contains<U>(&self, x: &U) -> bool where U: PartialEq<T> { loop { } }

    #[must_use]
    #[inline]
    #[unstable(feature = "result_contains_err", issue = "62358")]
    pub fn contains_err<F>(&self, f: &F) -> bool where F: PartialEq<E> { loop { } }


    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn ok(self) -> Option<T> { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn err(self) -> Option<E> { loop { } }


    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn as_ref(&self) -> Result<&T, &E> { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn as_mut(&mut self) -> Result<&mut T, &mut E> { loop { } }


    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn map<U, F: FnOnce(T) -> U>(self, op: F) -> Result<U,E> { loop { } }

    #[inline]
    #[unstable(feature = "result_map_or_else", issue = "53268")]
    pub fn map_or_else<U, M: FnOnce(T) -> U, F: FnOnce(E) -> U>(self, fallback: F, map: M) -> U { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn map_err<F, O: FnOnce(E) -> F>(self, op: O) -> Result<T,F> { loop { } }


    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn iter(&self) -> Iter<'_, T> { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> { loop { } }


    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn and<U>(self, res: Result<U, E>) -> Result<U, E> { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn and_then<U, F: FnOnce(T) -> Result<U, E>>(self, op: F) -> Result<U, E> { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn or<F>(self, res: Result<T, F>) -> Result<T, F> { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn or_else<F, O: FnOnce(E) -> Result<T, F>>(self, op: O) -> Result<T, F> { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn unwrap_or(self, optb: T) -> T { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn unwrap_or_else<F: FnOnce(E) -> T>(self, op: F) -> T { loop { } }
}

impl<T: Copy, E> Result<&T, E> {
    #[unstable(feature = "result_copied", reason = "newly added", issue = "63168")]
    pub fn copied(self) -> Result<T, E> { loop { } }
}

impl<T: Copy, E> Result<&mut T, E> {
    #[unstable(feature = "result_copied", reason = "newly added", issue = "63168")]
    pub fn copied(self) -> Result<T, E> { loop { } }
}

impl<T: Clone, E> Result<&T, E> {
    #[unstable(feature = "result_cloned", reason = "newly added", issue = "63168")]
    pub fn cloned(self) -> Result<T, E> { loop { } }
}

impl<T: Clone, E> Result<&mut T, E> {
    #[unstable(feature = "result_cloned", reason = "newly added", issue = "63168")]
    pub fn cloned(self) -> Result<T, E> { loop { } }
}


impl<T, E: fmt::Debug> Result<T, E> {
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn unwrap(self) -> T { loop { } }

    #[inline]
    #[stable(feature = "result_expect", since = "1.4.0")]
    pub fn expect(self, msg: &str) -> T { loop { } }
}

impl<T: fmt::Debug, E> Result<T, E> {
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn unwrap_err(self) -> E { loop { } }

    #[inline]
    #[stable(feature = "result_expect_err", since = "1.17.0")]
    pub fn expect_err(self, msg: &str) -> E { loop { } }
}

impl<T: Default, E> Result<T, E> {
    #[inline]
    #[stable(feature = "result_unwrap_or_default", since = "1.16.0")]
    pub fn unwrap_or_default(self) -> T { loop { } }
}

#[unstable(feature = "inner_deref", reason = "newly added", issue = "50264")]
impl<T: Deref, E> Result<T, E> {
    pub fn as_deref_ok(&self) -> Result<&T::Target, &E> { loop { } }
}

#[unstable(feature = "inner_deref", reason = "newly added", issue = "50264")]
impl<T, E: Deref> Result<T, E> {
    pub fn as_deref_err(&self) -> Result<&T, &E::Target>
    { loop { } }
}

#[unstable(feature = "inner_deref", reason = "newly added", issue = "50264")]
impl<T: Deref, E: Deref> Result<T, E> {
    pub fn as_deref(&self) -> Result<&T::Target, &E::Target>
    { loop { } }
}

#[unstable(feature = "inner_deref", reason = "newly added", issue = "50264")]
impl<T: DerefMut, E> Result<T, E> {
    pub fn as_deref_mut_ok(&mut self) -> Result<&mut T::Target, &mut E> { loop { } }
}

#[unstable(feature = "inner_deref", reason = "newly added", issue = "50264")]
impl<T, E: DerefMut> Result<T, E> {
    pub fn as_deref_mut_err(&mut self) -> Result<&mut T, &mut E::Target>
    { loop { } }
}

#[unstable(feature = "inner_deref", reason = "newly added", issue = "50264")]
impl<T: DerefMut, E: DerefMut> Result<T, E> {
    pub fn as_deref_mut(&mut self) -> Result<&mut T::Target, &mut E::Target>
    { loop { } }
}

impl<T, E> Result<Option<T>, E> {
    #[inline]
    #[stable(feature = "transpose_result", since = "1.33.0")]
    pub fn transpose(self) -> Option<Result<T, E>> { loop { } }
}

#[inline(never)]
#[cold]
fn unwrap_failed(msg: &str, error: &dyn fmt::Debug) -> ! { loop { } }


#[stable(feature = "rust1", since = "1.0.0")]
impl<T: Clone, E: Clone> Clone for Result<T, E> {
    #[inline]
    fn clone(&self) -> Self { loop { } }

    #[inline]
    fn clone_from(&mut self, source: &Self) { loop { } }
}


#[stable(feature = "rust1", since = "1.0.0")]
impl<T, E> IntoIterator for Result<T, E> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> IntoIter<T> { loop { } }
}

#[stable(since = "1.4.0", feature = "result_iter")]
impl<'a, T, E> IntoIterator for &'a Result<T, E> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> { loop { } }
}

#[stable(since = "1.4.0", feature = "result_iter")]
impl<'a, T, E> IntoIterator for &'a mut Result<T, E> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> { loop { } }
}


#[derive(Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Iter<'a, T: 'a> { inner: Option<&'a T> }

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> { loop { } }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a T> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> ExactSizeIterator for Iter<'_, T> {}

#[stable(feature = "fused", since = "1.26.0")]
impl<T> FusedIterator for Iter<'_, T> {}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<A> TrustedLen for Iter<'_, A> {}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> Clone for Iter<'_, T> {
    #[inline]
    fn clone(&self) -> Self { loop { } }
}

#[derive(Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct IterMut<'a, T: 'a> { inner: Option<&'a mut T> }

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<&'a mut T> { loop { } }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut T> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> ExactSizeIterator for IterMut<'_, T> {}

#[stable(feature = "fused", since = "1.26.0")]
impl<T> FusedIterator for IterMut<'_, T> {}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<A> TrustedLen for IterMut<'_, A> {}

#[derive(Clone, Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct IntoIter<T> { inner: Option<T> }

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> { loop { } }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> ExactSizeIterator for IntoIter<T> {}

#[stable(feature = "fused", since = "1.26.0")]
impl<T> FusedIterator for IntoIter<T> {}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<A> TrustedLen for IntoIter<A> {}


#[stable(feature = "rust1", since = "1.0.0")]
impl<A, E, V: FromIterator<A>> FromIterator<Result<A, E>> for Result<V, E> {
    #[inline]
    fn from_iter<I: IntoIterator<Item=Result<A, E>>>(iter: I) -> Result<V, E> { loop { } }
}

#[unstable(feature = "try_trait", issue = "42327")]
impl<T,E> ops::Try for Result<T, E> {
    type Ok = T;
    type Error = E;

    #[inline]
    fn into_result(self) -> Self { loop { } }

    #[inline]
    fn from_ok(v: T) -> Self { loop { } }

    #[inline]
    fn from_error(v: E) -> Self { loop { } }
}
