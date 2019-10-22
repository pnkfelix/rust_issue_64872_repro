#![stable(feature = "rust1", since = "1.0.0")]

#[lang = "eq"]
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(alias = "==")]
#[doc(alias = "!=")]
#[rustc_on_unimplemented(
    message="can't compare `{Self}` with `{Rhs}`",
    label="no implementation for `{Self} == {Rhs}`",
)]
pub trait PartialEq<Rhs: ?Sized = Self> {
    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn eq(&self, other: &Rhs) -> bool;

    #[inline]
    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn ne(&self, other: &Rhs) -> bool { loop { } }
}

#[rustc_builtin_macro]
#[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
#[allow_internal_unstable(core_intrinsics)]
pub macro PartialEq($item:item) { /* compiler built-in */ }

#[doc(alias = "==")]
#[doc(alias = "!=")]
#[stable(feature = "rust1", since = "1.0.0")]
pub trait Eq: PartialEq<Self> {
    #[doc(hidden)]
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn assert_receiver_is_total_eq(&self) { loop { } }
}

#[rustc_builtin_macro]
#[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
#[allow_internal_unstable(core_intrinsics, derive_eq)]
pub macro Eq($item:item) { /* compiler built-in */ }

#[doc(hidden)]
#[allow(missing_debug_implementations)]
#[unstable(feature = "derive_eq",
           reason = "deriving hack, should not be public",
           issue = "0")]
pub struct AssertParamIsEq<T: Eq + ?Sized> { _field: crate::marker::PhantomData<T> }

#[derive(Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
pub enum Ordering {
    #[stable(feature = "rust1", since = "1.0.0")]
    Less = -1,
    #[stable(feature = "rust1", since = "1.0.0")]
    Equal = 0,
    #[stable(feature = "rust1", since = "1.0.0")]
    Greater = 1,
}

impl Ordering {
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn reverse(self) -> Ordering { loop { } }

    #[inline]
    #[stable(feature = "ordering_chaining", since = "1.17.0")]
    pub fn then(self, other: Ordering) -> Ordering { loop { } }

    #[inline]
    #[stable(feature = "ordering_chaining", since = "1.17.0")]
    pub fn then_with<F: FnOnce() -> Ordering>(self, f: F) -> Ordering { loop { } }
}

#[derive(PartialEq, Eq, Debug)]
#[stable(feature = "reverse_cmp_key", since = "1.19.0")]
pub struct Reverse<T>(#[stable(feature = "reverse_cmp_key", since = "1.19.0")] pub T);

#[stable(feature = "reverse_cmp_key", since = "1.19.0")]
impl<T: PartialOrd> PartialOrd for Reverse<T> {
    #[inline]
    fn partial_cmp(&self, other: &Reverse<T>) -> Option<Ordering> { loop { } }

    #[inline]
    fn lt(&self, other: &Self) -> bool { loop { } }
    #[inline]
    fn le(&self, other: &Self) -> bool { loop { } }
    #[inline]
    fn ge(&self, other: &Self) -> bool { loop { } }
    #[inline]
    fn gt(&self, other: &Self) -> bool { loop { } }
}

#[stable(feature = "reverse_cmp_key", since = "1.19.0")]
impl<T: Ord> Ord for Reverse<T> {
    #[inline]
    fn cmp(&self, other: &Reverse<T>) -> Ordering { loop { } }
}

#[lang = "ord"]
#[doc(alias = "<")]
#[doc(alias = ">")]
#[doc(alias = "<=")]
#[doc(alias = ">=")]
#[stable(feature = "rust1", since = "1.0.0")]
pub trait Ord: Eq + PartialOrd<Self> {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn cmp(&self, other: &Self) -> Ordering;

    #[stable(feature = "ord_max_min", since = "1.21.0")]
    #[inline]
    fn max(self, other: Self) -> Self
    where Self: Sized { loop { } }

    #[stable(feature = "ord_max_min", since = "1.21.0")]
    #[inline]
    fn min(self, other: Self) -> Self
    where Self: Sized { loop { } }

    #[unstable(feature = "clamp", issue = "44095")]
    fn clamp(self, min: Self, max: Self) -> Self
    where Self: Sized { loop { } }
}

#[rustc_builtin_macro]
#[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
#[allow_internal_unstable(core_intrinsics)]
pub macro Ord($item:item) { /* compiler built-in */ }

#[lang = "partial_ord"]
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(alias = ">")]
#[doc(alias = "<")]
#[doc(alias = "<=")]
#[doc(alias = ">=")]
#[rustc_on_unimplemented(
    message="can't compare `{Self}` with `{Rhs}`",
    label="no implementation for `{Self} < {Rhs}` and `{Self} > {Rhs}`",
)]
pub trait PartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> {
    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

    #[inline]
    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn lt(&self, other: &Rhs) -> bool { loop { } }

    #[inline]
    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn le(&self, other: &Rhs) -> bool { loop { } }

    #[inline]
    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn gt(&self, other: &Rhs) -> bool { loop { } }

    #[inline]
    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn ge(&self, other: &Rhs) -> bool { loop { } }
}

#[rustc_builtin_macro]
#[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
#[allow_internal_unstable(core_intrinsics)]
pub macro PartialOrd($item:item) { /* compiler built-in */ }

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
pub fn min<T: Ord>(v1: T, v2: T) -> T { loop { } }

#[inline]
#[unstable(feature = "cmp_min_max_by", issue = "64460")]
pub fn min_by<T, F: FnOnce(&T, &T) -> Ordering>(v1: T, v2: T, compare: F) -> T { loop { } }

#[inline]
#[unstable(feature = "cmp_min_max_by", issue = "64460")]
pub fn min_by_key<T, F: FnMut(&T) -> K, K: Ord>(v1: T, v2: T, mut f: F) -> T { loop { } }

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
pub fn max<T: Ord>(v1: T, v2: T) -> T { loop { } }

#[inline]
#[unstable(feature = "cmp_min_max_by", issue = "64460")]
pub fn max_by<T, F: FnOnce(&T, &T) -> Ordering>(v1: T, v2: T, compare: F) -> T { loop { } }

#[inline]
#[unstable(feature = "cmp_min_max_by", issue = "64460")]
pub fn max_by_key<T, F: FnMut(&T) -> K, K: Ord>(v1: T, v2: T, mut f: F) -> T { loop { } }
