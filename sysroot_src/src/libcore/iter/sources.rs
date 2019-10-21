use crate::fmt;
use crate::marker;
use crate::usize;

use super::{FusedIterator, TrustedLen};

#[derive(Clone, Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Repeat<A> {
    element: A
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<A: Clone> Iterator for Repeat<A> {
    type Item = A;

    #[inline]
    fn next(&mut self) -> Option<A> { loop { } }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }
}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<A: Clone> TrustedLen for Repeat<A> {}

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
pub fn repeat<T: Clone>(elt: T) -> Repeat<T> { loop { } }

#[derive(Copy, Clone, Debug)]
#[stable(feature = "iterator_repeat_with", since = "1.28.0")]
pub struct RepeatWith<F> {
    repeater: F
}

#[stable(feature = "iterator_repeat_with", since = "1.28.0")]
impl<A, F: FnMut() -> A> Iterator for RepeatWith<F> {
    type Item = A;

    #[inline]
    fn next(&mut self) -> Option<A> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }
}

#[stable(feature = "iterator_repeat_with", since = "1.28.0")]
impl<A, F: FnMut() -> A> FusedIterator for RepeatWith<F> {}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<A, F: FnMut() -> A> TrustedLen for RepeatWith<F> {}

#[inline]
#[stable(feature = "iterator_repeat_with", since = "1.28.0")]
pub fn repeat_with<A, F: FnMut() -> A>(repeater: F) -> RepeatWith<F> { loop { } }

#[stable(feature = "iter_empty", since = "1.2.0")]
pub struct Empty<T>(marker::PhantomData<T>);

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<T> fmt::Debug for Empty<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "iter_empty", since = "1.2.0")]
impl<T> Iterator for Empty<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> { loop { } }

    fn size_hint(&self) -> (usize, Option<usize>){ loop { } }
}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<T> TrustedLen for Empty<T> {}

#[stable(feature = "iter_empty", since = "1.2.0")]
impl<T> Clone for Empty<T> {
    fn clone(&self) -> Empty<T> { loop { } }
}

#[stable(feature = "iter_empty", since = "1.2.0")]
impl<T> Default for Empty<T> {
    fn default() -> Empty<T> { loop { } }
}

#[stable(feature = "iter_empty", since = "1.2.0")]
pub const fn empty<T>() -> Empty<T> {
    Empty(marker::PhantomData)
}

#[derive(Clone, Debug)]
#[stable(feature = "iter_once", since = "1.2.0")]
pub struct Once<T> {
    inner: crate::option::IntoIter<T>
}

#[stable(feature = "iter_once", since = "1.2.0")]
impl<T> Iterator for Once<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> { loop { } }

    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }
}

#[stable(feature = "iter_once", since = "1.2.0")]
pub fn once<T>(value: T) -> Once<T> { loop { } }

#[derive(Copy, Clone, Debug)]
#[unstable(feature = "iter_once_with", issue = "57581")]
pub struct OnceWith<F> {
    gen: Option<F>,
}

#[unstable(feature = "iter_once_with", issue = "57581")]
impl<A, F: FnOnce() -> A> Iterator for OnceWith<F> {
    type Item = A;

    #[inline]
    fn next(&mut self) -> Option<A> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }
}

#[inline]
#[unstable(feature = "iter_once_with", issue = "57581")]
pub fn once_with<A, F: FnOnce() -> A>(gen: F) -> OnceWith<F> { loop { } }

#[inline]
#[stable(feature = "iter_from_fn", since = "1.34.0")]
pub fn from_fn<T, F>(f: F) -> FromFn<F>
    where F: FnMut() -> Option<T>
{ loop { } }

#[derive(Clone)]
#[stable(feature = "iter_from_fn", since = "1.34.0")]
pub struct FromFn<F>(F);

#[stable(feature = "iter_from_fn", since = "1.34.0")]
impl<T, F> Iterator for FromFn<F>
    where F: FnMut() -> Option<T>
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> { loop { } }
}

#[stable(feature = "iter_from_fn", since = "1.34.0")]
impl<F> fmt::Debug for FromFn<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "iter_successors", since = "1.34.0")]
pub fn successors<T, F>(first: Option<T>, succ: F) -> Successors<T, F>
    where F: FnMut(&T) -> Option<T>
{
    Successors {
        next: first,
        succ,
    }
}

#[derive(Clone)]
#[stable(feature = "iter_successors", since = "1.34.0")]
pub struct Successors<T, F> {
    next: Option<T>,
    succ: F,
}

#[stable(feature = "iter_successors", since = "1.34.0")]
impl<T, F> Iterator for Successors<T, F>
    where F: FnMut(&T) -> Option<T>
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }
}

#[stable(feature = "iter_successors", since = "1.34.0")]
impl<T, F> FusedIterator for Successors<T, F>
    where F: FnMut(&T) -> Option<T>
{}

#[stable(feature = "iter_successors", since = "1.34.0")]
impl<T: fmt::Debug, F> fmt::Debug for Successors<T, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}
