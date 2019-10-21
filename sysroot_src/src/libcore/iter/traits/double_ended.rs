use crate::ops::Try;

#[stable(feature = "rust1", since = "1.0.0")]
pub trait DoubleEndedIterator: Iterator {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn next_back(&mut self) -> Option<Self::Item>;

    #[inline]
    #[stable(feature = "iter_nth_back", since = "1.37.0")]
    fn nth_back(&mut self, mut n: usize) -> Option<Self::Item> { loop { } }

    #[inline]
    #[stable(feature = "iterator_try_fold", since = "1.27.0")]
    fn try_rfold<B, F, R>(&mut self, init: B, mut f: F) -> R
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> R,
        R: Try<Ok=B>
    { loop { } }

    #[inline]
    #[stable(feature = "iter_rfold", since = "1.27.0")]
    fn rfold<B, F>(mut self, accum: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    { loop { } }

    #[inline]
    #[stable(feature = "iter_rfind", since = "1.27.0")]
    fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool
    { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, I: DoubleEndedIterator + ?Sized> DoubleEndedIterator for &'a mut I {
    fn next_back(&mut self) -> Option<I::Item> { loop { } }
    fn nth_back(&mut self, n: usize) -> Option<I::Item> { loop { } }
}
