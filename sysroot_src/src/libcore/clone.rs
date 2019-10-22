//! The `Clone` trait for types that cannot be 'implicitly copied'.

#![stable(feature = "rust1", since = "1.0.0")]

#[stable(feature = "rust1", since = "1.0.0")]
#[lang = "clone"]
pub trait Clone : Sized {
    #[stable(feature = "rust1", since = "1.0.0")]
    #[must_use = "cloning is often expensive and is not expected to have side effects"]
    fn clone(&self) -> Self;

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn clone_from(&mut self, source: &Self) { loop { } }
}

#[rustc_builtin_macro]
#[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
#[allow_internal_unstable(core_intrinsics, derive_clone_copy)]
pub macro Clone($item:item) { /* compiler built-in */ }

#[doc(hidden)]
#[allow(missing_debug_implementations)]
#[unstable(feature = "derive_clone_copy",
           reason = "deriving hack, should not be public",
           issue = "0")]
pub struct AssertParamIsClone<T: Clone + ?Sized> { _field: crate::marker::PhantomData<T> }
#[doc(hidden)]
#[allow(missing_debug_implementations)]
#[unstable(feature = "derive_clone_copy",
           reason = "deriving hack, should not be public",
           issue = "0")]
pub struct AssertParamIsCopy<T: ?Sized> { _field: crate::marker::PhantomData<T> }

mod impls {

    use super::Clone;

    macro_rules! impl_clone {
        ($($t:ty)*) => {
            $(
                #[stable(feature = "rust1", since = "1.0.0")]
                impl Clone for $t {
                    #[inline]
                    fn clone(&self) -> Self { loop { } }
                }
            )*
        }
    }

    impl_clone! {
        usize u8 u16 u32 u64 u128
        isize i8 i16 i32 i64 i128
        f32 f64
        bool char
    }

    #[unstable(feature = "never_type", issue = "35121")]
    impl Clone for ! {
        #[inline]
        fn clone(&self) -> Self { loop { } }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl<T: ?Sized> Clone for *const T {
        #[inline]
        fn clone(&self) -> Self { loop { } }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl<T: ?Sized> Clone for *mut T {
        #[inline]
        fn clone(&self) -> Self { loop { } }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl<T: ?Sized> Clone for &T {
        #[inline]
        fn clone(&self) -> Self { loop { } }
    }

}
