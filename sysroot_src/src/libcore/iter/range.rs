use crate::ops::{self, Try};
use crate::usize;

use super::{FusedIterator, TrustedLen};

#[unstable(feature = "step_trait",
           reason = "likely to be replaced by finer-grained traits",
           issue = "42168")]
pub trait Step: Clone + PartialOrd + Sized {
    fn steps_between(start: &Self, end: &Self) -> Option<usize>;

    fn replace_one(&mut self) -> Self;

    fn replace_zero(&mut self) -> Self;

    fn add_one(&self) -> Self;

    fn sub_one(&self) -> Self;

    fn add_usize(&self, n: usize) -> Option<Self>;

    fn sub_usize(&self, n: usize) -> Option<Self> { loop { } }
}

macro_rules! step_identical_methods {
    () => {
        #[inline]
        fn replace_one(&mut self) -> Self { loop { } }

        #[inline]
        fn replace_zero(&mut self) -> Self { loop { } }

        #[inline]
        fn add_one(&self) -> Self { loop { } }

        #[inline]
        fn sub_one(&self) -> Self { loop { } }
    }
}

macro_rules! step_impl_unsigned {
    ($($t:ty)*) => ($(
        #[unstable(feature = "step_trait",
                   reason = "likely to be replaced by finer-grained traits",
                   issue = "42168")]
        impl Step for $t {
            #[inline]
            fn steps_between(start: &$t, end: &$t) -> Option<usize> { loop { } }

            #[inline]
            #[allow(unreachable_patterns)]
            fn add_usize(&self, n: usize) -> Option<Self> { loop { } }

            #[inline]
            #[allow(unreachable_patterns)]
            fn sub_usize(&self, n: usize) -> Option<Self> { loop { } }

            step_identical_methods!();
        }
    )*)
}
macro_rules! step_impl_signed {
    ($( [$t:ty : $unsigned:ty] )*) => ($(
        #[unstable(feature = "step_trait",
                   reason = "likely to be replaced by finer-grained traits",
                   issue = "42168")]
        impl Step for $t {
            #[inline]
            fn steps_between(start: &$t, end: &$t) -> Option<usize> { loop { } }

            #[inline]
            #[allow(unreachable_patterns)]
            fn add_usize(&self, n: usize) -> Option<Self> { loop { } }

            #[inline]
            #[allow(unreachable_patterns)]
            fn sub_usize(&self, n: usize) -> Option<Self> { loop { } }

            step_identical_methods!();
        }
    )*)
}

step_impl_unsigned!(usize u8 u16 u32 u64 u128);
step_impl_signed!([isize: usize] [i8: u8] [i16: u16]);
step_impl_signed!([i32: u32] [i64: u64] [i128: u128]);

#[stable(feature = "rust1", since = "1.0.0")]
impl<A: Step> Iterator for ops::Range<A> {
    type Item = A;

    #[inline]
    fn next(&mut self) -> Option<A> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<A> { loop { } }

    #[inline]
    fn last(mut self) -> Option<A> { loop { } }

    #[inline]
    fn min(mut self) -> Option<A> { loop { } }

    #[inline]
    fn max(mut self) -> Option<A> { loop { } }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<A: Step> FusedIterator for ops::Range<A> {}

#[stable(feature = "rust1", since = "1.0.0")]
impl<A: Step> Iterator for ops::RangeFrom<A> {
    type Item = A;

    #[inline]
    fn next(&mut self) -> Option<A> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<A> { loop { } }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<A: Step> FusedIterator for ops::RangeFrom<A> {}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<A: Step> TrustedLen for ops::RangeFrom<A> {}

#[stable(feature = "inclusive_range", since = "1.26.0")]
impl<A: Step> Iterator for ops::RangeInclusive<A> {
    type Item = A;

    #[inline]
    fn next(&mut self) -> Option<A> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<A> { loop { } }

    #[inline]
    fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R
    where
        Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Ok=B>
    { loop { } }

    #[inline]
    fn last(mut self) -> Option<A> { loop { } }

    #[inline]
    fn min(mut self) -> Option<A> { loop { } }

    #[inline]
    fn max(mut self) -> Option<A> { loop { } }
}
