#![no_core]

#![feature(decl_macro)]
#![feature(lang_items)]
#![feature(no_core)]
#![feature(rustc_attrs)]
#![feature(unboxed_closures)]

pub trait Object { fn method(&self) { } }

impl Object for u32 { }
impl Object for () { }
impl<T> Object for &T { }

pub fn unused() {
    let ref u = 0_u32;
    let _d = &u as &dyn crate::Object;
    loop { }
}

#[lang = "receiver"]
pub trait Receiver { }

impl<T: ?Sized> Receiver for &T {}

impl<T: ?Sized> Receiver for &mut T {}

#[lang = "fn"]
pub trait Fn<Args> {
    type Output;
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}

#[lang = "coerce_unsized"]
pub trait CoerceUnsized<T: ?Sized> { }

impl<'a, 'b: 'a, T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<&'a U> for &'b T {}

#[lang = "dispatch_from_dyn"]
pub trait DispatchFromDyn<T> { }

impl<'a, T: ?Sized+Unsize<U>, U: ?Sized> DispatchFromDyn<&'a U> for &'a T {}
impl<'a, T: ?Sized+Unsize<U>, U: ?Sized> DispatchFromDyn<&'a mut U> for &'a mut T {}

pub mod prelude { pub mod v1 { } }

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
