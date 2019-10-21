//! Manually manage memory through raw pointers.

#![stable(feature = "rust1", since = "1.0.0")]

use crate::fmt;
use crate::hash;
use crate::cmp::Ordering::{self};

#[stable(feature = "rust1", since = "1.0.0")]
pub use crate::intrinsics::copy_nonoverlapping;

#[stable(feature = "rust1", since = "1.0.0")]
pub use crate::intrinsics::copy;

#[stable(feature = "rust1", since = "1.0.0")]
pub use crate::intrinsics::write_bytes;

mod non_null;
#[stable(feature = "nonnull", since = "1.25.0")]
pub use non_null::NonNull;

mod unique;
#[unstable(feature = "ptr_internals", issue = "0")]
pub use unique::Unique;

#[stable(feature = "drop_in_place", since = "1.8.0")]
#[inline(always)]
pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) { loop { } }

#[lang = "drop_in_place"]
#[allow(unconditional_recursion)]
unsafe fn real_drop_in_place<T: ?Sized>(to_drop: &mut T) { loop { } }

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_promotable]
pub const fn null<T>() -> *const T { 0 as *const T }

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_promotable]
pub const fn null_mut<T>() -> *mut T { 0 as *mut T }

#[repr(C)]
pub(crate) union Repr<T> {
    pub(crate) rust: *const [T],
    rust_mut: *mut [T],
    pub(crate) raw: FatPtr<T>,
}

#[repr(C)]
pub(crate) struct FatPtr<T> {
    data: *const T,
    pub(crate) len: usize,
}

#[inline]
#[unstable(feature = "slice_from_raw_parts", reason = "recently added", issue = "36925")]
pub fn slice_from_raw_parts<T>(data: *const T, len: usize) -> *const [T] { loop { } }

#[inline]
#[unstable(feature = "slice_from_raw_parts", reason = "recently added", issue = "36925")]
pub fn slice_from_raw_parts_mut<T>(data: *mut T, len: usize) -> *mut [T] { loop { } }

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
pub unsafe fn swap<T>(x: *mut T, y: *mut T) { loop { } }

#[inline]
#[stable(feature = "swap_nonoverlapping", since = "1.27.0")]
pub unsafe fn swap_nonoverlapping<T>(x: *mut T, y: *mut T, count: usize) { loop { } }

#[inline]
pub(crate) unsafe fn swap_nonoverlapping_one<T>(x: *mut T, y: *mut T) { loop { } }

#[inline]
unsafe fn swap_nonoverlapping_bytes(x: *mut u8, y: *mut u8, len: usize) { loop { } }

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
pub unsafe fn replace<T>(dst: *mut T, mut src: T) -> T { loop { } }

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
pub unsafe fn read<T>(src: *const T) -> T { loop { } }

#[inline]
#[stable(feature = "ptr_unaligned", since = "1.17.0")]
pub unsafe fn read_unaligned<T>(src: *const T) -> T { loop { } }

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
pub unsafe fn write<T>(dst: *mut T, src: T) { loop { } }

#[inline]
#[stable(feature = "ptr_unaligned", since = "1.17.0")]
pub unsafe fn write_unaligned<T>(dst: *mut T, src: T) { loop { } }

#[inline]
#[stable(feature = "volatile", since = "1.9.0")]
pub unsafe fn read_volatile<T>(src: *const T) -> T { loop { } }

#[inline]
#[stable(feature = "volatile", since = "1.9.0")]
pub unsafe fn write_volatile<T>(dst: *mut T, src: T) { loop { } }

#[lang = "const_ptr"]
impl<T: ?Sized> *const T {
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn is_null(self) -> bool { loop { } }

    #[stable(feature = "ptr_cast", since = "1.38.0")]
    #[inline]
    pub const fn cast<U>(self) -> *const U {
        self as _
    }

    #[stable(feature = "ptr_as_ref", since = "1.9.0")]
    #[inline]
    pub unsafe fn as_ref<'a>(self) -> Option<&'a T> { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub unsafe fn offset(self, count: isize) -> *const T where T: Sized { loop { } }

    #[stable(feature = "ptr_wrapping_offset", since = "1.16.0")]
    #[inline]
    pub fn wrapping_offset(self, count: isize) -> *const T where T: Sized { loop { } }

    #[unstable(feature = "ptr_offset_from", issue = "41079")]
    #[inline]
        pub unsafe fn offset_from(self, origin: *const T) -> isize where T: Sized { loop { } }

    #[unstable(feature = "ptr_wrapping_offset_from", issue = "41079")]
    #[inline]
    pub fn wrapping_offset_from(self, origin: *const T) -> isize where T: Sized { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub unsafe fn add(self, count: usize) -> Self
        where T: Sized,
    { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub unsafe fn sub(self, count: usize) -> Self
        where T: Sized,
    { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub fn wrapping_add(self, count: usize) -> Self
        where T: Sized,
    { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub fn wrapping_sub(self, count: usize) -> Self
        where T: Sized,
    { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub unsafe fn read(self) -> T
        where T: Sized,
    { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub unsafe fn read_volatile(self) -> T
        where T: Sized,
    { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub unsafe fn read_unaligned(self) -> T
        where T: Sized,
    { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub unsafe fn copy_to(self, dest: *mut T, count: usize)
        where T: Sized,
    { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub unsafe fn copy_to_nonoverlapping(self, dest: *mut T, count: usize)
        where T: Sized,
    { loop { } }

    #[stable(feature = "align_offset", since = "1.36.0")]
    pub fn align_offset(self, align: usize) -> usize where T: Sized { loop { } }
}


#[lang = "mut_ptr"]
impl<T: ?Sized> *mut T {
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn is_null(self) -> bool { loop { } }

    #[stable(feature = "ptr_cast", since = "1.38.0")]
    pub const fn cast<U>(self) -> *mut U { self as _ }

    #[stable(feature = "ptr_as_ref", since = "1.9.0")]
    pub unsafe fn as_ref<'a>(self) -> Option<&'a T> { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub unsafe fn offset(self, count: isize) -> *mut T where T: Sized
    { loop { } }

    #[stable(feature = "ptr_wrapping_offset", since = "1.16.0")]
    pub fn wrapping_offset(self, count: isize) -> *mut T where T: Sized
    { loop { } }

    #[stable(feature = "ptr_as_ref", since = "1.9.0")]
    pub unsafe fn as_mut<'a>(self) -> Option<&'a mut T> { loop { } }

    #[unstable(feature = "ptr_offset_from", issue = "41079")]
    #[inline]
    pub unsafe fn offset_from(self, origin: *const T) -> isize where T: Sized { loop { } }

    #[unstable(feature = "ptr_wrapping_offset_from", issue = "41079")]
    #[inline]
    pub fn wrapping_offset_from(self, origin: *const T) -> isize where T: Sized { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub unsafe fn add(self, count: usize) -> Self
        where T: Sized,
    { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub unsafe fn sub(self, count: usize) -> Self
        where T: Sized,
    { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub fn wrapping_add(self, count: usize) -> Self
        where T: Sized,
    { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub fn wrapping_sub(self, count: usize) -> Self
        where T: Sized,
    { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub unsafe fn read(self) -> T
        where T: Sized,
    { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub unsafe fn read_volatile(self) -> T
        where T: Sized,
    { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub unsafe fn read_unaligned(self) -> T
        where T: Sized,
    { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub unsafe fn copy_to(self, dest: *mut T, count: usize)
        where T: Sized,
    { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub unsafe fn copy_to_nonoverlapping(self, dest: *mut T, count: usize)
        where T: Sized,
    { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub unsafe fn copy_from(self, src: *const T, count: usize)
        where T: Sized,
    { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub unsafe fn copy_from_nonoverlapping(self, src: *const T, count: usize)
        where T: Sized,
    { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
        pub unsafe fn drop_in_place(self) { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub unsafe fn write(self, val: T)
        where T: Sized,
    { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub unsafe fn write_bytes(self, val: u8, count: usize)
        where T: Sized,
    { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub unsafe fn write_volatile(self, val: T)
        where T: Sized,
    { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub unsafe fn write_unaligned(self, val: T)
        where T: Sized,
    { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub unsafe fn replace(self, src: T) -> T
        where T: Sized,
    { loop { } }

    #[stable(feature = "pointer_methods", since = "1.26.0")]
    #[inline]
    pub unsafe fn swap(self, with: *mut T)
        where T: Sized,
    { loop { } }

    #[stable(feature = "align_offset", since = "1.36.0")]
    pub fn align_offset(self, align: usize) -> usize where T: Sized { loop { } }
}

#[lang="align_offset"]
pub(crate) unsafe fn align_offset<T: Sized>(p: *const T, a: usize) -> usize { loop { } }



#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> PartialEq for *const T {
    #[inline]
    fn eq(&self, other: &*const T) -> bool { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Eq for *const T {}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> PartialEq for *mut T {
    #[inline]
    fn eq(&self, other: &*mut T) -> bool { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Eq for *mut T {}

#[stable(feature = "ptr_eq", since = "1.17.0")]
#[inline]
pub fn eq<T: ?Sized>(a: *const T, b: *const T) -> bool { loop { } }

#[stable(feature = "ptr_hash", since = "1.35.0")]
pub fn hash<T: ?Sized, S: hash::Hasher>(hashee: *const T, into: &mut S) { loop { } }

macro_rules! fnptr_impls_safety_abi {
    ($FnTy: ty, $($Arg: ident),*) => {
        #[stable(feature = "fnptr_impls", since = "1.4.0")]
        impl<Ret, $($Arg),*> PartialEq for $FnTy {
            #[inline]
            fn eq(&self, other: &Self) -> bool { loop { } }
        }

        #[stable(feature = "fnptr_impls", since = "1.4.0")]
        impl<Ret, $($Arg),*> Eq for $FnTy {}

        #[stable(feature = "fnptr_impls", since = "1.4.0")]
        impl<Ret, $($Arg),*> PartialOrd for $FnTy {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> { loop { } }
        }

        #[stable(feature = "fnptr_impls", since = "1.4.0")]
        impl<Ret, $($Arg),*> Ord for $FnTy {
            #[inline]
            fn cmp(&self, other: &Self) -> Ordering { loop { } }
        }

        #[stable(feature = "fnptr_impls", since = "1.4.0")]
        impl<Ret, $($Arg),*> hash::Hash for $FnTy {
            fn hash<HH: hash::Hasher>(&self, state: &mut HH) { loop { } }
        }

        #[stable(feature = "fnptr_impls", since = "1.4.0")]
        impl<Ret, $($Arg),*> fmt::Pointer for $FnTy {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
        }

        #[stable(feature = "fnptr_impls", since = "1.4.0")]
        impl<Ret, $($Arg),*> fmt::Debug for $FnTy {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
        }
    }
}

macro_rules! fnptr_impls_args {
    ($($Arg: ident),+) => {
        fnptr_impls_safety_abi! { extern "Rust" fn($($Arg),+) -> Ret, $($Arg),+ }
        fnptr_impls_safety_abi! { extern "C" fn($($Arg),+) -> Ret, $($Arg),+ }
        fnptr_impls_safety_abi! { extern "C" fn($($Arg),+ , ...) -> Ret, $($Arg),+ }
        fnptr_impls_safety_abi! { unsafe extern "Rust" fn($($Arg),+) -> Ret, $($Arg),+ }
        fnptr_impls_safety_abi! { unsafe extern "C" fn($($Arg),+) -> Ret, $($Arg),+ }
        fnptr_impls_safety_abi! { unsafe extern "C" fn($($Arg),+ , ...) -> Ret, $($Arg),+ }
    };
    () => {
        fnptr_impls_safety_abi! { extern "Rust" fn() -> Ret, }
        fnptr_impls_safety_abi! { extern "C" fn() -> Ret, }
        fnptr_impls_safety_abi! { unsafe extern "Rust" fn() -> Ret, }
        fnptr_impls_safety_abi! { unsafe extern "C" fn() -> Ret, }
    };
}

fnptr_impls_args! { }
fnptr_impls_args! { A }
fnptr_impls_args! { A, B }
fnptr_impls_args! { A, B, C }
fnptr_impls_args! { A, B, C, D }
fnptr_impls_args! { A, B, C, D, E }
fnptr_impls_args! { A, B, C, D, E, F }
fnptr_impls_args! { A, B, C, D, E, F, G }
fnptr_impls_args! { A, B, C, D, E, F, G, H }
fnptr_impls_args! { A, B, C, D, E, F, G, H, I }
fnptr_impls_args! { A, B, C, D, E, F, G, H, I, J }
fnptr_impls_args! { A, B, C, D, E, F, G, H, I, J, K }
fnptr_impls_args! { A, B, C, D, E, F, G, H, I, J, K, L }

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Ord for *const T {
    #[inline]
    fn cmp(&self, other: &*const T) -> Ordering { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> PartialOrd for *const T {
    #[inline]
    fn partial_cmp(&self, other: &*const T) -> Option<Ordering> { loop { } }

    #[inline]
    fn lt(&self, other: &*const T) -> bool { loop { } }

    #[inline]
    fn le(&self, other: &*const T) -> bool { loop { } }

    #[inline]
    fn gt(&self, other: &*const T) -> bool { loop { } }

    #[inline]
    fn ge(&self, other: &*const T) -> bool { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Ord for *mut T {
    #[inline]
    fn cmp(&self, other: &*mut T) -> Ordering { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> PartialOrd for *mut T {
    #[inline]
    fn partial_cmp(&self, other: &*mut T) -> Option<Ordering> { loop { } }

    #[inline]
    fn lt(&self, other: &*mut T) -> bool { loop { } }

    #[inline]
    fn le(&self, other: &*mut T) -> bool { loop { } }

    #[inline]
    fn gt(&self, other: &*mut T) -> bool { loop { } }

    #[inline]
    fn ge(&self, other: &*mut T) -> bool { loop { } }
}
