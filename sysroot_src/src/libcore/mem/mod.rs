//! Basic functions for dealing with memory.

#![stable(feature = "rust1", since = "1.0.0")]

use crate::clone;
use crate::cmp;
use crate::fmt;
use crate::hash;
use crate::intrinsics;
use crate::marker::{Copy, PhantomData, Sized};

mod manually_drop;
#[stable(feature = "manually_drop", since = "1.20.0")]
pub use manually_drop::ManuallyDrop;

mod maybe_uninit;
#[stable(feature = "maybe_uninit", since = "1.36.0")]
pub use maybe_uninit::MaybeUninit;

#[stable(feature = "rust1", since = "1.0.0")]
#[doc(inline)]
pub use crate::intrinsics::transmute;

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
pub fn forget<T>(t: T) { loop { } }

#[inline]
#[unstable(feature = "forget_unsized", issue = "0")]
pub fn forget_unsized<T: ?Sized>(t: T) { loop { } }

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_promotable]
pub const fn size_of<T>() -> usize {
    intrinsics::size_of::<T>()
}

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
pub fn size_of_val<T: ?Sized>(val: &T) -> usize { loop { } }

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_deprecated(reason = "use `align_of` instead", since = "1.2.0")]
pub fn min_align_of<T>() -> usize { loop { } }

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_deprecated(reason = "use `align_of_val` instead", since = "1.2.0")]
pub fn min_align_of_val<T: ?Sized>(val: &T) -> usize { loop { } }

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_promotable]
pub const fn align_of<T>() -> usize {
    intrinsics::min_align_of::<T>()
}

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
pub fn align_of_val<T: ?Sized>(val: &T) -> usize { loop { } }

#[inline]
#[stable(feature = "needs_drop", since = "1.21.0")]
pub const fn needs_drop<T>() -> bool {
    intrinsics::needs_drop::<T>()
}

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
#[allow(deprecated_in_future)]
#[allow(deprecated)]
pub unsafe fn zeroed<T>() -> T { loop { } }

#[inline]
#[rustc_deprecated(since = "1.39.0", reason = "use `mem::MaybeUninit` instead")]
#[stable(feature = "rust1", since = "1.0.0")]
#[allow(deprecated_in_future)]
#[allow(deprecated)]
pub unsafe fn uninitialized<T>() -> T { loop { } }

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
pub fn swap<T>(x: &mut T, y: &mut T) { loop { } }

#[inline]
#[stable(feature = "mem_take", since = "1.40.0")]
pub fn take<T: Default>(dest: &mut T) -> T { loop { } }

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
pub fn replace<T>(dest: &mut T, mut src: T) -> T { loop { } }

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
pub fn drop<T>(_x: T) { loop { } }

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
pub unsafe fn transmute_copy<T, U>(src: &T) -> U { loop { } }

/*(
// #[stable(feature = "discriminant_value", since = "1.21.0")]
pub struct Discriminant<T>(u64, PhantomData<fn() -> T>);


#[stable(feature = "discriminant_value", since = "1.21.0")]
impl<T> Copy for Discriminant<T> {}

#[stable(feature = "discriminant_value", since = "1.21.0")]
impl<T> clone::Clone for Discriminant<T> {
    fn clone(&self) -> Self { loop { } }
}

#[stable(feature = "discriminant_value", since = "1.21.0")]
impl<T> cmp::PartialEq for Discriminant<T> {
    fn eq(&self, rhs: &Self) -> bool { loop { } }
}

#[stable(feature = "discriminant_value", since = "1.21.0")]
impl<T> cmp::Eq for Discriminant<T> {}

#[stable(feature = "discriminant_value", since = "1.21.0")]
impl<T> hash::Hash for Discriminant<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) { loop { } }
}

#[stable(feature = "discriminant_value", since = "1.21.0")]
impl<T> fmt::Debug for Discriminant<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "discriminant_value", since = "1.21.0")]
pub fn discriminant<T>(v: &T) -> Discriminant<T> { loop { } }
*/
