//! # The Rust Core Library

#![cfg(not(test))]

#![stable(feature = "core", since = "1.6.0")]
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
#![feature(arbitrary_self_types)]
#![feature(cfg_target_has_atomic)]
#![feature(const_fn)]
#![feature(const_fn_union)]
#![feature(const_generics)]
#![feature(custom_inner_attributes)]
#![feature(decl_macro)]
#![feature(doc_cfg)]
#![feature(doc_spotlight)]
#![feature(extern_types)]
#![feature(fundamental)]
#![feature(intrinsics)]
#![feature(lang_items)]
#![feature(link_llvm_intrinsics)]
#![feature(never_type)]
#![feature(nll)]
#![feature(exhaustive_patterns)]
#![feature(no_core)]
#![feature(on_unimplemented)]
#![feature(optin_builtin_traits)]
#![feature(prelude_import)]
#![feature(repr_simd, platform_intrinsics)]
#![feature(rustc_attrs)]
#![feature(rustc_const_unstable)]
#![feature(simd_ffi)]
#![feature(specialization)]
#![feature(staged_api)]
#![feature(std_internals)]
#![feature(stmt_expr_attributes)]
#![feature(transparent_unions)]
#![feature(unboxed_closures)]
#![feature(unsized_locals)]
#![feature(untagged_unions)]
#![feature(unwind_attributes)]
#![feature(doc_alias)]
#![feature(mmx_target_feature)]
#![feature(tbm_target_feature)]
#![feature(sse4a_target_feature)]
#![feature(arm_target_feature)]
#![feature(powerpc_target_feature)]
#![feature(mips_target_feature)]
#![feature(aarch64_target_feature)]
#![feature(wasm_target_feature)]
#![feature(avx512_target_feature)]
#![feature(cmpxchg16b_target_feature)]
#![feature(rtm_target_feature)]
#![feature(f16c_target_feature)]
#![feature(hexagon_target_feature)]
#![feature(const_transmute)]
#![feature(non_exhaustive)]
#![feature(structural_match)]
#![feature(abi_unadjusted)]
#![feature(adx_target_feature)]
#![feature(external_doc)]
#![feature(associated_type_bounds)]

#[prelude_import]
#[allow(unused)]
use prelude::v1::*;

pub mod ops;
pub mod panic;
pub mod fmt
{
    #![stable(feature = "rust1", since = "1.0.0")]

    #[stable(feature = "rust1", since = "1.0.0")]
    #[doc(alias = "{:?}")]
    #[rustc_diagnostic_item = "debug_trait"]
    pub trait Debug {
        #[stable(feature = "rust1", since = "1.0.0")]
        fn fmt(&self, f: &mut Formatter<'_>) -> Result;
    }


    #[stable(feature = "rust1", since = "1.0.0")]
    impl<T: ?Sized + Debug> Debug for &T {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
    }
    #[stable(feature = "rust1", since = "1.0.0")]
    impl<T: ?Sized + Debug> Debug for &mut T {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl Debug for usize {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl Debug for u32 {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl Debug for Arguments<'_> {
        fn fmt(&self, fmt: &mut Formatter<'_>) -> Result { loop { } }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl Debug for str {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
    }

    pub(crate) mod macros {
        #[rustc_builtin_macro]
        #[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
        #[allow_internal_unstable(core_intrinsics)]
        pub macro Debug($item:item) { /* compiler built-in */ }
    }

    #[allow(missing_debug_implementations)]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub struct Formatter<'a> {
        inner: &'a (),
    }

    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub struct DebugTuple<'a, 'b: 'a> { inner: &'a &'b () }
    impl<'a, 'b: 'a> DebugTuple<'a, 'b> {
        #[stable(feature = "debug_builders", since = "1.2.0")]
        pub fn field(&mut self, _value: &dyn Debug) -> &mut DebugTuple<'a, 'b> { loop { } }

        #[stable(feature = "debug_builders", since = "1.2.0")]
        pub fn finish(&mut self) -> Result { loop { } }

        fn is_pretty(&self) -> bool { loop { } }
    }

    impl<'a, 'b: 'a> DebugStruct<'a, 'b> {
        #[stable(feature = "debug_builders", since = "1.2.0")]
        pub fn field(&mut self, name: &str, value: &dyn Debug) -> &mut DebugStruct<'a, 'b> { loop { } }

        #[stable(feature = "debug_builders", since = "1.2.0")]
        pub fn finish(&mut self) -> Result { loop { } }

        fn is_pretty(&self) -> bool { loop { } }
    }

    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub struct DebugStruct<'a, 'b: 'a> { inner: &'a &'b () }

    impl<'a> Formatter<'a> {
        #[stable(feature = "debug_builders", since = "1.2.0")]
        pub fn debug_tuple<'b>(&'b mut self, _name: &str) -> DebugTuple<'b, 'a> { loop { } }
        #[stable(feature = "debug_builders", since = "1.2.0")]
        pub fn debug_struct<'b>(&'b mut self, name: &str) -> DebugStruct<'b, 'a> { loop { } }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub type Result = crate::result::Result<(), Error>;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub struct Arguments<'a> {
        inner: &'a (),
    }

    #[doc(alias = "{}")]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub trait Display {
        #[stable(feature = "rust1", since = "1.0.0")]
        fn fmt(&self, f: &mut Formatter<'_>) -> Result;
    }
    #[stable(feature = "rust1", since = "1.0.0")]
    #[derive(Debug)]
    pub struct Error;
}

#[stable(feature = "rust1", since = "1.0.0")] pub mod usize { }
pub mod prelude {
    #![stable(feature = "core_prelude", since = "1.4.0")]
    pub mod v1 {
        #![stable(feature = "core_prelude", since = "1.4.0")]

        // Re-exported core operators
        #[stable(feature = "core_prelude", since = "1.4.0")]
        #[doc(no_inline)]
        pub use crate::marker::{Sized};
        #[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
        #[doc(no_inline)]
        pub use crate::fmt::macros::Debug;
        #[stable(feature = "core_prelude", since = "1.4.0")]
        #[doc(no_inline)]
        pub use crate::iter::{Iterator, IntoIterator};
        #[stable(feature = "core_prelude", since = "1.4.0")]
        #[doc(no_inline)]
        pub use crate::option::Option::{self, Some, None};
        #[stable(feature = "core_prelude", since = "1.4.0")]
        #[doc(no_inline)]
        pub use crate::result::Result::{self, Ok, Err};
        #[stable(feature = "core_prelude", since = "1.4.0")]
        #[doc(no_inline)]
        pub use crate::ops::{Fn, FnMut, FnOnce};
    }
}

pub mod intrinsics {
    #![unstable(feature = "core_intrinsics",
                reason = "intrinsics are unlikely to ever be stabilized, instead \
                          they should be used through stabilized interfaces \
                          in the rest of the standard library",
                issue = "0")]
    extern "rust-intrinsic" {
        #[stable(feature = "rust1", since = "1.0.0")]
        pub fn transmute<T, U>(e: T) -> U;
        pub fn size_of<T>() -> usize;
        pub fn min_align_of<T>() -> usize;
        pub fn needs_drop<T>() -> bool;
    }
}

pub mod ptr {
    #![stable(feature = "rust1", since = "1.0.0")]

    #[lang = "drop_in_place"]
    #[allow(unconditional_recursion)]
    unsafe fn real_drop_in_place<T: ?Sized>(to_drop: &mut T) { loop { } }
}

pub mod marker {
    #![stable(feature = "rust1", since = "1.0.0")]

    #[stable(feature = "rust1", since = "1.0.0")]
    #[lang = "copy"]
    pub trait Copy { }

    #[lang = "freeze"]
    pub(crate) unsafe auto trait Freeze {}
    unsafe impl<T: ?Sized> Freeze for &T {}
    unsafe impl<T: ?Sized> Freeze for &mut T {}

    #[lang = "phantom_data"]
    #[structural_match]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub struct PhantomData<T:?Sized>;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub unsafe auto trait Send { }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[lang = "sized"]
    #[fundamental] // for Default, for example, which requires that `[T]: !Default` be evaluatable
    pub trait Sized { }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[lang = "sync"]
    pub unsafe auto trait Sync { }

    #[unstable(feature = "unsize", issue = "27732")]
    #[lang = "unsize"]
    pub trait Unsize<T: ?Sized> { }
}



pub mod any {
    #![stable(feature = "rust1", since = "1.0.0")]
    use crate::fmt;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub trait Any: 'static {
        #[stable(feature = "get_type_id", since = "1.34.0")]
        fn type_id(&self) -> TypeId;
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl fmt::Debug for dyn Any {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
    }

    #[derive(Debug)]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub struct TypeId {

    }
}

pub mod panicking {
    #![allow(dead_code, missing_docs)]
    #![unstable(feature = "core_panic",
                reason = "internal details of the implementation of the `panic!` \
                          and related macros",
                issue = "0")]

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
    #![stable(feature = "rust1", since = "1.0.0")]

    #[stable(feature = "rust1", since = "1.0.0")]
    pub trait Iterator {
        #[stable(feature = "rust1", since = "1.0.0")]
        type Item;

        #[stable(feature = "rust1", since = "1.0.0")]
        fn next(&mut self) -> Option<Self::Item>;
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub trait IntoIterator {
        #[stable(feature = "rust1", since = "1.0.0")]
        type Item;

        #[stable(feature = "rust1", since = "1.0.0")]
        type IntoIter: Iterator<Item=Self::Item>;

        #[stable(feature = "rust1", since = "1.0.0")]
        fn into_iter(self) -> Self::IntoIter;
    }
}
pub mod option {
    #![stable(feature = "rust1", since = "1.0.0")]
    #[derive(Debug)]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub enum Option<T> {
        #[stable(feature = "rust1", since = "1.0.0")]
        None,
        #[stable(feature = "rust1", since = "1.0.0")]
        Some(#[stable(feature = "rust1", since = "1.0.0")] T),
    }
}

pub mod result {
    #![stable(feature = "rust1", since = "1.0.0")]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub enum Result<T, E> {
        #[stable(feature = "rust1", since = "1.0.0")]
        Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
        #[stable(feature = "rust1", since = "1.0.0")]
        Err(#[stable(feature = "rust1", since = "1.0.0")] E),
    }
}

pub mod alloc {
    #![stable(feature = "alloc_module", since = "1.28.0")]

    #[stable(feature = "alloc_layout", since = "1.28.0")]
    #[derive(Debug)]
    #[lang = "alloc_layout"]
    pub struct Layout {
        size_: usize,
    }
}
