use crate::convert::From;
use crate::ops::{CoerceUnsized, DispatchFromDyn};
use crate::fmt;
use crate::marker::{PhantomData, Unsize};
use crate::mem;
use crate::ptr::NonNull;

#[unstable(feature = "ptr_internals", issue = "0",
           reason = "use `NonNull` instead and consider `PhantomData<T>` \
                     (if you also use `#[may_dangle]`), `Send`, and/or `Sync`")]
#[doc(hidden)]
#[repr(transparent)]
#[rustc_layout_scalar_valid_range_start(1)]
pub struct Unique<T: ?Sized> {
    pointer: *const T,
    _marker: PhantomData<T>,
}

#[unstable(feature = "ptr_internals", issue = "0")]
unsafe impl<T: Send + ?Sized> Send for Unique<T> { }

#[unstable(feature = "ptr_internals", issue = "0")]
unsafe impl<T: Sync + ?Sized> Sync for Unique<T> { }

#[unstable(feature = "ptr_internals", issue = "0")]
impl<T: Sized> Unique<T> {
    #[inline]
    pub const fn empty() -> Self {
        unsafe {
            Unique::new_unchecked(mem::align_of::<T>() as *mut T)
        }
    }
}

#[unstable(feature = "ptr_internals", issue = "0")]
impl<T: ?Sized> Unique<T> {
    #[inline]
    pub const unsafe fn new_unchecked(ptr: *mut T) -> Self {
        Unique { pointer: ptr as _, _marker: PhantomData }
    }

    #[inline]
    pub fn new(ptr: *mut T) -> Option<Self> { loop { } }

    #[inline]
    pub const fn as_ptr(self) -> *mut T {
        self.pointer as *mut T
    }

    #[inline]
    pub unsafe fn as_ref(&self) -> &T { loop { } }

    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T { loop { } }

    #[inline]
    pub const fn cast<U>(self) -> Unique<U> {
        unsafe {
            Unique::new_unchecked(self.as_ptr() as *mut U)
        }
    }
}

#[unstable(feature = "ptr_internals", issue = "0")]
impl<T: ?Sized> Clone for Unique<T> {
    #[inline]
    fn clone(&self) -> Self { loop { } }
}

// #[unstable(feature = "ptr_internals", issue = "0")]
// impl<T: ?Sized> Copy for Unique<T> { }

#[unstable(feature = "ptr_internals", issue = "0")]
impl<T: ?Sized, U: ?Sized> CoerceUnsized<Unique<U>> for Unique<T> where T: Unsize<U> { }

#[unstable(feature = "ptr_internals", issue = "0")]
impl<T: ?Sized, U: ?Sized> DispatchFromDyn<Unique<U>> for Unique<T> where T: Unsize<U> { }

#[unstable(feature = "ptr_internals", issue = "0")]
impl<T: ?Sized> fmt::Debug for Unique<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[unstable(feature = "ptr_internals", issue = "0")]
impl<T: ?Sized> fmt::Pointer for Unique<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[unstable(feature = "ptr_internals", issue = "0")]
impl<T: ?Sized> From<&mut T> for Unique<T> {
    #[inline]
    fn from(reference: &mut T) -> Self { loop { } }
}

#[unstable(feature = "ptr_internals", issue = "0")]
impl<T: ?Sized> From<&T> for Unique<T> {
    #[inline]
    fn from(reference: &T) -> Self { loop { } }
}

#[unstable(feature = "ptr_internals", issue = "0")]
impl<T: ?Sized> From<NonNull<T>> for Unique<T> {
    #[inline]
    fn from(p: NonNull<T>) -> Self { loop { } }
}
