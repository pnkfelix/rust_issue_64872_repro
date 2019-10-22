use crate::convert::From;
use crate::ops::{CoerceUnsized, DispatchFromDyn};
use crate::fmt;
use crate::marker::Unsize;
use crate::mem;
use crate::ptr::Unique;
use crate::cmp::Ordering;

#[stable(feature = "nonnull", since = "1.25.0")]
#[repr(transparent)]
#[rustc_layout_scalar_valid_range_start(1)]
#[rustc_nonnull_optimization_guaranteed]
pub struct NonNull<T: ?Sized> {
    pointer: *const T,
}

#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> !Send for NonNull<T> { }

#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> !Sync for NonNull<T> { }

impl<T: Sized> NonNull<T> {
    #[stable(feature = "nonnull", since = "1.25.0")]
    #[inline]
    pub const fn dangling() -> Self {
        unsafe {
            let ptr = mem::align_of::<T>() as *mut T;
            NonNull::new_unchecked(ptr)
        }
    }
}

impl<T: ?Sized> NonNull<T> {
    #[stable(feature = "nonnull", since = "1.25.0")]
    #[inline]
    pub const unsafe fn new_unchecked(ptr: *mut T) -> Self {
        NonNull { pointer: ptr as _ }
    }

    #[stable(feature = "nonnull", since = "1.25.0")]
    #[inline]
    pub fn new(ptr: *mut T) -> Option<Self> {
        if !ptr.is_null() {
            Some(unsafe { Self::new_unchecked(ptr) })
        } else {
            None
        }
    }

    #[stable(feature = "nonnull", since = "1.25.0")]
    #[inline]
    pub const fn as_ptr(self) -> *mut T {
        self.pointer as *mut T
    }

    #[stable(feature = "nonnull", since = "1.25.0")]
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        &*self.as_ptr()
    }

    #[stable(feature = "nonnull", since = "1.25.0")]
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        &mut *self.as_ptr()
    }

    #[stable(feature = "nonnull_cast", since = "1.27.0")]
    #[inline]
    pub const fn cast<U>(self) -> NonNull<U> {
        unsafe {
            NonNull::new_unchecked(self.as_ptr() as *mut U)
        }
    }
}

#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> Clone for NonNull<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> Copy for NonNull<T> { }

#[unstable(feature = "coerce_unsized", issue = "27732")]
impl<T: ?Sized, U: ?Sized> CoerceUnsized<NonNull<U>> for NonNull<T> where T: Unsize<U> { }

#[unstable(feature = "dispatch_from_dyn", issue = "0")]
impl<T: ?Sized, U: ?Sized> DispatchFromDyn<NonNull<U>> for NonNull<T> where T: Unsize<U> { }

#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> fmt::Debug for NonNull<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.as_ptr(), f)
    }
}

#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> fmt::Pointer for NonNull<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.as_ptr(), f)
    }
}

#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> Eq for NonNull<T> {}

#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> PartialEq for NonNull<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_ptr() == other.as_ptr()
    }
}

#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> Ord for NonNull<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ptr().cmp(&other.as_ptr())
    }
}

#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> PartialOrd for NonNull<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ptr().partial_cmp(&other.as_ptr())
    }
}

#[unstable(feature = "ptr_internals", issue = "0")]
impl<T: ?Sized> From<Unique<T>> for NonNull<T> {
    #[inline]
    fn from(unique: Unique<T>) -> Self {
        unsafe { NonNull::new_unchecked(unique.as_ptr()) }
    }
}

#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> From<&mut T> for NonNull<T> {
    #[inline]
    fn from(reference: &mut T) -> Self {
        unsafe { NonNull { pointer: reference as *mut T } }
    }
}

#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> From<&T> for NonNull<T> {
    #[inline]
    fn from(reference: &T) -> Self {
        unsafe { NonNull { pointer: reference as *const T } }
    }
}
