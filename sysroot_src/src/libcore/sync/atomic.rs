//! Atomic types

#![stable(feature = "rust1", since = "1.0.0")]
#![cfg_attr(not(target_has_atomic_load_store = "8"), allow(dead_code))]
#![cfg_attr(not(target_has_atomic_load_store = "8"), allow(unused_imports))]

use crate::cell::UnsafeCell;
use crate::fmt;

#[inline]
#[stable(feature = "spin_loop_hint", since = "1.24.0")]
pub fn spin_loop_hint() { loop { } }

#[cfg(any(bootstrap, target_has_atomic_load_store = "8"))]
#[stable(feature = "rust1", since = "1.0.0")]
#[repr(C, align(1))]
pub struct AtomicBool {
    v: UnsafeCell<u8>,
}

#[cfg(any(bootstrap, target_has_atomic_load_store = "8"))]
#[stable(feature = "rust1", since = "1.0.0")]
impl Default for AtomicBool {
    fn default() -> Self { loop { } }
}

#[cfg(any(bootstrap, target_has_atomic_load_store = "8"))]
#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl Sync for AtomicBool {}

#[cfg(any(bootstrap, target_has_atomic_load_store = "ptr"))]
#[stable(feature = "rust1", since = "1.0.0")]
#[cfg_attr(target_pointer_width = "16", repr(C, align(2)))]
#[cfg_attr(target_pointer_width = "32", repr(C, align(4)))]
#[cfg_attr(target_pointer_width = "64", repr(C, align(8)))]
pub struct AtomicPtr<T> {
    p: UnsafeCell<*mut T>,
}

#[cfg(any(bootstrap, target_has_atomic_load_store = "ptr"))]
#[stable(feature = "rust1", since = "1.0.0")]
impl<T> Default for AtomicPtr<T> {
    fn default() -> AtomicPtr<T> { loop { } }
}

#[cfg(any(bootstrap, target_has_atomic_load_store = "ptr"))]
#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<T> Send for AtomicPtr<T> {}
#[cfg(any(bootstrap, target_has_atomic_load_store = "ptr"))]
#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<T> Sync for AtomicPtr<T> {}

#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Ordering {
    #[stable(feature = "rust1", since = "1.0.0")]
    Relaxed,
    #[stable(feature = "rust1", since = "1.0.0")]
    Release,
    #[stable(feature = "rust1", since = "1.0.0")]
    Acquire,
    #[stable(feature = "rust1", since = "1.0.0")]
    AcqRel,
    #[stable(feature = "rust1", since = "1.0.0")]
    SeqCst,
}

#[cfg(any(bootstrap, target_has_atomic_load_store = "8"))]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_deprecated(
    since = "1.34.0",
    reason = "the `new` function is now preferred",
    suggestion = "AtomicBool::new(false)",
)]
pub const ATOMIC_BOOL_INIT: AtomicBool = AtomicBool::new(false);

#[cfg(any(bootstrap, target_has_atomic_load_store = "8"))]
impl AtomicBool {
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub const fn new(v: bool) -> AtomicBool {
        AtomicBool { v: UnsafeCell::new(v as u8) }
    }

    #[inline]
    #[stable(feature = "atomic_access", since = "1.15.0")]
    pub fn get_mut(&mut self) -> &mut bool { loop { } }

    #[inline]
    #[stable(feature = "atomic_access", since = "1.15.0")]
    pub fn into_inner(self) -> bool { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn load(&self, order: Ordering) -> bool { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn store(&self, val: bool, order: Ordering) { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[cfg(target_has_atomic = "8")]
    pub fn swap(&self, val: bool, order: Ordering) -> bool { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[cfg(target_has_atomic = "8")]
    pub fn compare_and_swap(&self, current: bool, new: bool, order: Ordering) -> bool { loop { } }

    #[inline]
    #[stable(feature = "extended_compare_and_swap", since = "1.10.0")]
    #[cfg(target_has_atomic = "8")]
    pub fn compare_exchange(&self,
                            current: bool,
                            new: bool,
                            success: Ordering,
                            failure: Ordering)
                            -> Result<bool, bool> { loop { } }

    #[inline]
    #[stable(feature = "extended_compare_and_swap", since = "1.10.0")]
    #[cfg(target_has_atomic = "8")]
    pub fn compare_exchange_weak(&self,
                                 current: bool,
                                 new: bool,
                                 success: Ordering,
                                 failure: Ordering)
                                 -> Result<bool, bool> { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[cfg(target_has_atomic = "8")]
    pub fn fetch_and(&self, val: bool, order: Ordering) -> bool { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[cfg(target_has_atomic = "8")]
    pub fn fetch_nand(&self, val: bool, order: Ordering) -> bool { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[cfg(target_has_atomic = "8")]
    pub fn fetch_or(&self, val: bool, order: Ordering) -> bool { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[cfg(target_has_atomic = "8")]
    pub fn fetch_xor(&self, val: bool, order: Ordering) -> bool { loop { } }
}

#[cfg(any(bootstrap, target_has_atomic_load_store = "ptr"))]
impl<T> AtomicPtr<T> {
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub const fn new(p: *mut T) -> AtomicPtr<T> {
        AtomicPtr { p: UnsafeCell::new(p) }
    }

    #[inline]
    #[stable(feature = "atomic_access", since = "1.15.0")]
    pub fn get_mut(&mut self) -> &mut *mut T { loop { } }

    #[inline]
    #[stable(feature = "atomic_access", since = "1.15.0")]
    pub fn into_inner(self) -> *mut T { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn load(&self, order: Ordering) -> *mut T { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn store(&self, ptr: *mut T, order: Ordering) { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[cfg(target_has_atomic = "ptr")]
    pub fn swap(&self, ptr: *mut T, order: Ordering) -> *mut T { loop { } }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[cfg(target_has_atomic = "ptr")]
    pub fn compare_and_swap(&self, current: *mut T, new: *mut T, order: Ordering) -> *mut T { loop { } }

    #[inline]
    #[stable(feature = "extended_compare_and_swap", since = "1.10.0")]
    #[cfg(target_has_atomic = "ptr")]
    pub fn compare_exchange(&self,
                            current: *mut T,
                            new: *mut T,
                            success: Ordering,
                            failure: Ordering)
                            -> Result<*mut T, *mut T> { loop { } }

    #[inline]
    #[stable(feature = "extended_compare_and_swap", since = "1.10.0")]
    #[cfg(target_has_atomic = "ptr")]
    pub fn compare_exchange_weak(&self,
                                 current: *mut T,
                                 new: *mut T,
                                 success: Ordering,
                                 failure: Ordering)
                                 -> Result<*mut T, *mut T> { loop { } }
}

#[cfg(any(bootstrap, target_has_atomic_load_store = "8"))]
#[stable(feature = "atomic_bool_from", since = "1.24.0")]
impl From<bool> for AtomicBool {
    #[inline]
    fn from(b: bool) -> Self { loop { } }
}

#[cfg(any(bootstrap, target_has_atomic_load_store = "ptr"))]
#[stable(feature = "atomic_from", since = "1.23.0")]
impl<T> From<*mut T> for AtomicPtr<T> {
    #[inline]
    fn from(p: *mut T) -> Self { loop { } }
}

#[cfg(any(bootstrap, target_has_atomic_load_store = "8"))]
macro_rules! atomic_int {
    ($cfg_cas:meta,
     $stable:meta,
     $stable_cxchg:meta,
     $stable_debug:meta,
     $stable_access:meta,
     $stable_from:meta,
     $stable_nand:meta,
     $stable_init_const:meta,
     $s_int_type:expr, $int_ref:expr,
     $extra_feature:expr,
     $min_fn:ident, $max_fn:ident,
     $align:expr,
     $atomic_new:expr,
     $int_type:ident $atomic_type:ident $atomic_init:ident) => {
        #[doc = $s_int_type]
        #[doc = $int_ref]
        #[$stable]
        #[repr(C, align($align))]
        pub struct $atomic_type {
            v: UnsafeCell<$int_type>,
        }

        #[$stable_init_const]
        #[rustc_deprecated(
            since = "1.34.0",
            reason = "the `new` function is now preferred",
            suggestion = $atomic_new,
        )]
        pub const $atomic_init: $atomic_type = $atomic_type::new(0);

        #[$stable]
        impl Default for $atomic_type {
            fn default() -> Self { loop { } }
        }

        #[$stable_from]
        impl From<$int_type> for $atomic_type {
            doc_comment! {
                concat!(
"Converts an `", stringify!($int_type), "` into an `", stringify!($atomic_type), "`."),
                #[inline]
                fn from(v: $int_type) -> Self { loop { } }
            }
        }

        #[$stable_debug]
        impl fmt::Debug for $atomic_type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
        }

        #[$stable]
        unsafe impl Sync for $atomic_type {}

        impl $atomic_type {
            doc_comment! {
                concat!("Creates a new atomic integer.

# Examples

```
", $extra_feature, "use std::sync::atomic::", stringify!($atomic_type), ";

let atomic_forty_two = ", stringify!($atomic_type), "::new(42);
```"),
                #[inline]
                #[$stable]
                pub const fn new(v: $int_type) -> Self {
                    $atomic_type {v: UnsafeCell::new(v)}
                }
            }

            doc_comment! {
                concat!("Returns a mutable reference to the underlying integer.

This is safe because the mutable reference guarantees that no other threads are
concurrently accessing the atomic data.

# Examples

```
", $extra_feature, "use std::sync::atomic::{", stringify!($atomic_type), ", Ordering};

let mut some_var = ", stringify!($atomic_type), "::new(10);
assert_eq!(*some_var.get_mut(), 10);
*some_var.get_mut() = 5;
assert_eq!(some_var.load(Ordering::SeqCst), 5);
```"),
                #[inline]
                #[$stable_access]
                pub fn get_mut(&mut self) -> &mut $int_type { loop { } }
            }

            doc_comment! {
                concat!("Consumes the atomic and returns the contained value.

This is safe because passing `self` by value guarantees that no other threads are
concurrently accessing the atomic data.

# Examples

```
", $extra_feature, "use std::sync::atomic::", stringify!($atomic_type), ";

let some_var = ", stringify!($atomic_type), "::new(5);
assert_eq!(some_var.into_inner(), 5);
```"),
                #[inline]
                #[$stable_access]
                pub fn into_inner(self) -> $int_type { loop { } }
            }

            doc_comment! {
                concat!("Loads a value from the atomic integer.

`load` takes an [`Ordering`] argument which describes the memory ordering of this operation.
Possible values are [`SeqCst`], [`Acquire`] and [`Relaxed`].

# Panics

Panics if `order` is [`Release`] or [`AcqRel`].

[`Ordering`]: enum.Ordering.html
[`Relaxed`]: enum.Ordering.html#variant.Relaxed
[`Release`]: enum.Ordering.html#variant.Release
[`Acquire`]: enum.Ordering.html#variant.Acquire
[`AcqRel`]: enum.Ordering.html#variant.AcqRel
[`SeqCst`]: enum.Ordering.html#variant.SeqCst

# Examples

```
", $extra_feature, "use std::sync::atomic::{", stringify!($atomic_type), ", Ordering};

let some_var = ", stringify!($atomic_type), "::new(5);

assert_eq!(some_var.load(Ordering::Relaxed), 5);
```"),
                #[inline]
                #[$stable]
                pub fn load(&self, order: Ordering) -> $int_type { loop { } }
            }

            doc_comment! {
                concat!("Stores a value into the atomic integer.

`store` takes an [`Ordering`] argument which describes the memory ordering of this operation.
 Possible values are [`SeqCst`], [`Release`] and [`Relaxed`].

# Panics

Panics if `order` is [`Acquire`] or [`AcqRel`].

[`Ordering`]: enum.Ordering.html
[`Relaxed`]: enum.Ordering.html#variant.Relaxed
[`Release`]: enum.Ordering.html#variant.Release
[`Acquire`]: enum.Ordering.html#variant.Acquire
[`AcqRel`]: enum.Ordering.html#variant.AcqRel
[`SeqCst`]: enum.Ordering.html#variant.SeqCst

# Examples

```
", $extra_feature, "use std::sync::atomic::{", stringify!($atomic_type), ", Ordering};

let some_var = ", stringify!($atomic_type), "::new(5);

some_var.store(10, Ordering::Relaxed);
assert_eq!(some_var.load(Ordering::Relaxed), 10);
```"),
                #[inline]
                #[$stable]
                pub fn store(&self, val: $int_type, order: Ordering) { loop { } }
            }

            doc_comment! {
                concat!("Stores a value into the atomic integer, returning the previous value.

`swap` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

[`Ordering`]: enum.Ordering.html
[`Relaxed`]: enum.Ordering.html#variant.Relaxed
[`Release`]: enum.Ordering.html#variant.Release
[`Acquire`]: enum.Ordering.html#variant.Acquire

# Examples

```
", $extra_feature, "use std::sync::atomic::{", stringify!($atomic_type), ", Ordering};

let some_var = ", stringify!($atomic_type), "::new(5);

assert_eq!(some_var.swap(10, Ordering::Relaxed), 5);
```"),
                #[inline]
                #[$stable]
                #[$cfg_cas]
                pub fn swap(&self, val: $int_type, order: Ordering) -> $int_type { loop { } }
            }

            doc_comment! {
                concat!("Stores a value into the atomic integer if the current value is the same as
the `current` value.

The return value is always the previous value. If it is equal to `current`, then the
value was updated.

`compare_and_swap` also takes an [`Ordering`] argument which describes the memory
ordering of this operation. Notice that even when using [`AcqRel`], the operation
might fail and hence just perform an `Acquire` load, but not have `Release` semantics.
Using [`Acquire`] makes the store part of this operation [`Relaxed`] if it
happens, and using [`Release`] makes the load part [`Relaxed`].

[`Ordering`]: enum.Ordering.html
[`Relaxed`]: enum.Ordering.html#variant.Relaxed
[`Release`]: enum.Ordering.html#variant.Release
[`Acquire`]: enum.Ordering.html#variant.Acquire
[`AcqRel`]: enum.Ordering.html#variant.AcqRel

# Examples

```
", $extra_feature, "use std::sync::atomic::{", stringify!($atomic_type), ", Ordering};

let some_var = ", stringify!($atomic_type), "::new(5);

assert_eq!(some_var.compare_and_swap(5, 10, Ordering::Relaxed), 5);
assert_eq!(some_var.load(Ordering::Relaxed), 10);

assert_eq!(some_var.compare_and_swap(6, 12, Ordering::Relaxed), 10);
assert_eq!(some_var.load(Ordering::Relaxed), 10);
```"),
                #[inline]
                #[$stable]
                #[$cfg_cas]
                pub fn compare_and_swap(&self,
                                        current: $int_type,
                                        new: $int_type,
                                        order: Ordering) -> $int_type { loop { } }
            }

            doc_comment! {
                concat!("Stores a value into the atomic integer if the current value is the same as
the `current` value.

The return value is a result indicating whether the new value was written and
containing the previous value. On success this value is guaranteed to be equal to
`current`.

`compare_exchange` takes two [`Ordering`] arguments to describe the memory
ordering of this operation. The first describes the required ordering if the
operation succeeds while the second describes the required ordering when the
operation fails. Using [`Acquire`] as success ordering makes the store part
of this operation [`Relaxed`], and using [`Release`] makes the successful load
[`Relaxed`]. The failure ordering can only be [`SeqCst`], [`Acquire`] or [`Relaxed`]
and must be equivalent to or weaker than the success ordering.

[`Ordering`]: enum.Ordering.html
[`Relaxed`]: enum.Ordering.html#variant.Relaxed
[`Release`]: enum.Ordering.html#variant.Release
[`Acquire`]: enum.Ordering.html#variant.Acquire
[`SeqCst`]: enum.Ordering.html#variant.SeqCst

# Examples

```
", $extra_feature, "use std::sync::atomic::{", stringify!($atomic_type), ", Ordering};

let some_var = ", stringify!($atomic_type), "::new(5);

assert_eq!(some_var.compare_exchange(5, 10,
                                     Ordering::Acquire,
                                     Ordering::Relaxed),
           Ok(5));
assert_eq!(some_var.load(Ordering::Relaxed), 10);

assert_eq!(some_var.compare_exchange(6, 12,
                                     Ordering::SeqCst,
                                     Ordering::Acquire),
           Err(10));
assert_eq!(some_var.load(Ordering::Relaxed), 10);
```"),
                #[inline]
                #[$stable_cxchg]
                #[$cfg_cas]
                pub fn compare_exchange(&self,
                                        current: $int_type,
                                        new: $int_type,
                                        success: Ordering,
                                        failure: Ordering) -> Result<$int_type, $int_type> { loop { } }
            }

            doc_comment! {
                concat!("Stores a value into the atomic integer if the current value is the same as
the `current` value.

Unlike [`compare_exchange`], this function is allowed to spuriously fail even
when the comparison succeeds, which can result in more efficient code on some
platforms. The return value is a result indicating whether the new value was
written and containing the previous value.

`compare_exchange_weak` takes two [`Ordering`] arguments to describe the memory
ordering of this operation. The first describes the required ordering if the
operation succeeds while the second describes the required ordering when the
operation fails. Using [`Acquire`] as success ordering makes the store part
of this operation [`Relaxed`], and using [`Release`] makes the successful load
[`Relaxed`]. The failure ordering can only be [`SeqCst`], [`Acquire`] or [`Relaxed`]
and must be equivalent to or weaker than the success ordering.

[`compare_exchange`]: #method.compare_exchange
[`Ordering`]: enum.Ordering.html
[`Relaxed`]: enum.Ordering.html#variant.Relaxed
[`Release`]: enum.Ordering.html#variant.Release
[`Acquire`]: enum.Ordering.html#variant.Acquire
[`SeqCst`]: enum.Ordering.html#variant.SeqCst

# Examples

```
", $extra_feature, "use std::sync::atomic::{", stringify!($atomic_type), ", Ordering};

let val = ", stringify!($atomic_type), "::new(4);

let mut old = val.load(Ordering::Relaxed);
loop {
    let new = old * 2;
    match val.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {
        Ok(_) => break,
        Err(x) => old = x,
    }
}
```"),
                #[inline]
                #[$stable_cxchg]
                #[$cfg_cas]
                pub fn compare_exchange_weak(&self,
                                             current: $int_type,
                                             new: $int_type,
                                             success: Ordering,
                                             failure: Ordering) -> Result<$int_type, $int_type> { loop { } }
            }

            doc_comment! {
                concat!("Adds to the current value, returning the previous value.

This operation wraps around on overflow.

`fetch_add` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

[`Ordering`]: enum.Ordering.html
[`Relaxed`]: enum.Ordering.html#variant.Relaxed
[`Release`]: enum.Ordering.html#variant.Release
[`Acquire`]: enum.Ordering.html#variant.Acquire

# Examples

```
", $extra_feature, "use std::sync::atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(0);
assert_eq!(foo.fetch_add(10, Ordering::SeqCst), 0);
assert_eq!(foo.load(Ordering::SeqCst), 10);
```"),
                #[inline]
                #[$stable]
                #[$cfg_cas]
                pub fn fetch_add(&self, val: $int_type, order: Ordering) -> $int_type { loop { } }
            }

            doc_comment! {
                concat!("Subtracts from the current value, returning the previous value.

This operation wraps around on overflow.

`fetch_sub` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

[`Ordering`]: enum.Ordering.html
[`Relaxed`]: enum.Ordering.html#variant.Relaxed
[`Release`]: enum.Ordering.html#variant.Release
[`Acquire`]: enum.Ordering.html#variant.Acquire

# Examples

```
", $extra_feature, "use std::sync::atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(20);
assert_eq!(foo.fetch_sub(10, Ordering::SeqCst), 20);
assert_eq!(foo.load(Ordering::SeqCst), 10);
```"),
                #[inline]
                #[$stable]
                #[$cfg_cas]
                pub fn fetch_sub(&self, val: $int_type, order: Ordering) -> $int_type { loop { } }
            }

            doc_comment! {
                concat!("Bitwise \"and\" with the current value.

Performs a bitwise \"and\" operation on the current value and the argument `val`, and
sets the new value to the result.

Returns the previous value.

`fetch_and` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

[`Ordering`]: enum.Ordering.html
[`Relaxed`]: enum.Ordering.html#variant.Relaxed
[`Release`]: enum.Ordering.html#variant.Release
[`Acquire`]: enum.Ordering.html#variant.Acquire

# Examples

```
", $extra_feature, "use std::sync::atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(0b101101);
assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);
assert_eq!(foo.load(Ordering::SeqCst), 0b100001);
```"),
                #[inline]
                #[$stable]
                #[$cfg_cas]
                pub fn fetch_and(&self, val: $int_type, order: Ordering) -> $int_type { loop { } }
            }

            doc_comment! {
                concat!("Bitwise \"nand\" with the current value.

Performs a bitwise \"nand\" operation on the current value and the argument `val`, and
sets the new value to the result.

Returns the previous value.

`fetch_nand` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

[`Ordering`]: enum.Ordering.html
[`Relaxed`]: enum.Ordering.html#variant.Relaxed
[`Release`]: enum.Ordering.html#variant.Release
[`Acquire`]: enum.Ordering.html#variant.Acquire

# Examples

```
", $extra_feature, "
use std::sync::atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(0x13);
assert_eq!(foo.fetch_nand(0x31, Ordering::SeqCst), 0x13);
assert_eq!(foo.load(Ordering::SeqCst), !(0x13 & 0x31));
```"),
                #[inline]
                #[$stable_nand]
                #[$cfg_cas]
                pub fn fetch_nand(&self, val: $int_type, order: Ordering) -> $int_type { loop { } }
            }

            doc_comment! {
                concat!("Bitwise \"or\" with the current value.

Performs a bitwise \"or\" operation on the current value and the argument `val`, and
sets the new value to the result.

Returns the previous value.

`fetch_or` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

[`Ordering`]: enum.Ordering.html
[`Relaxed`]: enum.Ordering.html#variant.Relaxed
[`Release`]: enum.Ordering.html#variant.Release
[`Acquire`]: enum.Ordering.html#variant.Acquire

# Examples

```
", $extra_feature, "use std::sync::atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(0b101101);
assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);
assert_eq!(foo.load(Ordering::SeqCst), 0b111111);
```"),
                #[inline]
                #[$stable]
                #[$cfg_cas]
                pub fn fetch_or(&self, val: $int_type, order: Ordering) -> $int_type { loop { } }
            }

            doc_comment! {
                concat!("Bitwise \"xor\" with the current value.

Performs a bitwise \"xor\" operation on the current value and the argument `val`, and
sets the new value to the result.

Returns the previous value.

`fetch_xor` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

[`Ordering`]: enum.Ordering.html
[`Relaxed`]: enum.Ordering.html#variant.Relaxed
[`Release`]: enum.Ordering.html#variant.Release
[`Acquire`]: enum.Ordering.html#variant.Acquire

# Examples

```
", $extra_feature, "use std::sync::atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(0b101101);
assert_eq!(foo.fetch_xor(0b110011, Ordering::SeqCst), 0b101101);
assert_eq!(foo.load(Ordering::SeqCst), 0b011110);
```"),
                #[inline]
                #[$stable]
                #[$cfg_cas]
                pub fn fetch_xor(&self, val: $int_type, order: Ordering) -> $int_type { loop { } }
            }

            doc_comment! {
                concat!("Fetches the value, and applies a function to it that returns an optional
new value. Returns a `Result` of `Ok(previous_value)` if the function returned `Some(_)`, else
`Err(previous_value)`.

Note: This may call the function multiple times if the value has been changed from other threads in
the meantime, as long as the function returns `Some(_)`, but the function will have been applied
but once to the stored value.

`fetch_update` takes two [`Ordering`] arguments to describe the memory
ordering of this operation. The first describes the required ordering for loads
and failed updates while the second describes the required ordering when the
operation finally succeeds. Beware that this is different from the two
modes in [`compare_exchange`]!

Using [`Acquire`] as success ordering makes the store part
of this operation [`Relaxed`], and using [`Release`] makes the final successful load
[`Relaxed`]. The (failed) load ordering can only be [`SeqCst`], [`Acquire`] or [`Relaxed`]
and must be equivalent to or weaker than the success ordering.

[`bool`]: ../../../std/primitive.bool.html
[`compare_exchange`]: #method.compare_exchange
[`Ordering`]: enum.Ordering.html
[`Relaxed`]: enum.Ordering.html#variant.Relaxed
[`Release`]: enum.Ordering.html#variant.Release
[`Acquire`]: enum.Ordering.html#variant.Acquire
[`SeqCst`]: enum.Ordering.html#variant.SeqCst

# Examples

```rust
#![feature(no_more_cas)]
", $extra_feature, "use std::sync::atomic::{", stringify!($atomic_type), ", Ordering};

let x = ", stringify!($atomic_type), "::new(7);
assert_eq!(x.fetch_update(|_| None, Ordering::SeqCst, Ordering::SeqCst), Err(7));
assert_eq!(x.fetch_update(|x| Some(x + 1), Ordering::SeqCst, Ordering::SeqCst), Ok(7));
assert_eq!(x.fetch_update(|x| Some(x + 1), Ordering::SeqCst, Ordering::SeqCst), Ok(8));
assert_eq!(x.load(Ordering::SeqCst), 9);
```"),
                #[inline]
                #[unstable(feature = "no_more_cas",
                       reason = "no more CAS loops in user code",
                       issue = "48655")]
                #[$cfg_cas]
                pub fn fetch_update<F>(&self,
                                       mut f: F,
                                       fetch_order: Ordering,
                                       set_order: Ordering) -> Result<$int_type, $int_type>
                where F: FnMut($int_type) -> Option<$int_type> { loop { } }
            }

            doc_comment! {
                concat!("Maximum with the current value.

Finds the maximum of the current value and the argument `val`, and
sets the new value to the result.

Returns the previous value.

`fetch_max` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

[`Ordering`]: enum.Ordering.html
[`Relaxed`]: enum.Ordering.html#variant.Relaxed
[`Release`]: enum.Ordering.html#variant.Release
[`Acquire`]: enum.Ordering.html#variant.Acquire

# Examples

```
#![feature(atomic_min_max)]
", $extra_feature, "use std::sync::atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(23);
assert_eq!(foo.fetch_max(42, Ordering::SeqCst), 23);
assert_eq!(foo.load(Ordering::SeqCst), 42);
```

If you want to obtain the maximum value in one step, you can use the following:

```
#![feature(atomic_min_max)]
", $extra_feature, "use std::sync::atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(23);
let bar = 42;
let max_foo = foo.fetch_max(bar, Ordering::SeqCst).max(bar);
assert!(max_foo == 42);
```"),
                #[inline]
                #[unstable(feature = "atomic_min_max",
                       reason = "easier and faster min/max than writing manual CAS loop",
                       issue = "48655")]
                #[$cfg_cas]
                pub fn fetch_max(&self, val: $int_type, order: Ordering) -> $int_type { loop { } }
            }

            doc_comment! {
                concat!("Minimum with the current value.

Finds the minimum of the current value and the argument `val`, and
sets the new value to the result.

Returns the previous value.

`fetch_min` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

[`Ordering`]: enum.Ordering.html
[`Relaxed`]: enum.Ordering.html#variant.Relaxed
[`Release`]: enum.Ordering.html#variant.Release
[`Acquire`]: enum.Ordering.html#variant.Acquire

# Examples

```
#![feature(atomic_min_max)]
", $extra_feature, "use std::sync::atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(23);
assert_eq!(foo.fetch_min(42, Ordering::Relaxed), 23);
assert_eq!(foo.load(Ordering::Relaxed), 23);
assert_eq!(foo.fetch_min(22, Ordering::Relaxed), 23);
assert_eq!(foo.load(Ordering::Relaxed), 22);
```

If you want to obtain the minimum value in one step, you can use the following:

```
#![feature(atomic_min_max)]
", $extra_feature, "use std::sync::atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(23);
let bar = 12;
let min_foo = foo.fetch_min(bar, Ordering::SeqCst).min(bar);
assert_eq!(min_foo, 12);
```"),
                #[inline]
                #[unstable(feature = "atomic_min_max",
                       reason = "easier and faster min/max than writing manual CAS loop",
                       issue = "48655")]
                #[$cfg_cas]
                pub fn fetch_min(&self, val: $int_type, order: Ordering) -> $int_type { loop { } }
            }

        }
    }
}

#[cfg(any(bootstrap, target_has_atomic_load_store = "8"))]
atomic_int! {
    cfg(target_has_atomic = "8"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    unstable(feature = "integer_atomics", issue = "32976"),
    "i8", "../../../std/primitive.i8.html",
    "",
    atomic_min, atomic_max,
    1,
    "AtomicI8::new(0)",
    i8 AtomicI8 ATOMIC_I8_INIT
}
#[cfg(any(bootstrap, target_has_atomic_load_store = "8"))]
atomic_int! {
    cfg(target_has_atomic = "8"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    unstable(feature = "integer_atomics", issue = "32976"),
    "u8", "../../../std/primitive.u8.html",
    "",
    atomic_umin, atomic_umax,
    1,
    "AtomicU8::new(0)",
    u8 AtomicU8 ATOMIC_U8_INIT
}
#[cfg(any(bootstrap, target_has_atomic_load_store = "16"))]
atomic_int! {
    cfg(target_has_atomic = "16"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    unstable(feature = "integer_atomics", issue = "32976"),
    "i16", "../../../std/primitive.i16.html",
    "",
    atomic_min, atomic_max,
    2,
    "AtomicI16::new(0)",
    i16 AtomicI16 ATOMIC_I16_INIT
}
#[cfg(any(bootstrap, target_has_atomic_load_store = "16"))]
atomic_int! {
    cfg(target_has_atomic = "16"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    unstable(feature = "integer_atomics", issue = "32976"),
    "u16", "../../../std/primitive.u16.html",
    "",
    atomic_umin, atomic_umax,
    2,
    "AtomicU16::new(0)",
    u16 AtomicU16 ATOMIC_U16_INIT
}
#[cfg(any(bootstrap, target_has_atomic_load_store = "32"))]
atomic_int! {
    cfg(target_has_atomic = "32"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    unstable(feature = "integer_atomics", issue = "32976"),
    "i32", "../../../std/primitive.i32.html",
    "",
    atomic_min, atomic_max,
    4,
    "AtomicI32::new(0)",
    i32 AtomicI32 ATOMIC_I32_INIT
}
#[cfg(any(bootstrap, target_has_atomic_load_store = "32"))]
atomic_int! {
    cfg(target_has_atomic = "32"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    unstable(feature = "integer_atomics", issue = "32976"),
    "u32", "../../../std/primitive.u32.html",
    "",
    atomic_umin, atomic_umax,
    4,
    "AtomicU32::new(0)",
    u32 AtomicU32 ATOMIC_U32_INIT
}
#[cfg(any(
    all(bootstrap, target_has_atomic = "64"),
    target_has_atomic_load_store = "64"
))]
atomic_int! {
    cfg(target_has_atomic = "64"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    unstable(feature = "integer_atomics", issue = "32976"),
    "i64", "../../../std/primitive.i64.html",
    "",
    atomic_min, atomic_max,
    8,
    "AtomicI64::new(0)",
    i64 AtomicI64 ATOMIC_I64_INIT
}
#[cfg(any(
    all(bootstrap, target_has_atomic = "64"),
    target_has_atomic_load_store = "64"
))]
atomic_int! {
    cfg(target_has_atomic = "64"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    stable(feature = "integer_atomics_stable", since = "1.34.0"),
    unstable(feature = "integer_atomics", issue = "32976"),
    "u64", "../../../std/primitive.u64.html",
    "",
    atomic_umin, atomic_umax,
    8,
    "AtomicU64::new(0)",
    u64 AtomicU64 ATOMIC_U64_INIT
}
#[cfg(target_has_atomic_load_store = "128")]
atomic_int! {
    cfg(target_has_atomic = "128"),
    unstable(feature = "integer_atomics", issue = "32976"),
    unstable(feature = "integer_atomics", issue = "32976"),
    unstable(feature = "integer_atomics", issue = "32976"),
    unstable(feature = "integer_atomics", issue = "32976"),
    unstable(feature = "integer_atomics", issue = "32976"),
    unstable(feature = "integer_atomics", issue = "32976"),
    unstable(feature = "integer_atomics", issue = "32976"),
    "i128", "../../../std/primitive.i128.html",
    "#![feature(integer_atomics)]\n\n",
    atomic_min, atomic_max,
    16,
    "AtomicI128::new(0)",
    i128 AtomicI128 ATOMIC_I128_INIT
}
#[cfg(target_has_atomic_load_store = "128")]
atomic_int! {
    cfg(target_has_atomic = "128"),
    unstable(feature = "integer_atomics", issue = "32976"),
    unstable(feature = "integer_atomics", issue = "32976"),
    unstable(feature = "integer_atomics", issue = "32976"),
    unstable(feature = "integer_atomics", issue = "32976"),
    unstable(feature = "integer_atomics", issue = "32976"),
    unstable(feature = "integer_atomics", issue = "32976"),
    unstable(feature = "integer_atomics", issue = "32976"),
    "u128", "../../../std/primitive.u128.html",
    "#![feature(integer_atomics)]\n\n",
    atomic_umin, atomic_umax,
    16,
    "AtomicU128::new(0)",
    u128 AtomicU128 ATOMIC_U128_INIT
}
#[cfg(any(bootstrap, target_has_atomic_load_store = "ptr"))]
#[cfg(target_pointer_width = "16")]
macro_rules! ptr_width {
    () => { 2 }
}
#[cfg(any(bootstrap, target_has_atomic_load_store = "ptr"))]
#[cfg(target_pointer_width = "32")]
macro_rules! ptr_width {
    () => { 4 }
}
#[cfg(any(bootstrap, target_has_atomic_load_store = "ptr"))]
#[cfg(target_pointer_width = "64")]
macro_rules! ptr_width {
    () => { 8 }
}
#[cfg(any(bootstrap, target_has_atomic_load_store = "ptr"))]
atomic_int!{
    cfg(target_has_atomic = "ptr"),
    stable(feature = "rust1", since = "1.0.0"),
    stable(feature = "extended_compare_and_swap", since = "1.10.0"),
    stable(feature = "atomic_debug", since = "1.3.0"),
    stable(feature = "atomic_access", since = "1.15.0"),
    stable(feature = "atomic_from", since = "1.23.0"),
    stable(feature = "atomic_nand", since = "1.27.0"),
    stable(feature = "rust1", since = "1.0.0"),
    "isize", "../../../std/primitive.isize.html",
    "",
    atomic_min, atomic_max,
    ptr_width!(),
    "AtomicIsize::new(0)",
    isize AtomicIsize ATOMIC_ISIZE_INIT
}
#[cfg(any(bootstrap, target_has_atomic_load_store = "ptr"))]
atomic_int!{
    cfg(target_has_atomic = "ptr"),
    stable(feature = "rust1", since = "1.0.0"),
    stable(feature = "extended_compare_and_swap", since = "1.10.0"),
    stable(feature = "atomic_debug", since = "1.3.0"),
    stable(feature = "atomic_access", since = "1.15.0"),
    stable(feature = "atomic_from", since = "1.23.0"),
    stable(feature = "atomic_nand", since = "1.27.0"),
    stable(feature = "rust1", since = "1.0.0"),
    "usize", "../../../std/primitive.usize.html",
    "",
    atomic_umin, atomic_umax,
    ptr_width!(),
    "AtomicUsize::new(0)",
    usize AtomicUsize ATOMIC_USIZE_INIT
}

#[inline]
#[cfg(target_has_atomic = "8")]
fn strongest_failure_ordering(order: Ordering) -> Ordering { loop { } }

#[inline]
unsafe fn atomic_store<T>(dst: *mut T, val: T, order: Ordering) { loop { } }

#[inline]
unsafe fn atomic_load<T>(dst: *const T, order: Ordering) -> T { loop { } }

#[inline]
#[cfg(target_has_atomic = "8")]
unsafe fn atomic_swap<T>(dst: *mut T, val: T, order: Ordering) -> T { loop { } }

#[inline]
#[cfg(target_has_atomic = "8")]
unsafe fn atomic_add<T>(dst: *mut T, val: T, order: Ordering) -> T { loop { } }

#[inline]
#[cfg(target_has_atomic = "8")]
unsafe fn atomic_sub<T>(dst: *mut T, val: T, order: Ordering) -> T { loop { } }

#[inline]
#[cfg(target_has_atomic = "8")]
unsafe fn atomic_compare_exchange<T>(dst: *mut T,
                                     old: T,
                                     new: T,
                                     success: Ordering,
                                     failure: Ordering)
                                     -> Result<T, T> { loop { } }

#[inline]
#[cfg(target_has_atomic = "8")]
unsafe fn atomic_compare_exchange_weak<T>(dst: *mut T,
                                          old: T,
                                          new: T,
                                          success: Ordering,
                                          failure: Ordering)
                                          -> Result<T, T> { loop { } }

#[inline]
#[cfg(target_has_atomic = "8")]
unsafe fn atomic_and<T>(dst: *mut T, val: T, order: Ordering) -> T { loop { } }

#[inline]
#[cfg(target_has_atomic = "8")]
unsafe fn atomic_nand<T>(dst: *mut T, val: T, order: Ordering) -> T { loop { } }

#[inline]
#[cfg(target_has_atomic = "8")]
unsafe fn atomic_or<T>(dst: *mut T, val: T, order: Ordering) -> T { loop { } }

#[inline]
#[cfg(target_has_atomic = "8")]
unsafe fn atomic_xor<T>(dst: *mut T, val: T, order: Ordering) -> T { loop { } }

#[inline]
#[cfg(target_has_atomic = "8")]
unsafe fn atomic_max<T>(dst: *mut T, val: T, order: Ordering) -> T { loop { } }

#[inline]
#[cfg(target_has_atomic = "8")]
unsafe fn atomic_min<T>(dst: *mut T, val: T, order: Ordering) -> T { loop { } }

#[inline]
#[cfg(target_has_atomic = "8")]
unsafe fn atomic_umax<T>(dst: *mut T, val: T, order: Ordering) -> T { loop { } }

#[inline]
#[cfg(target_has_atomic = "8")]
unsafe fn atomic_umin<T>(dst: *mut T, val: T, order: Ordering) -> T { loop { } }

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
#[cfg_attr(target_arch = "wasm32", allow(unused_variables))]
pub fn fence(order: Ordering) { loop { } }


#[inline]
#[stable(feature = "compiler_fences", since = "1.21.0")]
pub fn compiler_fence(order: Ordering) { loop { } }


#[cfg(any(bootstrap, target_has_atomic_load_store = "8"))]
#[stable(feature = "atomic_debug", since = "1.3.0")]
impl fmt::Debug for AtomicBool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[cfg(any(bootstrap, target_has_atomic_load_store = "ptr"))]
#[stable(feature = "atomic_debug", since = "1.3.0")]
impl<T> fmt::Debug for AtomicPtr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[cfg(any(bootstrap, target_has_atomic_load_store = "ptr"))]
#[stable(feature = "atomic_pointer", since = "1.24.0")]
impl<T> fmt::Pointer for AtomicPtr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}
