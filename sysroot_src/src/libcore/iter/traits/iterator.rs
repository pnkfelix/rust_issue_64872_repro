use crate::cmp::{Ordering};
use crate::ops::{Try};

use super::super::{Chain, Cycle, Copied, Cloned, Enumerate, Filter, FilterMap, Fuse};
use super::super::{Flatten, FlatMap};
use super::super::{Inspect, Map, Peekable, Scan, Skip, SkipWhile, StepBy, Take, TakeWhile};
use super::super::{Zip, Sum, Product, FromIterator};

fn _assert_is_object_safe(_: &dyn Iterator<Item=()>) {}

#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_on_unimplemented(
    on(
        _Self="[std::ops::Range<Idx>; 1]",
        label="if you meant to iterate between two values, remove the square brackets",
        note="`[start..end]` is an array of one `Range`; you might have meant to have a `Range` \
              without the brackets: `start..end`"
    ),
    on(
        _Self="[std::ops::RangeFrom<Idx>; 1]",
        label="if you meant to iterate from a value onwards, remove the square brackets",
        note="`[start..]` is an array of one `RangeFrom`; you might have meant to have a \
              `RangeFrom` without the brackets: `start..`, keeping in mind that iterating over an \
              unbounded iterator will run forever unless you `break` or `return` from within the \
              loop"
    ),
    on(
        _Self="[std::ops::RangeTo<Idx>; 1]",
        label="if you meant to iterate until a value, remove the square brackets and add a \
               starting value",
        note="`[..end]` is an array of one `RangeTo`; you might have meant to have a bounded \
              `Range` without the brackets: `0..end`"
    ),
    on(
        _Self="[std::ops::RangeInclusive<Idx>; 1]",
        label="if you meant to iterate between two values, remove the square brackets",
        note="`[start..=end]` is an array of one `RangeInclusive`; you might have meant to have a \
              `RangeInclusive` without the brackets: `start..=end`"
    ),
    on(
        _Self="[std::ops::RangeToInclusive<Idx>; 1]",
        label="if you meant to iterate until a value (including it), remove the square brackets \
               and add a starting value",
        note="`[..=end]` is an array of one `RangeToInclusive`; you might have meant to have a \
              bounded `RangeInclusive` without the brackets: `0..=end`"
    ),
    on(
        _Self="std::ops::RangeTo<Idx>",
        label="if you meant to iterate until a value, add a starting value",
        note="`..end` is a `RangeTo`, which cannot be iterated on; you might have meant to have a \
              bounded `Range`: `0..end`"
    ),
    on(
        _Self="std::ops::RangeToInclusive<Idx>",
        label="if you meant to iterate until a value (including it), add a starting value",
        note="`..=end` is a `RangeToInclusive`, which cannot be iterated on; you might have meant \
              to have a bounded `RangeInclusive`: `0..=end`"
    ),
    on(
        _Self="&str",
        label="`{Self}` is not an iterator; try calling `.chars()` or `.bytes()`"
    ),
    on(
        _Self="std::string::String",
        label="`{Self}` is not an iterator; try calling `.chars()` or `.bytes()`"
    ),
    on(
        _Self="[]",
        label="borrow the array with `&` or call `.iter()` on it to iterate over it",
        note="arrays are not iterators, but slices like the following are: `&[1, 2, 3]`"
    ),
    on(
        _Self="{integral}",
        note="if you want to iterate between `start` until a value `end`, use the exclusive range \
              syntax `start..end` or the inclusive range syntax `start..=end`"
    ),
    label="`{Self}` is not an iterator",
    message="`{Self}` is not an iterator"
)]
#[doc(spotlight)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub trait Iterator {
    #[stable(feature = "rust1", since = "1.0.0")]
    type Item;

    #[stable(feature = "rust1", since = "1.0.0")]
    fn next(&mut self) -> Option<Self::Item>;

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn count(self) -> usize where Self: Sized { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn last(self) -> Option<Self::Item> where Self: Sized { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn nth(&mut self, mut n: usize) -> Option<Self::Item> { loop { } }

    #[inline]
    #[stable(feature = "iterator_step_by", since = "1.28.0")]
    fn step_by(self, step: usize) -> StepBy<Self> where Self: Sized { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn chain<U>(self, other: U) -> Chain<Self, U::IntoIter> where
        Self: Sized, U: IntoIterator<Item=Self::Item>,
    { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn zip<U>(self, other: U) -> Zip<Self, U::IntoIter> where
        Self: Sized, U: IntoIterator
    { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn map<B, F>(self, f: F) -> Map<Self, F> where
        Self: Sized, F: FnMut(Self::Item) -> B,
    { loop { } }

    #[inline]
    #[stable(feature = "iterator_for_each", since = "1.21.0")]
    fn for_each<F>(self, f: F) where
        Self: Sized, F: FnMut(Self::Item),
    { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn filter<P>(self, predicate: P) -> Filter<Self, P> where
        Self: Sized, P: FnMut(&Self::Item) -> bool,
    { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F> where
        Self: Sized, F: FnMut(Self::Item) -> Option<B>,
    { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn enumerate(self) -> Enumerate<Self> where Self: Sized { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn peekable(self) -> Peekable<Self> where Self: Sized { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P> where
        Self: Sized, P: FnMut(&Self::Item) -> bool,
    { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P> where
        Self: Sized, P: FnMut(&Self::Item) -> bool,
    { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn skip(self, n: usize) -> Skip<Self> where Self: Sized { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn take(self, n: usize) -> Take<Self> where Self: Sized, { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F>
        where Self: Sized, F: FnMut(&mut St, Self::Item) -> Option<B>,
    { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>
        where Self: Sized, U: IntoIterator, F: FnMut(Self::Item) -> U,
    { loop { } }

    #[inline]
    #[stable(feature = "iterator_flatten", since = "1.29.0")]
    fn flatten(self) -> Flatten<Self>
    where Self: Sized, Self::Item: IntoIterator { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn fuse(self) -> Fuse<Self> where Self: Sized { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn inspect<F>(self, f: F) -> Inspect<Self, F> where
        Self: Sized, F: FnMut(&Self::Item),
    { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    fn by_ref(&mut self) -> &mut Self where Self: Sized { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[must_use = "if you really need to exhaust the iterator, consider `.for_each(drop)` instead"]
    fn collect<B: FromIterator<Self::Item>>(self) -> B where Self: Sized { loop { } }

    #[unstable(feature = "iter_is_partitioned", reason = "new API", issue = "62544")]
    fn is_partitioned<P>(mut self, mut predicate: P) -> bool
    where
        Self: Sized,
        P: FnMut(Self::Item) -> bool,
    { loop { } }

    #[inline]
    #[stable(feature = "iterator_try_fold", since = "1.27.0")]
    fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R where
        Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Ok=B>
    { loop { } }

    #[inline]
    #[stable(feature = "iterator_try_fold", since = "1.27.0")]
    fn try_for_each<F, R>(&mut self, f: F) -> R where
        Self: Sized, F: FnMut(Self::Item) -> R, R: Try<Ok=()>
    { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn fold<B, F>(mut self, init: B, f: F) -> B where
        Self: Sized, F: FnMut(B, Self::Item) -> B,
    { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn all<F>(&mut self, f: F) -> bool where
        Self: Sized, F: FnMut(Self::Item) -> bool
    { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn any<F>(&mut self, f: F) -> bool where
        Self: Sized,
        F: FnMut(Self::Item) -> bool
    { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    { loop { } }

    #[inline]
    #[stable(feature = "iterator_find_map", since = "1.30.0")]
    fn find_map<B, F>(&mut self, f: F) -> Option<B> where
        Self: Sized,
        F: FnMut(Self::Item) -> Option<B>,
    { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn position<P>(&mut self, predicate: P) -> Option<usize> where
        Self: Sized,
        P: FnMut(Self::Item) -> bool,
    { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn max(self) -> Option<Self::Item> where Self: Sized, Self::Item: Ord
    { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn min(self) -> Option<Self::Item> where Self: Sized, Self::Item: Ord
    { loop { } }

    #[inline]
    #[stable(feature = "iter_cmp_by_key", since = "1.6.0")]
    fn max_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where Self: Sized, F: FnMut(&Self::Item) -> B,
    { loop { } }

    #[inline]
    #[stable(feature = "iter_max_by", since = "1.15.0")]
    fn max_by<F>(self, compare: F) -> Option<Self::Item>
        where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    { loop { } }

    #[inline]
    #[stable(feature = "iter_cmp_by_key", since = "1.6.0")]
    fn min_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where Self: Sized, F: FnMut(&Self::Item) -> B,
    { loop { } }

    #[inline]
    #[stable(feature = "iter_min_by", since = "1.15.0")]
    fn min_by<F>(self, compare: F) -> Option<Self::Item>
        where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    { loop { } }


    #[stable(feature = "iter_copied", since = "1.36.0")]
    fn copied<'a, T: 'a>(self) -> Copied<Self>
        where Self: Sized + Iterator<Item=&'a T>, T: Copy
    { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    fn cloned<'a, T: 'a>(self) -> Cloned<Self>
        where Self: Sized + Iterator<Item=&'a T>, T: Clone
    { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    fn cycle(self) -> Cycle<Self> where Self: Sized + Clone { loop { } }

    #[stable(feature = "iter_arith", since = "1.11.0")]
    fn sum<S>(self) -> S
        where Self: Sized,
              S: Sum<Self::Item>,
    { loop { } }

    #[stable(feature = "iter_arith", since = "1.11.0")]
    fn product<P>(self) -> P
        where Self: Sized,
              P: Product<Self::Item>,
    { loop { } }

    #[stable(feature = "iter_order", since = "1.5.0")]
    fn cmp<I>(self, other: I) -> Ordering
    where
        I: IntoIterator<Item = Self::Item>,
        Self::Item: Ord,
        Self: Sized,
    { loop { } }

    #[unstable(feature = "iter_order_by", issue = "64295")]
    fn cmp_by<I, F>(mut self, other: I, mut cmp: F) -> Ordering
    where
        Self: Sized,
        I: IntoIterator,
        F: FnMut(Self::Item, I::Item) -> Ordering,
    { loop { } }

    #[stable(feature = "iter_order", since = "1.5.0")]
    fn partial_cmp<I>(self, other: I) -> Option<Ordering>
    where
        I: IntoIterator,
        Self::Item: PartialOrd<I::Item>,
        Self: Sized,
    { loop { } }

    #[unstable(feature = "iter_order_by", issue = "64295")]
    fn partial_cmp_by<I, F>(mut self, other: I, mut partial_cmp: F) -> Option<Ordering>
    where
        Self: Sized,
        I: IntoIterator,
        F: FnMut(Self::Item, I::Item) -> Option<Ordering>,
    { loop { } }

    #[stable(feature = "iter_order", since = "1.5.0")]
    fn eq<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialEq<I::Item>,
        Self: Sized,
    { loop { } }

    #[unstable(feature = "iter_order_by", issue = "64295")]
    fn eq_by<I, F>(mut self, other: I, mut eq: F) -> bool
    where
        Self: Sized,
        I: IntoIterator,
        F: FnMut(Self::Item, I::Item) -> bool,
    { loop { } }

    #[stable(feature = "iter_order", since = "1.5.0")]
    fn ne<I>(self, other: I) -> bool where
        I: IntoIterator,
        Self::Item: PartialEq<I::Item>,
        Self: Sized,
    { loop { } }

    #[stable(feature = "iter_order", since = "1.5.0")]
    fn lt<I>(self, other: I) -> bool where
        I: IntoIterator,
        Self::Item: PartialOrd<I::Item>,
        Self: Sized,
    { loop { } }

    #[stable(feature = "iter_order", since = "1.5.0")]
    fn le<I>(self, other: I) -> bool where
        I: IntoIterator,
        Self::Item: PartialOrd<I::Item>,
        Self: Sized,
    { loop { } }

    #[stable(feature = "iter_order", since = "1.5.0")]
    fn gt<I>(self, other: I) -> bool where
        I: IntoIterator,
        Self::Item: PartialOrd<I::Item>,
        Self: Sized,
    { loop { } }

    #[stable(feature = "iter_order", since = "1.5.0")]
    fn ge<I>(self, other: I) -> bool where
        I: IntoIterator,
        Self::Item: PartialOrd<I::Item>,
        Self: Sized,
    { loop { } }

    #[inline]
    #[unstable(feature = "is_sorted", reason = "new API", issue = "53485")]
    fn is_sorted(self) -> bool
    where
        Self: Sized,
        Self::Item: PartialOrd,
    { loop { } }

    #[unstable(feature = "is_sorted", reason = "new API", issue = "53485")]
    fn is_sorted_by<F>(mut self, mut compare: F) -> bool
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Option<Ordering>
    { loop { } }

    #[inline]
    #[unstable(feature = "is_sorted", reason = "new API", issue = "53485")]
    fn is_sorted_by_key<F, K>(self, f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> K,
        K: PartialOrd
    { loop { } }
}

#[inline]
fn fold1<I, F>(mut it: I, f: F) -> Option<I::Item>
    where
        I: Iterator,
        F: FnMut(I::Item, I::Item) -> I::Item,
{ loop { } }

#[stable(feature = "rust1", since = "1.0.0")]
impl<I: Iterator + ?Sized> Iterator for &mut I {
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> { loop { } }
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }
    fn nth(&mut self, n: usize) -> Option<Self::Item> { loop { } }
}
