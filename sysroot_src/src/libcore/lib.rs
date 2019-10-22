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
#![feature(asm)]
#![feature(cfg_target_has_atomic)]
#![feature(concat_idents)]
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
#![feature(const_int_conversion)]
#![feature(const_transmute)]
#![feature(non_exhaustive)]
#![feature(structural_match)]
#![feature(abi_unadjusted)]
#![feature(adx_target_feature)]
#![feature(maybe_uninit_slice)]
#![feature(external_doc)]
#![feature(associated_type_bounds)]

#[prelude_import]
#[allow(unused)]
use prelude::v1::*;

#[macro_use]
mod macros;

#[macro_use]
mod internal_macros;

#[stable(feature = "rust1", since = "1.0.0")] pub mod isize { }
#[stable(feature = "rust1", since = "1.0.0")] pub mod i8 { }
#[stable(feature = "rust1", since = "1.0.0")] pub mod i16 { }
#[stable(feature = "rust1", since = "1.0.0")] pub mod i32 { }
#[stable(feature = "rust1", since = "1.0.0")] pub mod i64 { }
#[stable(feature = "rust1", since = "1.0.0")] pub mod i128 { }

#[stable(feature = "rust1", since = "1.0.0")] pub mod usize { }
#[stable(feature = "rust1", since = "1.0.0")] pub mod u8 { }
#[stable(feature = "rust1", since = "1.0.0")] pub mod u16 { }
#[stable(feature = "rust1", since = "1.0.0")] pub mod u32 { }
#[stable(feature = "rust1", since = "1.0.0")] pub mod u64 { }
#[stable(feature = "rust1", since = "1.0.0")] pub mod u128 { }

#[stable(feature = "rust1", since = "1.0.0")] pub mod f32 { }
#[stable(feature = "rust1", since = "1.0.0")] pub mod f64 { }

#[macro_use]
pub mod num;

/* The libcore prelude, not as all-encompassing as the libstd prelude */

pub mod prelude;

/* Core modules for ownership management */

pub mod intrinsics;
pub mod mem;
pub mod ptr {
    #![stable(feature = "rust1", since = "1.0.0")]

    #[stable(feature = "nonnull", since = "1.25.0")]
    #[derive(Debug)]
    pub struct NonNull<X: ?Sized>(*const X);

    #[lang = "drop_in_place"]
    #[allow(unconditional_recursion)]
    unsafe fn real_drop_in_place<T: ?Sized>(to_drop: &mut T) { loop { } }
}
// pub mod hint;

/* Core language traits */

pub mod marker {
    #![stable(feature = "rust1", since = "1.0.0")]

    #[stable(feature = "rust1", since = "1.0.0")]
    #[lang = "copy"]
    pub trait Copy { }

    #[lang = "freeze"]
    pub(crate) unsafe auto trait Freeze {}
    unsafe impl<T: ?Sized> Freeze for PhantomData<T> {}
    unsafe impl<T: ?Sized> Freeze for *const T {}
    unsafe impl<T: ?Sized> Freeze for *mut T {}
    unsafe impl<T: ?Sized> Freeze for &T {}
    unsafe impl<T: ?Sized> Freeze for &mut T {}

    #[lang = "phantom_data"]
    #[structural_match]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub struct PhantomData<T:?Sized>;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub unsafe auto trait Send { }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl<T: ?Sized> !Send for *const T { }
    #[stable(feature = "rust1", since = "1.0.0")]
    impl<T: ?Sized> !Send for *mut T { }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[lang = "sized"]
    #[fundamental] // for Default, for example, which requires that `[T]: !Default` be evaluatable
    pub trait Sized { }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[lang = "sync"]
    pub unsafe auto trait Sync { }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl<T: ?Sized> !Sync for *const T { }
    #[stable(feature = "rust1", since = "1.0.0")]
    impl<T: ?Sized> !Sync for *mut T { }

    #[unstable(feature = "unsize", issue = "27732")]
    #[lang = "unsize"]
    pub trait Unsize<T: ?Sized> { }


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
}

pub mod ops;
// pub mod cmp;
// pub mod clone;
// pub mod default;
// pub mod convert;
// pub mod borrow;

/* Core types and methods on primitives */

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
        t: u64,
    }
}

// pub mod array;
pub mod panic;
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

pub mod fmt;
pub mod alloc;
