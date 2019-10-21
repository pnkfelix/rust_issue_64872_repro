use crate::fmt;
use crate::ops::{Try};
use crate::usize;

use super::{Iterator, DoubleEndedIterator, ExactSizeIterator, FusedIterator, TrustedLen};

mod chain;
mod flatten;
mod zip;

pub use self::chain::Chain;
#[stable(feature = "rust1", since = "1.0.0")]
pub use self::flatten::{FlatMap, Flatten};
pub use self::zip::Zip;
pub(crate) use self::zip::TrustedRandomAccess;

#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Rev<T> {
    iter: T
}
impl<T> Rev<T> {
    pub(super) fn new(iter: T) -> Rev<T> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> Iterator for Rev<I> where I: DoubleEndedIterator {
    type Item = <I as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<<I as Iterator>::Item> { loop { } }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<<I as Iterator>::Item> { loop { } }

    fn try_fold<B, F, R>(&mut self, init: B, f: F) -> R where
        Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Ok=B>
    { loop { } }

    fn fold<Acc, F>(self, init: Acc, f: F) -> Acc
        where F: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }

    #[inline]
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
        where P: FnMut(&Self::Item) -> bool
    { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> DoubleEndedIterator for Rev<I> where I: DoubleEndedIterator {
    #[inline]
    fn next_back(&mut self) -> Option<<I as Iterator>::Item> { loop { } }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<<I as Iterator>::Item> { loop { } }

    fn try_rfold<B, F, R>(&mut self, init: B, f: F) -> R where
        Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Ok=B>
    { loop { } }

    fn rfold<Acc, F>(self, init: Acc, f: F) -> Acc
        where F: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }

    fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
        where P: FnMut(&Self::Item) -> bool
    { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> ExactSizeIterator for Rev<I>
    where I: ExactSizeIterator + DoubleEndedIterator
{
    fn len(&self) -> usize { loop { } }

    fn is_empty(&self) -> bool { loop { } }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<I> FusedIterator for Rev<I>
    where I: FusedIterator + DoubleEndedIterator {}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<I> TrustedLen for Rev<I>
    where I: TrustedLen + DoubleEndedIterator {}

#[stable(feature = "iter_copied", since = "1.36.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Clone, Debug)]
pub struct Copied<I> {
    it: I,
}

impl<I> Copied<I> {
    pub(super) fn new(it: I) -> Copied<I> { loop { } }
}

#[stable(feature = "iter_copied", since = "1.36.0")]
impl<'a, I, T: 'a> Iterator for Copied<I>
    where I: Iterator<Item=&'a T>, T: Copy
{
    type Item = T;

    fn next(&mut self) -> Option<T> { loop { } }

    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    fn try_fold<B, F, R>(&mut self, init: B, f: F) -> R where
        Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Ok=B>
    { loop { } }

    fn fold<Acc, F>(self, init: Acc, f: F) -> Acc
        where F: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }
}

#[stable(feature = "iter_copied", since = "1.36.0")]
impl<'a, I, T: 'a> DoubleEndedIterator for Copied<I>
    where I: DoubleEndedIterator<Item=&'a T>, T: Copy
{
    fn next_back(&mut self) -> Option<T> { loop { } }

    fn try_rfold<B, F, R>(&mut self, init: B, f: F) -> R where
        Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Ok=B>
    { loop { } }

    fn rfold<Acc, F>(self, init: Acc, f: F) -> Acc
        where F: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }
}

#[stable(feature = "iter_copied", since = "1.36.0")]
impl<'a, I, T: 'a> ExactSizeIterator for Copied<I>
    where I: ExactSizeIterator<Item=&'a T>, T: Copy
{
    fn len(&self) -> usize { loop { } }

    fn is_empty(&self) -> bool { loop { } }
}

#[stable(feature = "iter_copied", since = "1.36.0")]
impl<'a, I, T: 'a> FusedIterator for Copied<I>
    where I: FusedIterator<Item=&'a T>, T: Copy
{}

#[doc(hidden)]
unsafe impl<'a, I, T: 'a> TrustedRandomAccess for Copied<I>
    where I: TrustedRandomAccess<Item=&'a T>, T: Copy
{
    unsafe fn get_unchecked(&mut self, i: usize) -> Self::Item { loop { } }

    #[inline]
    fn may_have_side_effect() -> bool { loop { } }
}

#[stable(feature = "iter_copied", since = "1.36.0")]
unsafe impl<'a, I, T: 'a> TrustedLen for Copied<I>
    where I: TrustedLen<Item=&'a T>,
          T: Copy
{}

#[stable(feature = "iter_cloned", since = "1.1.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Clone, Debug)]
pub struct Cloned<I> {
    it: I,
}
impl<I> Cloned<I> {
    pub(super) fn new(it: I) -> Cloned<I> { loop { } }
}

#[stable(feature = "iter_cloned", since = "1.1.0")]
impl<'a, I, T: 'a> Iterator for Cloned<I>
    where I: Iterator<Item=&'a T>, T: Clone
{
    type Item = T;

    fn next(&mut self) -> Option<T> { loop { } }

    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    fn try_fold<B, F, R>(&mut self, init: B, f: F) -> R where
        Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Ok=B>
    { loop { } }

    fn fold<Acc, F>(self, init: Acc, f: F) -> Acc
        where F: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }
}

#[stable(feature = "iter_cloned", since = "1.1.0")]
impl<'a, I, T: 'a> DoubleEndedIterator for Cloned<I>
    where I: DoubleEndedIterator<Item=&'a T>, T: Clone
{
    fn next_back(&mut self) -> Option<T> { loop { } }

    fn try_rfold<B, F, R>(&mut self, init: B, f: F) -> R where
        Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Ok=B>
    { loop { } }

    fn rfold<Acc, F>(self, init: Acc, f: F) -> Acc
        where F: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }
}

#[stable(feature = "iter_cloned", since = "1.1.0")]
impl<'a, I, T: 'a> ExactSizeIterator for Cloned<I>
    where I: ExactSizeIterator<Item=&'a T>, T: Clone
{
    fn len(&self) -> usize { loop { } }

    fn is_empty(&self) -> bool { loop { } }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<'a, I, T: 'a> FusedIterator for Cloned<I>
    where I: FusedIterator<Item=&'a T>, T: Clone
{}

#[doc(hidden)]
unsafe impl<'a, I, T: 'a> TrustedRandomAccess for Cloned<I>
    where I: TrustedRandomAccess<Item=&'a T>, T: Clone
{
    default unsafe fn get_unchecked(&mut self, i: usize) -> Self::Item { loop { } }

    #[inline]
    default fn may_have_side_effect() -> bool { loop { } }
}

#[doc(hidden)]
unsafe impl<'a, I, T: 'a> TrustedRandomAccess for Cloned<I>
    where I: TrustedRandomAccess<Item=&'a T>, T: Copy
{
    unsafe fn get_unchecked(&mut self, i: usize) -> Self::Item { loop { } }

    #[inline]
    fn may_have_side_effect() -> bool { loop { } }
}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<'a, I, T: 'a> TrustedLen for Cloned<I>
    where I: TrustedLen<Item=&'a T>,
          T: Clone
{}

#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Cycle<I> {
    orig: I,
    iter: I,
}
impl<I: Clone> Cycle<I> {
    pub(super) fn new(iter: I) -> Cycle<I> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> Iterator for Cycle<I> where I: Clone + Iterator {
    type Item = <I as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<<I as Iterator>::Item> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn try_fold<Acc, F, R>(&mut self, mut acc: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
    { loop { } }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<I> FusedIterator for Cycle<I> where I: Clone + Iterator {}

#[must_use = "iterators are lazy and do nothing unless consumed"]
#[stable(feature = "iterator_step_by", since = "1.28.0")]
#[derive(Clone, Debug)]
pub struct StepBy<I> {
    iter: I,
    step: usize,
    first_take: bool,
}
impl<I> StepBy<I> {
    pub(super) fn new(iter: I, step: usize) -> StepBy<I> { loop { } }
}

#[stable(feature = "iterator_step_by", since = "1.28.0")]
impl<I> Iterator for StepBy<I> where I: Iterator {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn nth(&mut self, mut n: usize) -> Option<Self::Item> { loop { } }

    fn try_fold<Acc, F, R>(&mut self, mut acc: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
    { loop { } }
}

impl<I> StepBy<I> where I: ExactSizeIterator {
    fn next_back_index(&self) -> usize { loop { } }
}

#[stable(feature = "double_ended_step_by_iterator", since = "1.38.0")]
impl<I> DoubleEndedIterator for StepBy<I> where I: DoubleEndedIterator + ExactSizeIterator {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> { loop { } }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> { loop { } }

    fn try_rfold<Acc, F, R>(&mut self, init: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
    { loop { } }
}

#[stable(feature = "iterator_step_by", since = "1.28.0")]
impl<I> ExactSizeIterator for StepBy<I> where I: ExactSizeIterator {}

#[must_use = "iterators are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Clone)]
pub struct Map<I, F> {
    iter: I,
    f: F,
}
impl<I, F> Map<I, F> {
    pub(super) fn new(iter: I, f: F) -> Map<I, F> { loop { } }
}

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<I: fmt::Debug, F> fmt::Debug for Map<I, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<B, I: Iterator, F> Iterator for Map<I, F> where F: FnMut(I::Item) -> B {
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<B> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    fn try_fold<Acc, G, R>(&mut self, init: Acc, g: G) -> R where
        Self: Sized, G: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    { loop { } }

    fn fold<Acc, G>(self, init: Acc, g: G) -> Acc
        where G: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<B, I: DoubleEndedIterator, F> DoubleEndedIterator for Map<I, F> where
    F: FnMut(I::Item) -> B,
{
    #[inline]
    fn next_back(&mut self) -> Option<B> { loop { } }

    fn try_rfold<Acc, G, R>(&mut self, init: Acc, g: G) -> R where
        Self: Sized, G: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    { loop { } }

    fn rfold<Acc, G>(self, init: Acc, g: G) -> Acc
        where G: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<B, I: ExactSizeIterator, F> ExactSizeIterator for Map<I, F>
    where F: FnMut(I::Item) -> B
{
    fn len(&self) -> usize { loop { } }

    fn is_empty(&self) -> bool { loop { } }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<B, I: FusedIterator, F> FusedIterator for Map<I, F>
    where F: FnMut(I::Item) -> B {}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<B, I, F> TrustedLen for Map<I, F>
    where I: TrustedLen,
          F: FnMut(I::Item) -> B {}

#[doc(hidden)]
unsafe impl<B, I, F> TrustedRandomAccess for Map<I, F>
    where I: TrustedRandomAccess,
          F: FnMut(I::Item) -> B,
{
    unsafe fn get_unchecked(&mut self, i: usize) -> Self::Item { loop { } }
    #[inline]
    fn may_have_side_effect() -> bool { loop { } }
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Clone)]
pub struct Filter<I, P> {
    iter: I,
    predicate: P,
}
impl<I, P> Filter<I, P> {
    pub(super) fn new(iter: I, predicate: P) -> Filter<I, P> { loop { } }
}

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<I: fmt::Debug, P> fmt::Debug for Filter<I, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I: Iterator, P> Iterator for Filter<I, P> where P: FnMut(&I::Item) -> bool {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn count(self) -> usize { loop { } }

    #[inline]
    fn try_fold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    { loop { } }

    #[inline]
    fn fold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I: DoubleEndedIterator, P> DoubleEndedIterator for Filter<I, P>
    where P: FnMut(&I::Item) -> bool,
{
    #[inline]
    fn next_back(&mut self) -> Option<I::Item> { loop { } }

    #[inline]
    fn try_rfold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    { loop { } }

    #[inline]
    fn rfold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<I: FusedIterator, P> FusedIterator for Filter<I, P>
    where P: FnMut(&I::Item) -> bool {}

#[must_use = "iterators are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Clone)]
pub struct FilterMap<I, F> {
    iter: I,
    f: F,
}
impl<I, F> FilterMap<I, F> {
    pub(super) fn new(iter: I, f: F) -> FilterMap<I, F> { loop { } }
}

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<I: fmt::Debug, F> fmt::Debug for FilterMap<I, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}


#[stable(feature = "rust1", since = "1.0.0")]
impl<B, I: Iterator, F> Iterator for FilterMap<I, F>
    where F: FnMut(I::Item) -> Option<B>,
{
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<B> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn try_fold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    { loop { } }

    #[inline]
    fn fold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<B, I: DoubleEndedIterator, F> DoubleEndedIterator for FilterMap<I, F>
    where F: FnMut(I::Item) -> Option<B>,
{
    #[inline]
    fn next_back(&mut self) -> Option<B> { loop { } }

    #[inline]
    fn try_rfold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    { loop { } }

    #[inline]
    fn rfold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<B, I: FusedIterator, F> FusedIterator for FilterMap<I, F>
    where F: FnMut(I::Item) -> Option<B> {}

#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Enumerate<I> {
    iter: I,
    count: usize,
}
impl<I> Enumerate<I> {
    pub(super) fn new(iter: I) -> Enumerate<I> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> Iterator for Enumerate<I> where I: Iterator {
    type Item = (usize, <I as Iterator>::Item);

    #[inline]
    fn next(&mut self) -> Option<(usize, <I as Iterator>::Item)> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<(usize, I::Item)> { loop { } }

    #[inline]
    fn count(self) -> usize { loop { } }

    #[inline]
    fn try_fold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    { loop { } }

    #[inline]
    fn fold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> DoubleEndedIterator for Enumerate<I> where
    I: ExactSizeIterator + DoubleEndedIterator
{
    #[inline]
    fn next_back(&mut self) -> Option<(usize, <I as Iterator>::Item)> { loop { } }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<(usize, <I as Iterator>::Item)> { loop { } }

    #[inline]
    fn try_rfold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    { loop { } }

    #[inline]
    fn rfold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> ExactSizeIterator for Enumerate<I> where I: ExactSizeIterator {
    fn len(&self) -> usize { loop { } }

    fn is_empty(&self) -> bool { loop { } }
}

#[doc(hidden)]
unsafe impl<I> TrustedRandomAccess for Enumerate<I>
    where I: TrustedRandomAccess
{
    unsafe fn get_unchecked(&mut self, i: usize) -> (usize, I::Item) { loop { } }

    fn may_have_side_effect() -> bool { loop { } }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<I> FusedIterator for Enumerate<I> where I: FusedIterator {}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<I> TrustedLen for Enumerate<I>
    where I: TrustedLen,
{}


#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Peekable<I: Iterator> {
    iter: I,
    peeked: Option<Option<I::Item>>,
}
impl<I: Iterator> Peekable<I> {
    pub(super) fn new(iter: I) -> Peekable<I> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I: Iterator> Iterator for Peekable<I> {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> { loop { } }

    #[inline]
    #[rustc_inherit_overflow_checks]
    fn count(mut self) -> usize { loop { } }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<I::Item> { loop { } }

    #[inline]
    fn last(mut self) -> Option<I::Item> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R where
        Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Ok=B>
    { loop { } }

    #[inline]
    fn fold<Acc, Fold>(self, init: Acc, mut fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }
}

#[stable(feature = "double_ended_peek_iterator", since = "1.38.0")]
impl<I> DoubleEndedIterator for Peekable<I> where I: DoubleEndedIterator {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> { loop { } }

    #[inline]
    fn try_rfold<B, F, R>(&mut self, init: B, mut f: F) -> R where
        Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Ok=B>
    { loop { } }

    #[inline]
    fn rfold<Acc, Fold>(self, init: Acc, mut fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I: ExactSizeIterator> ExactSizeIterator for Peekable<I> {}

#[stable(feature = "fused", since = "1.26.0")]
impl<I: FusedIterator> FusedIterator for Peekable<I> {}

impl<I: Iterator> Peekable<I> {
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn peek(&mut self) -> Option<&I::Item> { loop { } }
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Clone)]
pub struct SkipWhile<I, P> {
    iter: I,
    flag: bool,
    predicate: P,
}
impl<I, P> SkipWhile<I, P> {
    pub(super) fn new(iter: I, predicate: P) -> SkipWhile<I, P> { loop { } }
}

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<I: fmt::Debug, P> fmt::Debug for SkipWhile<I, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I: Iterator, P> Iterator for SkipWhile<I, P>
    where P: FnMut(&I::Item) -> bool
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn try_fold<Acc, Fold, R>(&mut self, mut init: Acc, mut fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    { loop { } }

    #[inline]
    fn fold<Acc, Fold>(mut self, mut init: Acc, mut fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<I, P> FusedIterator for SkipWhile<I, P>
    where I: FusedIterator, P: FnMut(&I::Item) -> bool {}

#[must_use = "iterators are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Clone)]
pub struct TakeWhile<I, P> {
    iter: I,
    flag: bool,
    predicate: P,
}
impl<I, P> TakeWhile<I, P> {
    pub(super) fn new(iter: I, predicate: P) -> TakeWhile<I, P> { loop { } }
}

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<I: fmt::Debug, P> fmt::Debug for TakeWhile<I, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I: Iterator, P> Iterator for TakeWhile<I, P>
    where P: FnMut(&I::Item) -> bool
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn try_fold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    { loop { } }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<I, P> FusedIterator for TakeWhile<I, P>
    where I: FusedIterator, P: FnMut(&I::Item) -> bool {}

#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Skip<I> {
    iter: I,
    n: usize
}
impl<I> Skip<I> {
    pub(super) fn new(iter: I, n: usize) -> Skip<I> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> Iterator for Skip<I> where I: Iterator {
    type Item = <I as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> { loop { } }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<I::Item> { loop { } }

    #[inline]
    fn count(self) -> usize { loop { } }

    #[inline]
    fn last(mut self) -> Option<I::Item> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn try_fold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    { loop { } }

    #[inline]
    fn fold<Acc, Fold>(mut self, init: Acc, fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> ExactSizeIterator for Skip<I> where I: ExactSizeIterator {}

#[stable(feature = "double_ended_skip_iterator", since = "1.9.0")]
impl<I> DoubleEndedIterator for Skip<I> where I: DoubleEndedIterator + ExactSizeIterator {
    fn next_back(&mut self) -> Option<Self::Item> { loop { } }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<I::Item> { loop { } }

    fn try_rfold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    { loop { } }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<I> FusedIterator for Skip<I> where I: FusedIterator {}

#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Take<I> {
    pub(super) iter: I,
    pub(super) n: usize
}
impl<I> Take<I> {
    pub(super) fn new(iter: I, n: usize) -> Take<I> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> Iterator for Take<I> where I: Iterator{
    type Item = <I as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<<I as Iterator>::Item> { loop { } }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<I::Item> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn try_fold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    { loop { } }
}

#[stable(feature = "double_ended_take_iterator", since = "1.38.0")]
impl<I> DoubleEndedIterator for Take<I> where I: DoubleEndedIterator + ExactSizeIterator {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> { loop { } }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> { loop { } }

    #[inline]
    fn try_rfold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok = Acc>
    { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> ExactSizeIterator for Take<I> where I: ExactSizeIterator {}

#[stable(feature = "fused", since = "1.26.0")]
impl<I> FusedIterator for Take<I> where I: FusedIterator {}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<I: TrustedLen> TrustedLen for Take<I> {}

#[must_use = "iterators are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Clone)]
pub struct Scan<I, St, F> {
    iter: I,
    f: F,
    state: St,
}
impl<I, St, F> Scan<I, St, F> {
    pub(super) fn new(iter: I, state: St, f: F) -> Scan<I, St, F> { loop { } }
}

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<I: fmt::Debug, St: fmt::Debug, F> fmt::Debug for Scan<I, St, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<B, I, St, F> Iterator for Scan<I, St, F> where
    I: Iterator,
    F: FnMut(&mut St, I::Item) -> Option<B>,
{
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<B> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn try_fold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    { loop { } }
}

#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Fuse<I> {
    iter: I,
    done: bool
}
impl<I> Fuse<I> {
    pub(super) fn new(iter: I) -> Fuse<I> { loop { } }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<I> FusedIterator for Fuse<I> where I: Iterator {}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> Iterator for Fuse<I> where I: Iterator {
    type Item = <I as Iterator>::Item;

    #[inline]
    default fn next(&mut self) -> Option<<I as Iterator>::Item> { loop { } }

    #[inline]
    default fn nth(&mut self, n: usize) -> Option<I::Item> { loop { } }

    #[inline]
    default fn last(self) -> Option<I::Item> { loop { } }

    #[inline]
    default fn count(self) -> usize { loop { } }

    #[inline]
    default fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    default fn try_fold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    { loop { } }

    #[inline]
    default fn fold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> DoubleEndedIterator for Fuse<I> where I: DoubleEndedIterator {
    #[inline]
    default fn next_back(&mut self) -> Option<<I as Iterator>::Item> { loop { } }

    #[inline]
    default fn nth_back(&mut self, n: usize) -> Option<<I as Iterator>::Item> { loop { } }

    #[inline]
    default fn try_rfold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    { loop { } }

    #[inline]
    default fn rfold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }
}

unsafe impl<I> TrustedRandomAccess for Fuse<I>
    where I: TrustedRandomAccess,
{
    unsafe fn get_unchecked(&mut self, i: usize) -> I::Item { loop { } }

    fn may_have_side_effect() -> bool { loop { } }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<I> Iterator for Fuse<I> where I: FusedIterator {
    #[inline]
    fn next(&mut self) -> Option<<I as Iterator>::Item> { loop { } }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<I::Item> { loop { } }

    #[inline]
    fn last(self) -> Option<I::Item> { loop { } }

    #[inline]
    fn count(self) -> usize { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn try_fold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    { loop { } }

    #[inline]
    fn fold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<I> DoubleEndedIterator for Fuse<I>
    where I: DoubleEndedIterator + FusedIterator
{
    #[inline]
    fn next_back(&mut self) -> Option<<I as Iterator>::Item> { loop { } }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<<I as Iterator>::Item> { loop { } }

    #[inline]
    fn try_rfold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    { loop { } }

    #[inline]
    fn rfold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }
}


#[stable(feature = "rust1", since = "1.0.0")]
impl<I> ExactSizeIterator for Fuse<I> where I: ExactSizeIterator {
    fn len(&self) -> usize { loop { } }

    fn is_empty(&self) -> bool { loop { } }
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Clone)]
pub struct Inspect<I, F> {
    iter: I,
    f: F,
}
impl<I, F> Inspect<I, F> {
    pub(super) fn new(iter: I, f: F) -> Inspect<I, F> { loop { } }
}

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<I: fmt::Debug, F> fmt::Debug for Inspect<I, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

impl<I: Iterator, F> Inspect<I, F> where F: FnMut(&I::Item) {
    #[inline]
    fn do_inspect(&mut self, elt: Option<I::Item>) -> Option<I::Item> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I: Iterator, F> Iterator for Inspect<I, F> where F: FnMut(&I::Item) {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn try_fold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    { loop { } }

    #[inline]
    fn fold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I: DoubleEndedIterator, F> DoubleEndedIterator for Inspect<I, F>
    where F: FnMut(&I::Item),
{
    #[inline]
    fn next_back(&mut self) -> Option<I::Item> { loop { } }

    #[inline]
    fn try_rfold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    { loop { } }

    #[inline]
    fn rfold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I: ExactSizeIterator, F> ExactSizeIterator for Inspect<I, F>
    where F: FnMut(&I::Item)
{
    fn len(&self) -> usize { loop { } }

    fn is_empty(&self) -> bool { loop { } }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<I: FusedIterator, F> FusedIterator for Inspect<I, F>
    where F: FnMut(&I::Item) {}

pub(crate) struct ResultShunt<'a, I, E> {
    iter: I,
    error: &'a mut Result<(), E>,
}

pub(crate) fn process_results<I, T, E, F, U>(iter: I, mut f: F) -> Result<U, E>
where
    I: Iterator<Item = Result<T, E>>,
    for<'a> F: FnMut(ResultShunt<'a, I, E>) -> U,
{ loop { } }

impl<I, T, E> Iterator for ResultShunt<'_, I, E>
    where I: Iterator<Item = Result<T, E>>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> { loop { } }

    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R
    where
        F: FnMut(B, Self::Item) -> R,
        R: Try<Ok = B>,
    { loop { } }
}
