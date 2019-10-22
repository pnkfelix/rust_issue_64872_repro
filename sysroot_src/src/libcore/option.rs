#![stable(feature = "rust1", since = "1.0.0")]

// use crate::iter::{FromIterator};
use crate::{fmt, ops::{self, Deref, DerefMut}};


#[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[stable(feature = "rust1", since = "1.0.0")]
pub enum Option<T> {
    #[stable(feature = "rust1", since = "1.0.0")]
    None,
    #[stable(feature = "rust1", since = "1.0.0")]
    Some(#[stable(feature = "rust1", since = "1.0.0")] T),
}


impl<T> Option<T> {

    #[must_use = "if you intended to assert that this has a value, consider `.unwrap()` instead"]
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn is_some(&self) -> bool { loop { } }

    #[must_use = "if you intended to assert that this doesn't have a value, consider \
                  `.and_then(|| panic!(\"`Option` had a value when expected `None`\"))` instead"]
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn is_none(&self) -> bool { loop { } }

    #[must_use]
    #[inline]
    #[unstable(feature = "option_result_contains", issue = "62358")]
    pub fn contains<U>(&self, x: &U) -> bool where U: PartialEq<T> { loop { } }


    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn as_ref(&self) -> Option<&T> { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn as_mut(&mut self) -> Option<&mut T> { loop { } }



    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn expect(self, msg: &str) -> T { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn unwrap(self) -> T { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn unwrap_or(self, def: T) -> T { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T { loop { } }


    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Option<U> { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn map_or_else<U, D: FnOnce() -> U, F: FnOnce(T) -> U>(self, default: D, f: F) -> U { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn ok_or<E>(self, err: E) -> Result<T, E> { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn ok_or_else<E, F: FnOnce() -> E>(self, err: F) -> Result<T, E> { loop { } }


    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn iter(&self) -> Iter<'_, T> { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> { loop { } }


    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn and<U>(self, optb: Option<U>) -> Option<U> { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn and_then<U, F: FnOnce(T) -> Option<U>>(self, f: F) -> Option<U> { loop { } }

    #[inline]
    #[stable(feature = "option_filter", since = "1.27.0")]
    pub fn filter<P: FnOnce(&T) -> bool>(self, predicate: P) -> Self { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn or(self, optb: Option<T>) -> Option<T> { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn or_else<F: FnOnce() -> Option<T>>(self, f: F) -> Option<T> { loop { } }

    #[inline]
    #[stable(feature = "option_xor", since = "1.37.0")]
    pub fn xor(self, optb: Option<T>) -> Option<T> { loop { } }


    #[inline]
    #[stable(feature = "option_entry", since = "1.20.0")]
    pub fn get_or_insert(&mut self, v: T) -> &mut T { loop { } }

    #[inline]
    #[stable(feature = "option_entry", since = "1.20.0")]
    pub fn get_or_insert_with<F: FnOnce() -> T>(&mut self, f: F) -> &mut T { loop { } }


    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn take(&mut self) -> Option<T> { loop { } }

    #[inline]
    #[stable(feature = "option_replace", since = "1.31.0")]
    pub fn replace(&mut self, value: T) -> Option<T> { loop { } }
}

impl<T: Copy> Option<&T> {
    #[stable(feature = "copied", since = "1.35.0")]
    pub fn copied(self) -> Option<T> { loop { } }
}

impl<T: Copy> Option<&mut T> {
    #[stable(feature = "copied", since = "1.35.0")]
    pub fn copied(self) -> Option<T> { loop { } }
}

impl<T: Clone> Option<&T> {
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn cloned(self) -> Option<T> { loop { } }
}

impl<T: Clone> Option<&mut T> {
    #[stable(since = "1.26.0", feature = "option_ref_mut_cloned")]
    pub fn cloned(self) -> Option<T> { loop { } }
}

impl<T: fmt::Debug> Option<T> {
    #[inline]
    #[unstable(feature = "option_expect_none", reason = "newly added", issue = "62633")]
    pub fn expect_none(self, msg: &str) { loop { } }

    #[inline]
    #[unstable(feature = "option_unwrap_none", reason = "newly added", issue = "62633")]
    pub fn unwrap_none(self) { loop { } }
}

impl<T: Default> Option<T> {
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn unwrap_or_default(self) -> T { loop { } }
}

impl<T: Deref> Option<T> {
    #[stable(feature = "option_deref", since = "1.40.0")]
    pub fn as_deref(&self) -> Option<&T::Target> { loop { } }
}

impl<T: DerefMut> Option<T> {
    #[stable(feature = "option_deref", since = "1.40.0")]
    pub fn as_deref_mut(&mut self) -> Option<&mut T::Target> { loop { } }
}

impl<T, E> Option<Result<T, E>> {
    #[inline]
    #[stable(feature = "transpose_result", since = "1.33.0")]
    pub fn transpose(self) -> Result<Option<T>, E> { loop { } }
}

#[inline(never)]
#[cold]
fn expect_failed(msg: &str) -> ! { loop { } }

#[inline(never)]
#[cold]
fn expect_none_failed(msg: &str, value: &dyn fmt::Debug) -> ! { loop { } }


#[stable(feature = "rust1", since = "1.0.0")]
impl<T: Clone> Clone for Option<T> {
    #[inline]
    fn clone(&self) -> Self { loop { } }

    #[inline]
    fn clone_from(&mut self, source: &Self) { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> Default for Option<T> {
    #[inline]
    fn default() -> Option<T> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> IntoIterator for Option<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> IntoIter<T> { loop { } }
}

#[stable(since = "1.4.0", feature = "option_iter")]
impl<'a, T> IntoIterator for &'a Option<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> { loop { } }
}

#[stable(since = "1.4.0", feature = "option_iter")]
impl<'a, T> IntoIterator for &'a mut Option<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> { loop { } }
}

#[stable(since = "1.12.0", feature = "option_from")]
impl<T> From<T> for Option<T> {
    fn from(val: T) -> Option<T> { loop { } }
}

#[stable(feature = "option_ref_from_ref_option", since = "1.30.0")]
impl<'a, T> From<&'a Option<T>> for Option<&'a T> {
    fn from(o: &'a Option<T>) -> Option<&'a T> { loop { } }
}

#[stable(feature = "option_ref_from_ref_option", since = "1.30.0")]
impl<'a, T> From<&'a mut Option<T>> for Option<&'a mut T> {
    fn from(o: &'a mut Option<T>) -> Option<&'a mut T> { loop { } }
}


#[derive(Clone, Debug)]
struct Item<A> {
    opt: Option<A>
}

impl<A> Iterator for Item<A> {
    type Item = A;

    #[inline]
    fn next(&mut self) -> Option<A> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Debug)]
pub struct Iter<'a, A: 'a> { inner: Item<&'a A> }

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, A> Iterator for Iter<'a, A> {
    type Item = &'a A;

    #[inline]
    fn next(&mut self) -> Option<&'a A> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<A> Clone for Iter<'_, A> {
    #[inline]
    fn clone(&self) -> Self { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Debug)]
pub struct IterMut<'a, A: 'a> { inner: Item<&'a mut A> }

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, A> Iterator for IterMut<'a, A> {
    type Item = &'a mut A;

    #[inline]
    fn next(&mut self) -> Option<&'a mut A> { loop { } }
}

#[derive(Clone, Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct IntoIter<A> { inner: Item<A> }

#[stable(feature = "rust1", since = "1.0.0")]
impl<A> Iterator for IntoIter<A> {
    type Item = A;

    #[inline]
    fn next(&mut self) -> Option<A> { loop { } }
}

#[unstable(feature = "try_trait", issue = "42327")]
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct NoneError;

#[unstable(feature = "try_trait", issue = "42327")]
impl<T> ops::Try for Option<T> {
    type Ok = T;
    type Error = NoneError;

    #[inline]
    fn into_result(self) -> Result<T, NoneError> { loop { } }

    #[inline]
    fn from_ok(v: T) -> Self { loop { } }

    #[inline]
    fn from_error(_: NoneError) -> Self { loop { } }
}

impl<T> Option<Option<T>> {
    #[inline]
    #[unstable(feature = "option_flattening", issue = "60258")]
    pub fn flatten(self) -> Option<T> { loop { } }
}
