//! Memory allocation APIs

#![stable(feature = "alloc_module", since = "1.28.0")]

use crate::cmp;
use crate::fmt;
use crate::mem;
use crate::usize;
use crate::ptr::{self, NonNull};
use crate::num::NonZeroUsize;

#[unstable(feature = "allocator_api", issue = "32838")]
#[derive(Debug)]
pub struct Excess(pub NonNull<u8>, pub usize);

fn size_align<T>() -> (usize, usize) { loop { } }

#[stable(feature = "alloc_layout", since = "1.28.0")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[lang = "alloc_layout"]
pub struct Layout {
    size_: usize,

    align_: NonZeroUsize,
}

impl Layout {
    #[stable(feature = "alloc_layout", since = "1.28.0")]
    #[inline]
    pub fn from_size_align(size: usize, align: usize) -> Result<Self, LayoutErr> { loop { } }

    #[stable(feature = "alloc_layout", since = "1.28.0")]
    #[inline]
    pub const unsafe fn from_size_align_unchecked(size: usize, align: usize) -> Self {
        Layout { size_: size, align_: NonZeroUsize::new_unchecked(align) }
    }

    #[stable(feature = "alloc_layout", since = "1.28.0")]
    #[inline]
    pub fn size(&self) -> usize { loop { } }

    #[stable(feature = "alloc_layout", since = "1.28.0")]
    #[inline]
    pub fn align(&self) -> usize { loop { } }

    #[stable(feature = "alloc_layout", since = "1.28.0")]
    #[inline]
    pub fn new<T>() -> Self { loop { } }

    #[stable(feature = "alloc_layout", since = "1.28.0")]
    #[inline]
    pub fn for_value<T: ?Sized>(t: &T) -> Self { loop { } }

    #[unstable(feature = "alloc_layout_extra", issue = "55724")]
    #[inline]
    pub fn align_to(&self, align: usize) -> Result<Self, LayoutErr> { loop { } }

    #[unstable(feature = "alloc_layout_extra", issue = "55724")]
    #[inline]
    pub fn padding_needed_for(&self, align: usize) -> usize { loop { } }

    #[unstable(feature = "alloc_layout_extra", issue = "55724")]
    #[inline]
    pub fn pad_to_align(&self) -> Result<Layout, LayoutErr> { loop { } }

    #[unstable(feature = "alloc_layout_extra", issue = "55724")]
    #[inline]
    pub fn repeat(&self, n: usize) -> Result<(Self, usize), LayoutErr> { loop { } }

    #[unstable(feature = "alloc_layout_extra", issue = "55724")]
    #[inline]
    pub fn extend(&self, next: Self) -> Result<(Self, usize), LayoutErr> { loop { } }

    #[unstable(feature = "alloc_layout_extra", issue = "55724")]
    #[inline]
    pub fn repeat_packed(&self, n: usize) -> Result<Self, LayoutErr> { loop { } }

    #[unstable(feature = "alloc_layout_extra", issue = "55724")]
    #[inline]
    pub fn extend_packed(&self, next: Self) -> Result<Self, LayoutErr> { loop { } }

    #[unstable(feature = "alloc_layout_extra", issue = "55724")]
    #[inline]
    pub fn array<T>(n: usize) -> Result<Self, LayoutErr> { loop { } }
}

#[stable(feature = "alloc_layout", since = "1.28.0")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LayoutErr {
    private: ()
}

#[stable(feature = "alloc_layout", since = "1.28.0")]
impl fmt::Display for LayoutErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[unstable(feature = "allocator_api", issue = "32838")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AllocErr;

#[unstable(feature = "allocator_api", issue = "32838")]
impl fmt::Display for AllocErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[unstable(feature = "allocator_api", issue = "32838")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CannotReallocInPlace;

#[unstable(feature = "allocator_api", issue = "32838")]
impl CannotReallocInPlace {
    pub fn description(&self) -> &str { loop { } }
}

#[unstable(feature = "allocator_api", issue = "32838")]
impl fmt::Display for CannotReallocInPlace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "global_alloc", since = "1.28.0")]
pub unsafe trait GlobalAlloc {
    #[stable(feature = "global_alloc", since = "1.28.0")]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8;

    #[stable(feature = "global_alloc", since = "1.28.0")]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout);

    #[stable(feature = "global_alloc", since = "1.28.0")]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 { loop { } }

    #[stable(feature = "global_alloc", since = "1.28.0")]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 { loop { } }
}

#[unstable(feature = "allocator_api", issue = "32838")]
pub unsafe trait Alloc {


    unsafe fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, AllocErr>;

    unsafe fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout);


    #[inline]
    fn usable_size(&self, layout: &Layout) -> (usize, usize) { loop { } }


    unsafe fn realloc(&mut self,
                      ptr: NonNull<u8>,
                      layout: Layout,
                      new_size: usize) -> Result<NonNull<u8>, AllocErr> { loop { } }

    unsafe fn alloc_zeroed(&mut self, layout: Layout) -> Result<NonNull<u8>, AllocErr> { loop { } }

    unsafe fn alloc_excess(&mut self, layout: Layout) -> Result<Excess, AllocErr> { loop { } }

    unsafe fn realloc_excess(&mut self,
                             ptr: NonNull<u8>,
                             layout: Layout,
                             new_size: usize) -> Result<Excess, AllocErr> { loop { } }

    unsafe fn grow_in_place(&mut self,
                            ptr: NonNull<u8>,
                            layout: Layout,
                            new_size: usize) -> Result<(), CannotReallocInPlace> { loop { } }

    unsafe fn shrink_in_place(&mut self,
                              ptr: NonNull<u8>,
                              layout: Layout,
                              new_size: usize) -> Result<(), CannotReallocInPlace> { loop { } }



    fn alloc_one<T>(&mut self) -> Result<NonNull<T>, AllocErr>
        where Self: Sized
    { loop { } }

    unsafe fn dealloc_one<T>(&mut self, ptr: NonNull<T>)
        where Self: Sized
    { loop { } }

    fn alloc_array<T>(&mut self, n: usize) -> Result<NonNull<T>, AllocErr>
        where Self: Sized
    { loop { } }

    unsafe fn realloc_array<T>(&mut self,
                               ptr: NonNull<T>,
                               n_old: usize,
                               n_new: usize) -> Result<NonNull<T>, AllocErr>
        where Self: Sized
    { loop { } }

    unsafe fn dealloc_array<T>(&mut self, ptr: NonNull<T>, n: usize) -> Result<(), AllocErr>
        where Self: Sized
    { loop { } }
}
