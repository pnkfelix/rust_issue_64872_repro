use crate::ops::Try;
use crate::usize;

use super::super::{Iterator, DoubleEndedIterator, FusedIterator, TrustedLen};

#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Chain<A, B> {
    a: A,
    b: B,
    state: ChainState,
}
impl<A, B> Chain<A, B> {
    pub(in super::super) fn new(a: A, b: B) -> Chain<A, B> { loop { } }
}

#[derive(Clone, Debug)]
enum ChainState {
    Both,
    Front,
    Back,
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<A, B> Iterator for Chain<A, B> where
    A: Iterator,
    B: Iterator<Item = A::Item>
{
    type Item = A::Item;

    #[inline]
    fn next(&mut self) -> Option<A::Item> { loop { } }

    #[inline]
    #[rustc_inherit_overflow_checks]
    fn count(self) -> usize { loop { } }

    fn try_fold<Acc, F, R>(&mut self, init: Acc, mut f: F) -> R where
        Self: Sized, F: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    { loop { } }

    fn fold<Acc, F>(self, init: Acc, mut f: F) -> Acc
        where F: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }

    #[inline]
    fn nth(&mut self, mut n: usize) -> Option<A::Item> { loop { } }

    #[inline]
    fn find<P>(&mut self, mut predicate: P) -> Option<Self::Item> where
        P: FnMut(&Self::Item) -> bool,
    { loop { } }

    #[inline]
    fn last(self) -> Option<A::Item> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<A, B> DoubleEndedIterator for Chain<A, B> where
    A: DoubleEndedIterator,
    B: DoubleEndedIterator<Item=A::Item>,
{
    #[inline]
    fn next_back(&mut self) -> Option<A::Item> { loop { } }

    #[inline]
    fn nth_back(&mut self, mut n: usize) -> Option<A::Item> { loop { } }

    fn try_rfold<Acc, F, R>(&mut self, init: Acc, mut f: F) -> R where
        Self: Sized, F: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    { loop { } }

    fn rfold<Acc, F>(self, init: Acc, mut f: F) -> Acc
        where F: FnMut(Acc, Self::Item) -> Acc,
    { loop { } }

}

#[stable(feature = "fused", since = "1.26.0")]
impl<A, B> FusedIterator for Chain<A, B>
    where A: FusedIterator,
          B: FusedIterator<Item=A::Item>,
{}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<A, B> TrustedLen for Chain<A, B>
    where A: TrustedLen, B: TrustedLen<Item=A::Item>,
{}
