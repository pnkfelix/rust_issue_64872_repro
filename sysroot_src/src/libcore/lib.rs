#![no_core]

#![allow(unused_variables, unused_mut, dead_code)]

#![feature(decl_macro)]
#![feature(fundamental)]
#![feature(lang_items)]
#![feature(no_core)]
#![feature(optin_builtin_traits)]
#![feature(prelude_import)]
#![feature(rustc_attrs)]
#![feature(unboxed_closures)]

#[prelude_import]
#[allow(unused)]
use prelude::v1::*;

pub mod ops {
    mod deref {
        #[lang = "receiver"]
        #[doc(hidden)]
        pub trait Receiver { }

        impl<T: ?Sized> Receiver for &T {}

        impl<T: ?Sized> Receiver for &mut T {}
    }

    mod function {
        #[lang = "fn"]
        #[rustc_paren_sugar]
        #[fundamental] // so that regex can rely that `&str: !FnMut`
        #[must_use = "closures are lazy and do nothing unless called"]
        pub trait Fn<Args> : FnMut<Args> {
            extern "rust-call" fn call(&self, args: Args) -> Self::Output;
        }

        #[lang = "fn_mut"]
        #[rustc_paren_sugar]
        #[fundamental] // so that regex can rely that `&str: !FnMut`
        #[must_use = "closures are lazy and do nothing unless called"]
        pub trait FnMut<Args> : FnOnce<Args> {
            extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
        }

        #[lang = "fn_once"]
        #[rustc_paren_sugar]
        #[fundamental] // so that regex can rely that `&str: !FnMut`
        #[must_use = "closures are lazy and do nothing unless called"]
        pub trait FnOnce<Args> {
            type Output;

            extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
        }
    }
    mod unsize {
        use crate::marker::Unsize;

        #[lang = "coerce_unsized"]
        pub trait CoerceUnsized<T: ?Sized> { }

        impl<'a, T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<&'a mut U> for &'a mut T {}
        impl<'a, 'b: 'a, T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<&'a U> for &'b mut T {}
        impl<'a, T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<*mut U> for &'a mut T {}
        impl<'a, T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<*const U> for &'a mut T {}

        impl<'a, 'b: 'a, T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<&'a U> for &'b T {}
        impl<'a, T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<*const U> for &'a T {}

        impl<T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<*mut U> for *mut T {}
        impl<T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<*const U> for *mut T {}

        impl<T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<*const U> for *const T {}

        #[lang = "dispatch_from_dyn"]
        pub trait DispatchFromDyn<T> { }

        impl<'a, T: ?Sized+Unsize<U>, U: ?Sized> DispatchFromDyn<&'a U> for &'a T {}
        impl<'a, T: ?Sized+Unsize<U>, U: ?Sized> DispatchFromDyn<&'a mut U> for &'a mut T {}
        impl<T: ?Sized+Unsize<U>, U: ?Sized> DispatchFromDyn<*const U> for *const T {}
        impl<T: ?Sized+Unsize<U>, U: ?Sized> DispatchFromDyn<*mut U> for *mut T {}
    }

    pub use self::function::{Fn, FnMut, FnOnce};
}

#[derive(Debug)]
struct UnusedWithFieldOfTypeU32 {
    inner: u32,
}

pub mod fmt
{

    pub trait Debug {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result;
    }


    impl<T: ?Sized + Debug> Debug for &T {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
    }

    impl Debug for u32 {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
    }

    pub(crate) mod macros {
        #[rustc_builtin_macro]
        pub macro Debug($item:item) { /* compiler built-in */ }
    }

    pub struct Formatter<'a> {
        inner: &'a (),
    }

    pub struct DebugTuple<'a, 'b: 'a> { inner: &'a &'b () }

    impl<'a, 'b: 'a> DebugTuple<'a, 'b> {
        pub fn field(&mut self, _value: &dyn Debug) -> &mut DebugTuple<'a, 'b> { loop { } }

        pub fn finish(&mut self) -> Result { loop { } }
    }

    impl<'a, 'b: 'a> DebugStruct<'a, 'b> {
        pub fn field(&mut self, name: &str, value: &dyn Debug) -> &mut DebugStruct<'a, 'b> { loop { } }

        pub fn finish(&mut self) -> Result { loop { } }
    }

    pub struct DebugStruct<'a, 'b: 'a> { inner: &'a &'b () }

    impl<'a> Formatter<'a> {
        pub fn debug_tuple<'b>(&'b mut self, _name: &str) -> DebugTuple<'b, 'a> { loop { } }

        pub fn debug_struct<'b>(&'b mut self, name: &str) -> DebugStruct<'b, 'a> { loop { } }
    }

    pub type Result = crate::result::Result<(), Error>;

    pub struct Arguments<'a> {
        inner: &'a (),
    }

    pub trait Display {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result;
    }
    pub struct Error;
}

pub mod prelude {
    pub mod v1 {
        pub use crate::marker::{Sized};
        pub use crate::fmt::macros::Debug;
        pub use crate::result::Result::{self, Ok, Err};
    }
}

#[lang = "drop_in_place"]
unsafe fn real_drop_in_place<T: ?Sized>(to_drop: &mut T) { loop { } }

pub mod marker {
    #[lang = "copy"]
    pub trait Copy { }

    #[lang = "freeze"]
    pub(crate) unsafe auto trait Freeze {}
    unsafe impl<T: ?Sized> Freeze for &T {}
    unsafe impl<T: ?Sized> Freeze for &mut T {}

    #[lang = "sized"]
    #[fundamental] // for Default, for example, which requires that `[T]: !Default` be evaluatable
    pub trait Sized { }

    #[lang = "unsize"]
    pub trait Unsize<T: ?Sized> { }
}

pub mod result {
    pub enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}
