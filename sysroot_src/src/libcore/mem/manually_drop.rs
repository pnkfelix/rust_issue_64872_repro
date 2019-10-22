use crate::ops::{Deref, DerefMut};

#[stable(feature = "manually_drop", since = "1.20.0")]
#[lang = "manually_drop"]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct ManuallyDrop<T: ?Sized> {
    value: T,
}

impl<T> ManuallyDrop<T> {
    #[stable(feature = "manually_drop", since = "1.20.0")]
    #[inline(always)]
    pub const fn new(value: T) -> ManuallyDrop<T> {
        ManuallyDrop { value }
    }

    #[stable(feature = "manually_drop", since = "1.20.0")]
    #[inline(always)]
    pub const fn into_inner(slot: ManuallyDrop<T>) -> T {
        slot.value
    }

    #[must_use = "if you don't need the value, you can use `ManuallyDrop::drop` instead"]
    #[unstable(feature = "manually_drop_take", issue = "55422")]
    #[inline]
    pub unsafe fn take(slot: &mut ManuallyDrop<T>) -> T { loop { } }
}

impl<T: ?Sized> ManuallyDrop<T> {
    #[stable(feature = "manually_drop", since = "1.20.0")]
    #[inline]
    pub unsafe fn drop(slot: &mut ManuallyDrop<T>) { loop { } }
}

#[stable(feature = "manually_drop", since = "1.20.0")]
impl<T: ?Sized> Deref for ManuallyDrop<T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &T { loop { } }
}

#[stable(feature = "manually_drop", since = "1.20.0")]
impl<T: ?Sized> DerefMut for ManuallyDrop<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut T { loop { } }
}
