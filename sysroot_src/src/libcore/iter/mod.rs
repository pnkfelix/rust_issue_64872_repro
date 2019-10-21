//! Composable external iteration.

#![stable(feature = "rust1", since = "1.0.0")]

use crate::ops::Try;

#[stable(feature = "rust1", since = "1.0.0")]
pub use self::traits::Iterator;

#[unstable(feature = "step_trait",
           reason = "likely to be replaced by finer-grained traits",
           issue = "42168")]
pub use self::range::Step;

#[stable(feature = "rust1", since = "1.0.0")]
pub use self::sources::{Repeat, repeat};
#[stable(feature = "iterator_repeat_with", since = "1.28.0")]
pub use self::sources::{RepeatWith, repeat_with};
#[stable(feature = "iter_empty", since = "1.2.0")]
pub use self::sources::{Empty, empty};
#[stable(feature = "iter_once", since = "1.2.0")]
pub use self::sources::{Once, once};
#[unstable(feature = "iter_once_with", issue = "57581")]
pub use self::sources::{OnceWith, once_with};
#[stable(feature = "iter_from_fn", since = "1.34.0")]
pub use self::sources::{FromFn, from_fn};
#[stable(feature = "iter_successors", since = "1.34.0")]
pub use self::sources::{Successors, successors};

#[stable(feature = "rust1", since = "1.0.0")]
pub use self::traits::{FromIterator, IntoIterator, DoubleEndedIterator, Extend};
#[stable(feature = "rust1", since = "1.0.0")]
pub use self::traits::{ExactSizeIterator, Sum, Product};
#[stable(feature = "fused", since = "1.26.0")]
pub use self::traits::FusedIterator;
#[unstable(feature = "trusted_len", issue = "37572")]
pub use self::traits::TrustedLen;

#[stable(feature = "rust1", since = "1.0.0")]
pub use self::adapters::{Rev, Cycle, Chain, Zip, Map, Filter, FilterMap, Enumerate};
#[stable(feature = "rust1", since = "1.0.0")]
pub use self::adapters::{Peekable, SkipWhile, TakeWhile, Skip, Take, Scan, FlatMap};
#[stable(feature = "rust1", since = "1.0.0")]
pub use self::adapters::{Fuse, Inspect};
#[stable(feature = "iter_cloned", since = "1.1.0")]
pub use self::adapters::Cloned;
#[stable(feature = "iterator_step_by", since = "1.28.0")]
pub use self::adapters::StepBy;
#[stable(feature = "iterator_flatten", since = "1.29.0")]
pub use self::adapters::Flatten;
#[stable(feature = "iter_copied", since = "1.36.0")]
pub use self::adapters::Copied;

pub(crate) use self::adapters::{TrustedRandomAccess};

mod range;
mod sources;
mod traits;
mod adapters;

#[derive(PartialEq)]
enum LoopState<C, B> {
    Continue(C),
    Break(B),
}

impl<C, B> Try for LoopState<C, B> {
    type Ok = C;
    type Error = B;
    #[inline]
    fn into_result(self) -> Result<Self::Ok, Self::Error> { loop { } }
    #[inline]
    fn from_error(v: Self::Error) -> Self { loop { } }
    #[inline]
    fn from_ok(v: Self::Ok) -> Self { loop { } }
}

impl<C, B> LoopState<C, B> {
    #[inline]
    fn break_value(self) -> Option<B> { loop { } }
}

impl<R: Try> LoopState<R::Ok, R> {
    #[inline]
    fn from_try(r: R) -> Self { loop { } }
    #[inline]
    fn into_try(self) -> R { loop { } }
}
