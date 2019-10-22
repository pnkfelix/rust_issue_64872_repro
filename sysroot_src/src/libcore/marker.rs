//! Primitive traits and types representing basic properties of types.

#![stable(feature = "rust1", since = "1.0.0")]

use crate::hash::Hash;

#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_on_unimplemented(
    message="`{Self}` cannot be sent between threads safely",
    label="`{Self}` cannot be sent between threads safely"
)]
pub unsafe auto trait Send {
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> !Send for *const T { }
#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> !Send for *mut T { }

#[stable(feature = "rust1", since = "1.0.0")]
#[lang = "sized"]
#[rustc_on_unimplemented(
    on(parent_trait="std::path::Path", label="borrow the `Path` instead"),
    message="the size for values of type `{Self}` cannot be known at compilation time",
    label="doesn't have a size known at compile-time",
    note="to learn more, visit <https://doc.rust-lang.org/book/\
          ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>",
)]
#[fundamental] // for Default, for example, which requires that `[T]: !Default` be evaluatable
pub trait Sized {
}

#[unstable(feature = "unsize", issue = "27732")]
#[lang = "unsize"]
pub trait Unsize<T: ?Sized> {
}

#[stable(feature = "rust1", since = "1.0.0")]
#[lang = "copy"]
pub trait Copy : Clone {
}

#[rustc_builtin_macro]
#[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
#[allow_internal_unstable(core_intrinsics, derive_clone_copy)]
pub macro Copy($item:item) { /* compiler built-in */ }

#[stable(feature = "rust1", since = "1.0.0")]
#[lang = "sync"]
#[rustc_on_unimplemented(
    message="`{Self}` cannot be shared between threads safely",
    label="`{Self}` cannot be shared between threads safely"
)]
pub unsafe auto trait Sync {

}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> !Sync for *const T { }
#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> !Sync for *mut T { }

#[lang = "phantom_data"]
#[structural_match]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct PhantomData<T:?Sized>;

// impls! { PhantomData }

mod impls {
    #[stable(feature = "rust1", since = "1.0.0")]
    unsafe impl<T: Sync + ?Sized> Send for &T {}
    #[stable(feature = "rust1", since = "1.0.0")]
    unsafe impl<T: Send + ?Sized> Send for &mut T {}
}

#[lang = "freeze"]
pub(crate) unsafe auto trait Freeze {}

unsafe impl<T: ?Sized> Freeze for PhantomData<T> {}
unsafe impl<T: ?Sized> Freeze for *const T {}
unsafe impl<T: ?Sized> Freeze for *mut T {}
unsafe impl<T: ?Sized> Freeze for &T {}
unsafe impl<T: ?Sized> Freeze for &mut T {}

#[stable(feature = "pin", since = "1.33.0")]
#[lang = "unpin"]
pub auto trait Unpin {}

#[stable(feature = "pin", since = "1.33.0")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PhantomPinned;

#[stable(feature = "pin", since = "1.33.0")]
impl !Unpin for PhantomPinned {}

#[stable(feature = "pin", since = "1.33.0")]
impl<'a, T: ?Sized + 'a> Unpin for &'a T {}

#[stable(feature = "pin", since = "1.33.0")]
impl<'a, T: ?Sized + 'a> Unpin for &'a mut T {}

#[stable(feature = "pin_raw", since = "1.38.0")]
impl<T: ?Sized> Unpin for *const T {}

#[stable(feature = "pin_raw", since = "1.38.0")]
impl<T: ?Sized> Unpin for *mut T {}

mod copy_impls {

    use super::Copy;

    macro_rules! impl_copy {
        ($($t:ty)*) => {
            $(
                #[stable(feature = "rust1", since = "1.0.0")]
                impl Copy for $t {}
            )*
        }
    }

    impl_copy! {
        usize u8 u16 u32 u64 u128
        isize i8 i16 i32 i64 i128
        f32 f64
        bool char
    }

    #[unstable(feature = "never_type", issue = "35121")]
    impl Copy for ! {}

    #[stable(feature = "rust1", since = "1.0.0")]
    impl<T: ?Sized> Copy for *const T {}

    #[stable(feature = "rust1", since = "1.0.0")]
    impl<T: ?Sized> Copy for *mut T {}

    #[stable(feature = "rust1", since = "1.0.0")]
    impl<T: ?Sized> Copy for &T {}

}
