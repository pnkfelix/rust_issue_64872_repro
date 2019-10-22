use crate::mem::ManuallyDrop;

#[allow(missing_debug_implementations)]
#[stable(feature = "maybe_uninit", since = "1.36.0")]
#[lang = "maybe_uninit"]
#[repr(transparent)]
pub union MaybeUninit<T> {
    uninit: (),
    value: ManuallyDrop<T>,
}

impl<T> MaybeUninit<T> {
    #[stable(feature = "maybe_uninit", since = "1.36.0")]
    #[inline(always)]
    pub const fn new(val: T) -> MaybeUninit<T> {
        MaybeUninit { value: ManuallyDrop::new(val) }
    }

    #[stable(feature = "maybe_uninit", since = "1.36.0")]
    #[inline(always)]
    pub const fn uninit() -> MaybeUninit<T> {
        MaybeUninit { uninit: () }
    }

    #[unstable(feature = "internal_uninit_const", issue = "0",
        reason = "hack to work around promotability")]
    pub const UNINIT: Self = Self::uninit();

    #[stable(feature = "maybe_uninit", since = "1.36.0")]
    #[inline]
    pub fn zeroed() -> MaybeUninit<T> { loop { } }

    #[unstable(feature = "maybe_uninit_extra", issue = "63567")]
    #[inline(always)]
    pub fn write(&mut self, val: T) -> &mut T { loop { } }

    #[stable(feature = "maybe_uninit", since = "1.36.0")]
    #[inline(always)]
    pub fn as_ptr(&self) -> *const T { loop { } }

    #[stable(feature = "maybe_uninit", since = "1.36.0")]
    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut T { loop { } }

    #[stable(feature = "maybe_uninit", since = "1.36.0")]
    #[inline(always)]
    pub unsafe fn assume_init(self) -> T { loop { } }

    #[unstable(feature = "maybe_uninit_extra", issue = "63567")]
    #[inline(always)]
    pub unsafe fn read(&self) -> T { loop { } }

    #[unstable(feature = "maybe_uninit_ref", issue = "63568")]
    #[inline(always)]
    pub unsafe fn get_ref(&self) -> &T { loop { } }

    #[unstable(feature = "maybe_uninit_ref", issue = "63568")]
    #[inline(always)]
    pub unsafe fn get_mut(&mut self) -> &mut T { loop { } }

    #[unstable(feature = "maybe_uninit_slice", issue = "63569")]
    #[inline(always)]
    pub fn first_ptr(this: &[MaybeUninit<T>]) -> *const T { loop { } }

    #[unstable(feature = "maybe_uninit_slice", issue = "63569")]
    #[inline(always)]
    pub fn first_ptr_mut(this: &mut [MaybeUninit<T>]) -> *mut T { loop { } }
}
