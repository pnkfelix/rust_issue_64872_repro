#![no_core]

#![feature(decl_macro)]
#![feature(lang_items)]
#![feature(no_core)]
#![feature(rustc_attrs)]
#![feature(unboxed_closures)]

struct UnusedWithFieldOfTypeU32 {
    inner: u32,
}

impl crate::fmt::Debug for UnusedWithFieldOfTypeU32 {
    fn fmt(&self, f: &mut crate::fmt::Formatter) -> crate::fmt::Result {
        match self {
            UnusedWithFieldOfTypeU32 { inner: ref u } => {
                let mut debug_trait_builder: crate::fmt::DebugStruct = f.debug_struct("UnusedWithFieldOfTypeU32");
                debug_trait_builder.field("inner", &u);
                loop { }
            }
        }
    }
}

#[lang = "receiver"]
pub trait Receiver { }

impl<T: ?Sized> Receiver for &T {}

impl<T: ?Sized> Receiver for &mut T {}

#[lang = "fn"]
#[must_use = "closures are lazy and do nothing unless called"]
pub trait Fn<Args> {
    type Output;
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}

#[lang = "coerce_unsized"]
pub trait CoerceUnsized<T: ?Sized> { }

impl<'a, T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<&'a mut U> for &'a mut T {}
impl<'a, 'b: 'a, T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<&'a U> for &'b mut T {}

impl<'a, 'b: 'a, T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<&'a U> for &'b T {}
impl<'a, T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<*const U> for &'a T {}

#[lang = "dispatch_from_dyn"]
pub trait DispatchFromDyn<T> { }

impl<'a, T: ?Sized+Unsize<U>, U: ?Sized> DispatchFromDyn<&'a U> for &'a T {}
impl<'a, T: ?Sized+Unsize<U>, U: ?Sized> DispatchFromDyn<&'a mut U> for &'a mut T {}

pub mod fmt
{
    use crate::Sized;

    pub trait Debug {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result;
    }

    impl<T: ?Sized + Debug> Debug for &T {
        fn fmt(&self, _: &mut Formatter<'_>) -> Result { loop { } }
    }

    impl Debug for u32 {
        fn fmt(&self, _: &mut Formatter<'_>) -> Result { loop { } }
    }

    impl Debug for () {
        fn fmt(&self, _: &mut Formatter<'_>) -> Result { loop { } }
    }

    pub struct Formatter<'a> { _inner: &'a () }

    pub struct DebugTuple<'a, 'b: 'a> { _inner: &'a &'b () }

    impl<'a, 'b: 'a> DebugTuple<'a, 'b> {
        pub fn field(&mut self, _value: &dyn Debug) -> &mut DebugTuple<'a, 'b> { loop { } }

        pub fn finish(&mut self) -> Result { loop { } }
    }

    impl<'a, 'b: 'a> DebugStruct<'a, 'b> {
        pub fn field(&mut self, _: &str, _: &dyn Debug) -> &mut DebugStruct<'a, 'b> { loop { } }

        pub fn finish(&mut self) -> Result { loop { } }
    }

    pub struct DebugStruct<'a, 'b: 'a> { _inner: &'a &'b () }

    impl<'a> Formatter<'a> {
        pub fn debug_tuple<'b>(&'b mut self, _: &str) -> DebugTuple<'b, 'a> { loop { } }

        pub fn debug_struct<'b>(&'b mut self, _: &str) -> DebugStruct<'b, 'a> { loop { } }
    }

    pub type Result = crate::Result<(), Error>;

    pub struct Arguments<'a> { _inner: &'a () }

    pub struct Error;
}

pub mod prelude {
    pub mod v1 {
        pub use crate::{Sized};
    }
}

#[lang = "drop_in_place"]
unsafe fn real_drop_in_place<T: ?Sized>(_: &mut T) { loop { } }

#[lang = "copy"]
pub trait Copy { }

#[lang = "freeze"]
pub /*unsafe auto*/ trait Freeze {}

#[lang = "sized"]
pub trait Sized { }

#[lang = "unsize"]
pub trait Unsize<T: ?Sized> { }

pub enum Result<T, E> { Ok(T), Err(E) }
