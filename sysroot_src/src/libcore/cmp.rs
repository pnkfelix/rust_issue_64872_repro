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

#[derive(PartialEq, Debug)]
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

#[stable(feature = "rust1", since = "1.0.0")]
impl Eq for Ordering {}

#[stable(feature = "rust1", since = "1.0.0")]
impl Ord for Ordering {
    #[inline]
    fn cmp(&self, other: &Ordering) -> Ordering { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl PartialOrd for Ordering {
    #[inline]
    fn partial_cmp(&self, other: &Ordering) -> Option<Ordering> { loop { } }
}

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

mod impls {
    use crate::cmp::Ordering::{self};

    macro_rules! partial_eq_impl {
        ($($t:ty)*) => ($(
            #[stable(feature = "rust1", since = "1.0.0")]
            impl PartialEq for $t {
                #[inline]
                fn eq(&self, other: &$t) -> bool { loop { } }
                #[inline]
                fn ne(&self, other: &$t) -> bool { loop { } }
            }
        )*)
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl PartialEq for () {
        #[inline]
        fn eq(&self, _other: &()) -> bool { loop { } }
        #[inline]
        fn ne(&self, _other: &()) -> bool { loop { } }
    }

    partial_eq_impl! {
        bool char usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64
    }

    macro_rules! eq_impl {
        ($($t:ty)*) => ($(
            #[stable(feature = "rust1", since = "1.0.0")]
            impl Eq for $t {}
        )*)
    }

    eq_impl! { () bool char usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

    macro_rules! partial_ord_impl {
        ($($t:ty)*) => ($(
            #[stable(feature = "rust1", since = "1.0.0")]
            impl PartialOrd for $t {
                #[inline]
                fn partial_cmp(&self, other: &$t) -> Option<Ordering> { loop { } }
                #[inline]
                fn lt(&self, other: &$t) -> bool { loop { } }
                #[inline]
                fn le(&self, other: &$t) -> bool { loop { } }
                #[inline]
                fn ge(&self, other: &$t) -> bool { loop { } }
                #[inline]
                fn gt(&self, other: &$t) -> bool { loop { } }
            }
        )*)
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl PartialOrd for () {
        #[inline]
        fn partial_cmp(&self, _: &()) -> Option<Ordering> { loop { } }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl PartialOrd for bool {
        #[inline]
        fn partial_cmp(&self, other: &bool) -> Option<Ordering> { loop { } }
    }

    partial_ord_impl! { f32 f64 }

    macro_rules! ord_impl {
        ($($t:ty)*) => ($(
            #[stable(feature = "rust1", since = "1.0.0")]
            impl PartialOrd for $t {
                #[inline]
                fn partial_cmp(&self, other: &$t) -> Option<Ordering> { loop { } }
                #[inline]
                fn lt(&self, other: &$t) -> bool { loop { } }
                #[inline]
                fn le(&self, other: &$t) -> bool { loop { } }
                #[inline]
                fn ge(&self, other: &$t) -> bool { loop { } }
                #[inline]
                fn gt(&self, other: &$t) -> bool { loop { } }
            }

            #[stable(feature = "rust1", since = "1.0.0")]
            impl Ord for $t {
                #[inline]
                fn cmp(&self, other: &$t) -> Ordering { loop { } }
            }
        )*)
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl Ord for () {
        #[inline]
        fn cmp(&self, _other: &()) -> Ordering { loop { } }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl Ord for bool {
        #[inline]
        fn cmp(&self, other: &bool) -> Ordering { loop { } }
    }

    ord_impl! { char usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

    #[unstable(feature = "never_type", issue = "35121")]
    impl PartialEq for ! {
        fn eq(&self, _: &!) -> bool { loop { } }
    }

    #[unstable(feature = "never_type", issue = "35121")]
    impl Eq for ! {}

    #[unstable(feature = "never_type", issue = "35121")]
    impl PartialOrd for ! {
        fn partial_cmp(&self, _: &!) -> Option<Ordering> { loop { } }
    }

    #[unstable(feature = "never_type", issue = "35121")]
    impl Ord for ! {
        fn cmp(&self, _: &!) -> Ordering { loop { } }
    }


    #[stable(feature = "rust1", since = "1.0.0")]
    impl<A: ?Sized, B: ?Sized> PartialEq<&B> for &A where A: PartialEq<B> {
        #[inline]
        fn eq(&self, other: & &B) -> bool { loop { } }
        #[inline]
        fn ne(&self, other: & &B) -> bool { loop { } }
    }
    #[stable(feature = "rust1", since = "1.0.0")]
    impl<A: ?Sized, B: ?Sized> PartialOrd<&B> for &A where A: PartialOrd<B> {
        #[inline]
        fn partial_cmp(&self, other: &&B) -> Option<Ordering> { loop { } }
        #[inline]
        fn lt(&self, other: & &B) -> bool { loop { } }
        #[inline]
        fn le(&self, other: & &B) -> bool { loop { } }
        #[inline]
        fn ge(&self, other: & &B) -> bool { loop { } }
        #[inline]
        fn gt(&self, other: & &B) -> bool { loop { } }
    }
    #[stable(feature = "rust1", since = "1.0.0")]
    impl<A: ?Sized> Ord for &A where A: Ord {
        #[inline]
        fn cmp(&self, other: &Self) -> Ordering { loop { } }
    }
    #[stable(feature = "rust1", since = "1.0.0")]
    impl<A: ?Sized> Eq for &A where A: Eq {}


    #[stable(feature = "rust1", since = "1.0.0")]
    impl<A: ?Sized, B: ?Sized> PartialEq<&mut B> for &mut A where A: PartialEq<B> {
        #[inline]
        fn eq(&self, other: &&mut B) -> bool { loop { } }
        #[inline]
        fn ne(&self, other: &&mut B) -> bool { loop { } }
    }
    #[stable(feature = "rust1", since = "1.0.0")]
    impl<A: ?Sized, B: ?Sized> PartialOrd<&mut B> for &mut A where A: PartialOrd<B> {
        #[inline]
        fn partial_cmp(&self, other: &&mut B) -> Option<Ordering> { loop { } }
        #[inline]
        fn lt(&self, other: &&mut B) -> bool { loop { } }
        #[inline]
        fn le(&self, other: &&mut B) -> bool { loop { } }
        #[inline]
        fn ge(&self, other: &&mut B) -> bool { loop { } }
        #[inline]
        fn gt(&self, other: &&mut B) -> bool { loop { } }
    }
    #[stable(feature = "rust1", since = "1.0.0")]
    impl<A: ?Sized> Ord for &mut A where A: Ord {
        #[inline]
        fn cmp(&self, other: &Self) -> Ordering { loop { } }
    }
    #[stable(feature = "rust1", since = "1.0.0")]
    impl<A: ?Sized> Eq for &mut A where A: Eq {}

    #[stable(feature = "rust1", since = "1.0.0")]
    impl<A: ?Sized, B: ?Sized> PartialEq<&mut B> for &A where A: PartialEq<B> {
        #[inline]
        fn eq(&self, other: &&mut B) -> bool { loop { } }
        #[inline]
        fn ne(&self, other: &&mut B) -> bool { loop { } }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl<A: ?Sized, B: ?Sized> PartialEq<&B> for &mut A where A: PartialEq<B> {
        #[inline]
        fn eq(&self, other: &&B) -> bool { loop { } }
        #[inline]
        fn ne(&self, other: &&B) -> bool { loop { } }
    }
}
