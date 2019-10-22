//! # The Rust Core Library

#![cfg(not(test))]

#![doc(html_root_url = "https://doc.rust-lang.org/nightly/",
       html_playground_url = "https://play.rust-lang.org/",
       issue_tracker_base_url = "https://github.com/rust-lang/rust/issues/",
       test(no_crate_inject, attr(deny(warnings))),
       test(attr(allow(dead_code, deprecated, unused_variables, unused_mut))))]
#![no_core]

#![allow(unused_variables, unused_mut, dead_code)]
#![warn(deprecated_in_future)]
#![deny(intra_doc_link_resolution_failure)] // rustdoc is run without -D warnings
#![allow(explicit_outlives_requirements)]
#![allow(incomplete_features)]

#![feature(allow_internal_unstable)]
#![feature(decl_macro)]
#![feature(fundamental)]
#![feature(intrinsics)]
#![feature(lang_items)]
#![feature(no_core)]
#![feature(optin_builtin_traits)]
#![feature(prelude_import)]
#![feature(rustc_attrs)]
#![feature(unboxed_closures)]
#![feature(structural_match)]

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
        #[allow_internal_unstable(core_intrinsics)]
        pub macro Debug($item:item) { /* compiler built-in */ }
    }

    #[allow(missing_debug_implementations)]
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

        // Re-exported core operators
        #[doc(no_inline)]
        pub use crate::marker::{Sized};
        #[doc(no_inline)]
        pub use crate::fmt::macros::Debug;
        #[doc(no_inline)]
        pub use crate::iter::{Iterator, IntoIterator};
        #[doc(no_inline)]
        pub use crate::option::Option::{self, Some, None};
        #[doc(no_inline)]
        pub use crate::result::Result::{self, Ok, Err};
    }
}

pub mod intrinsics {
    extern "rust-intrinsic" {
        pub fn transmute<T, U>(e: T) -> U;
        pub fn size_of<T>() -> usize;
        pub fn min_align_of<T>() -> usize;
        pub fn needs_drop<T>() -> bool;
    }
}

pub mod ptr {
    #[lang = "drop_in_place"]
    #[allow(unconditional_recursion)]
    unsafe fn real_drop_in_place<T: ?Sized>(to_drop: &mut T) { loop { } }
}

pub mod marker {
    #[lang = "copy"]
    pub trait Copy { }

    #[lang = "freeze"]
    pub(crate) unsafe auto trait Freeze {}
    unsafe impl<T: ?Sized> Freeze for &T {}
    unsafe impl<T: ?Sized> Freeze for &mut T {}

    #[lang = "phantom_data"]
    #[structural_match]
    pub struct PhantomData<T:?Sized>;

    pub unsafe auto trait Send { }

    #[lang = "sized"]
    #[fundamental] // for Default, for example, which requires that `[T]: !Default` be evaluatable
    pub trait Sized { }

    #[lang = "sync"]
    pub unsafe auto trait Sync { }

    #[lang = "unsize"]
    pub trait Unsize<T: ?Sized> { }
}



pub mod any {
    use crate::fmt;

    pub trait Any: 'static {
        fn type_id(&self) -> TypeId;
    }

    impl fmt::Debug for dyn Any {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
    }

    pub struct TypeId {

    }
}

pub mod panicking {
    #![allow(dead_code, missing_docs)]

    use crate::fmt;

    #[cold]
    #[cfg_attr(not(feature="panic_immediate_abort"),inline(never))]
    #[lang = "panic"]
    pub fn panic(expr_file_line_col: &(&'static str, &'static str, u32, u32)) -> ! { loop { } }

    #[cold]
    #[cfg_attr(not(feature="panic_immediate_abort"),inline(never))]
    #[lang = "panic_bounds_check"]
    fn panic_bounds_check(file_line_col: &(&'static str, u32, u32),
                          index: usize, len: usize) -> ! { loop { } }

    #[cold]
    #[cfg_attr(not(feature="panic_immediate_abort"),inline(never))]
    #[cfg_attr(    feature="panic_immediate_abort" ,inline)]
    pub fn panic_fmt(fmt: fmt::Arguments<'_>, file_line_col: &(&'static str, u32, u32)) -> ! { loop { } }
}

pub mod iter {
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }

    pub trait IntoIterator {
        type Item;

        type IntoIter: Iterator<Item=Self::Item>;

        fn into_iter(self) -> Self::IntoIter;
    }
}
pub mod option {
    pub enum Option<T> {
        None,
        Some(T),
    }
}

pub mod result {
    pub enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}
