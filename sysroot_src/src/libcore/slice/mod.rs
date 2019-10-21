#![stable(feature = "rust1", since = "1.0.0")]


use crate::cmp::Ordering::{self};
use crate::fmt;
use crate::isize;
use crate::iter::*;
use crate::ops::{FnMut, self};
use crate::option::Option;
use crate::result::Result;
use crate::marker::{Copy, Send, Sync, Sized, self};

#[unstable(feature = "slice_internals", issue = "0",
           reason = "exposed from core to be reused in std; use the memchr crate")]
pub mod memchr;

mod rotate;
mod sort;


#[lang = "slice"]
#[cfg(not(test))]
impl<T> [T] {
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    #[allow(unused_attributes)]
    #[allow_internal_unstable(const_fn_union)]
    pub const fn len(&self) -> usize {
        unsafe {
            crate::ptr::Repr { rust: self }.raw.len
        }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn first(&self) -> Option<&T> { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn first_mut(&mut self) -> Option<&mut T> { loop { } }

    #[stable(feature = "slice_splits", since = "1.5.0")]
    #[inline]
    pub fn split_first(&self) -> Option<(&T, &[T])> { loop { } }

    #[stable(feature = "slice_splits", since = "1.5.0")]
    #[inline]
    pub fn split_first_mut(&mut self) -> Option<(&mut T, &mut [T])> { loop { } }

    #[stable(feature = "slice_splits", since = "1.5.0")]
    #[inline]
    pub fn split_last(&self) -> Option<(&T, &[T])> { loop { } }

    #[stable(feature = "slice_splits", since = "1.5.0")]
    #[inline]
    pub fn split_last_mut(&mut self) -> Option<(&mut T, &mut [T])> { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn last(&self) -> Option<&T> { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn last_mut(&mut self) -> Option<&mut T> { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn get<I>(&self, index: I) -> Option<&I::Output>
        where I: SliceIndex<Self>
    { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
        where I: SliceIndex<Self>
    { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub unsafe fn get_unchecked<I>(&self, index: I) -> &I::Output
        where I: SliceIndex<Self>
    { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub unsafe fn get_unchecked_mut<I>(&mut self, index: I) -> &mut I::Output
        where I: SliceIndex<Self>
    { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        self as *const [T] as *const T
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn swap(&mut self, a: usize, b: usize) { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn reverse(&mut self) { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn windows(&self, size: usize) -> Windows<'_, T> { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn chunks(&self, chunk_size: usize) -> Chunks<'_, T> { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn chunks_mut(&mut self, chunk_size: usize) -> ChunksMut<'_, T> { loop { } }

    #[stable(feature = "chunks_exact", since = "1.31.0")]
    #[inline]
    pub fn chunks_exact(&self, chunk_size: usize) -> ChunksExact<'_, T> { loop { } }

    #[stable(feature = "chunks_exact", since = "1.31.0")]
    #[inline]
    pub fn chunks_exact_mut(&mut self, chunk_size: usize) -> ChunksExactMut<'_, T> { loop { } }

    #[stable(feature = "rchunks", since = "1.31.0")]
    #[inline]
    pub fn rchunks(&self, chunk_size: usize) -> RChunks<'_, T> { loop { } }

    #[stable(feature = "rchunks", since = "1.31.0")]
    #[inline]
    pub fn rchunks_mut(&mut self, chunk_size: usize) -> RChunksMut<'_, T> { loop { } }

    #[stable(feature = "rchunks", since = "1.31.0")]
    #[inline]
    pub fn rchunks_exact(&self, chunk_size: usize) -> RChunksExact<'_, T> { loop { } }

    #[stable(feature = "rchunks", since = "1.31.0")]
    #[inline]
    pub fn rchunks_exact_mut(&mut self, chunk_size: usize) -> RChunksExactMut<'_, T> { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn split_at(&self, mid: usize) -> (&[T], &[T]) { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T]) { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn split<F>(&self, pred: F) -> Split<'_, T, F>
        where F: FnMut(&T) -> bool
    { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn split_mut<F>(&mut self, pred: F) -> SplitMut<'_, T, F>
        where F: FnMut(&T) -> bool
    { loop { } }

    #[stable(feature = "slice_rsplit", since = "1.27.0")]
    #[inline]
    pub fn rsplit<F>(&self, pred: F) -> RSplit<'_, T, F>
        where F: FnMut(&T) -> bool
    { loop { } }

    #[stable(feature = "slice_rsplit", since = "1.27.0")]
    #[inline]
    pub fn rsplit_mut<F>(&mut self, pred: F) -> RSplitMut<'_, T, F>
        where F: FnMut(&T) -> bool
    { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn splitn<F>(&self, n: usize, pred: F) -> SplitN<'_, T, F>
        where F: FnMut(&T) -> bool
    { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn splitn_mut<F>(&mut self, n: usize, pred: F) -> SplitNMut<'_, T, F>
        where F: FnMut(&T) -> bool
    { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn rsplitn<F>(&self, n: usize, pred: F) -> RSplitN<'_, T, F>
        where F: FnMut(&T) -> bool
    { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn rsplitn_mut<F>(&mut self, n: usize, pred: F) -> RSplitNMut<'_, T, F>
        where F: FnMut(&T) -> bool
    { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn contains(&self, x: &T) -> bool
        where T: PartialEq
    { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn starts_with(&self, needle: &[T]) -> bool
        where T: PartialEq
    { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn ends_with(&self, needle: &[T]) -> bool
        where T: PartialEq
    { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn binary_search(&self, x: &T) -> Result<usize, usize>
        where T: Ord
    { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn binary_search_by<'a, F>(&'a self, mut f: F) -> Result<usize, usize>
        where F: FnMut(&'a T) -> Ordering
    { loop { } }

    #[stable(feature = "slice_binary_search_by_key", since = "1.10.0")]
    #[inline]
    pub fn binary_search_by_key<'a, B, F>(&'a self, b: &B, mut f: F) -> Result<usize, usize>
        where F: FnMut(&'a T) -> B,
              B: Ord
    { loop { } }

    #[stable(feature = "sort_unstable", since = "1.20.0")]
    #[inline]
    pub fn sort_unstable(&mut self)
        where T: Ord
    { loop { } }

    #[stable(feature = "sort_unstable", since = "1.20.0")]
    #[inline]
    pub fn sort_unstable_by<F>(&mut self, mut compare: F)
        where F: FnMut(&T, &T) -> Ordering
    { loop { } }

    #[stable(feature = "sort_unstable", since = "1.20.0")]
    #[inline]
    pub fn sort_unstable_by_key<K, F>(&mut self, mut f: F)
        where F: FnMut(&T) -> K, K: Ord
    { loop { } }

    #[unstable(feature = "slice_partition_at_index", issue = "55300")]
    #[inline]
    pub fn partition_at_index(&mut self, index: usize) -> (&mut [T], &mut T, &mut [T])
        where T: Ord
    { loop { } }

    #[unstable(feature = "slice_partition_at_index", issue = "55300")]
    #[inline]
    pub fn partition_at_index_by<F>(&mut self, index: usize, mut compare: F)
                                    -> (&mut [T], &mut T, &mut [T])
        where F: FnMut(&T, &T) -> Ordering
    { loop { } }

    #[unstable(feature = "slice_partition_at_index", issue = "55300")]
    #[inline]
    pub fn partition_at_index_by_key<K, F>(&mut self, index: usize, mut f: F)
                                           -> (&mut [T], &mut T, &mut [T])
        where F: FnMut(&T) -> K, K: Ord
    { loop { } }

    #[unstable(feature = "slice_partition_dedup", issue = "54279")]
    #[inline]
    pub fn partition_dedup(&mut self) -> (&mut [T], &mut [T])
        where T: PartialEq
    { loop { } }

    #[unstable(feature = "slice_partition_dedup", issue = "54279")]
    #[inline]
    pub fn partition_dedup_by<F>(&mut self, mut same_bucket: F) -> (&mut [T], &mut [T])
        where F: FnMut(&mut T, &mut T) -> bool
    { loop { } }

    #[unstable(feature = "slice_partition_dedup", issue = "54279")]
    #[inline]
    pub fn partition_dedup_by_key<K, F>(&mut self, mut key: F) -> (&mut [T], &mut [T])
        where F: FnMut(&mut T) -> K,
              K: PartialEq,
    { loop { } }

    #[stable(feature = "slice_rotate", since = "1.26.0")]
    pub fn rotate_left(&mut self, mid: usize) { loop { } }

    #[stable(feature = "slice_rotate", since = "1.26.0")]
    pub fn rotate_right(&mut self, k: usize) { loop { } }

    #[stable(feature = "clone_from_slice", since = "1.7.0")]
    pub fn clone_from_slice(&mut self, src: &[T]) where T: Clone { loop { } }

    #[stable(feature = "copy_from_slice", since = "1.9.0")]
    pub fn copy_from_slice(&mut self, src: &[T]) where T: Copy { loop { } }

    #[stable(feature = "copy_within", since = "1.37.0")]
    pub fn copy_within<R: ops::RangeBounds<usize>>(&mut self, src: R, dest: usize)
    where
        T: Copy,
    { loop { } }

    #[stable(feature = "swap_with_slice", since = "1.27.0")]
    pub fn swap_with_slice(&mut self, other: &mut [T]) { loop { } }

    fn align_to_offsets<U>(&self) -> (usize, usize) { loop { } }

    #[stable(feature = "slice_align_to", since = "1.30.0")]
    pub unsafe fn align_to<U>(&self) -> (&[T], &[U], &[T]) { loop { } }

    #[stable(feature = "slice_align_to", since = "1.30.0")]
    pub unsafe fn align_to_mut<U>(&mut self) -> (&mut [T], &mut [U], &mut [T]) { loop { } }

    #[inline]
    #[unstable(feature = "is_sorted", reason = "new API", issue = "53485")]
    pub fn is_sorted(&self) -> bool
    where
        T: PartialOrd,
    { loop { } }

    #[unstable(feature = "is_sorted", reason = "new API", issue = "53485")]
    pub fn is_sorted_by<F>(&self, mut compare: F) -> bool
    where
        F: FnMut(&T, &T) -> Option<Ordering>
    { loop { } }

    #[inline]
    #[unstable(feature = "is_sorted", reason = "new API", issue = "53485")]
    pub fn is_sorted_by_key<F, K>(&self, f: F) -> bool
    where
        F: FnMut(&T) -> K,
        K: PartialOrd
    { loop { } }
}

#[lang = "slice_u8"]
#[cfg(not(test))]
impl [u8] {
    #[stable(feature = "ascii_methods_on_intrinsics", since = "1.23.0")]
    #[inline]
    pub fn is_ascii(&self) -> bool { loop { } }

    #[stable(feature = "ascii_methods_on_intrinsics", since = "1.23.0")]
    #[inline]
    pub fn eq_ignore_ascii_case(&self, other: &[u8]) -> bool { loop { } }

    #[stable(feature = "ascii_methods_on_intrinsics", since = "1.23.0")]
    #[inline]
    pub fn make_ascii_uppercase(&mut self) { loop { } }

    #[stable(feature = "ascii_methods_on_intrinsics", since = "1.23.0")]
    #[inline]
    pub fn make_ascii_lowercase(&mut self) { loop { } }

}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T, I> ops::Index<I> for [T]
    where I: SliceIndex<[T]>
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &I::Output { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T, I> ops::IndexMut<I> for [T]
    where I: SliceIndex<[T]>
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut I::Output { loop { } }
}

#[inline(never)]
#[cold]
fn slice_index_len_fail(index: usize, len: usize) -> ! { loop { } }

#[inline(never)]
#[cold]
fn slice_index_order_fail(index: usize, end: usize) -> ! { loop { } }

#[inline(never)]
#[cold]
fn slice_index_overflow_fail() -> ! { loop { } }

mod private_slice_index {
    use super::ops;
    #[stable(feature = "slice_get_slice", since = "1.28.0")]
    pub trait Sealed {}

    #[stable(feature = "slice_get_slice", since = "1.28.0")]
    impl Sealed for usize {}
    #[stable(feature = "slice_get_slice", since = "1.28.0")]
    impl Sealed for ops::Range<usize> {}
    #[stable(feature = "slice_get_slice", since = "1.28.0")]
    impl Sealed for ops::RangeTo<usize> {}
    #[stable(feature = "slice_get_slice", since = "1.28.0")]
    impl Sealed for ops::RangeFrom<usize> {}
    #[stable(feature = "slice_get_slice", since = "1.28.0")]
    impl Sealed for ops::RangeFull {}
    #[stable(feature = "slice_get_slice", since = "1.28.0")]
    impl Sealed for ops::RangeInclusive<usize> {}
    #[stable(feature = "slice_get_slice", since = "1.28.0")]
    impl Sealed for ops::RangeToInclusive<usize> {}
}

#[stable(feature = "slice_get_slice", since = "1.28.0")]
#[rustc_on_unimplemented(
    on(
        T = "str",
        label = "string indices are ranges of `usize`",
    ),
    on(
        all(any(T = "str", T = "&str", T = "std::string::String"), _Self="{integer}"),
        note="you can use `.chars().nth()` or `.bytes().nth()`
see chapter in The Book <https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings>"
    ),
    message = "the type `{T}` cannot be indexed by `{Self}`",
    label = "slice indices are of type `usize` or ranges of `usize`",
)]
pub trait SliceIndex<T: ?Sized>: private_slice_index::Sealed {
    #[stable(feature = "slice_get_slice", since = "1.28.0")]
    type Output: ?Sized;

    #[unstable(feature = "slice_index_methods", issue = "0")]
    fn get(self, slice: &T) -> Option<&Self::Output>;

    #[unstable(feature = "slice_index_methods", issue = "0")]
    fn get_mut(self, slice: &mut T) -> Option<&mut Self::Output>;

    #[unstable(feature = "slice_index_methods", issue = "0")]
    unsafe fn get_unchecked(self, slice: &T) -> &Self::Output;

    #[unstable(feature = "slice_index_methods", issue = "0")]
    unsafe fn get_unchecked_mut(self, slice: &mut T) -> &mut Self::Output;

    #[unstable(feature = "slice_index_methods", issue = "0")]
    fn index(self, slice: &T) -> &Self::Output;

    #[unstable(feature = "slice_index_methods", issue = "0")]
    fn index_mut(self, slice: &mut T) -> &mut Self::Output;
}

#[stable(feature = "slice_get_slice_impls", since = "1.15.0")]
impl<T> SliceIndex<[T]> for usize {
    type Output = T;

    #[inline]
    fn get(self, slice: &[T]) -> Option<&T> { loop { } }

    #[inline]
    fn get_mut(self, slice: &mut [T]) -> Option<&mut T> { loop { } }

    #[inline]
    unsafe fn get_unchecked(self, slice: &[T]) -> &T { loop { } }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: &mut [T]) -> &mut T { loop { } }

    #[inline]
    fn index(self, slice: &[T]) -> &T { loop { } }

    #[inline]
    fn index_mut(self, slice: &mut [T]) -> &mut T { loop { } }
}

#[stable(feature = "slice_get_slice_impls", since = "1.15.0")]
impl<T> SliceIndex<[T]> for  ops::Range<usize> {
    type Output = [T];

    #[inline]
    fn get(self, slice: &[T]) -> Option<&[T]> { loop { } }

    #[inline]
    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> { loop { } }

    #[inline]
    unsafe fn get_unchecked(self, slice: &[T]) -> &[T] { loop { } }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: &mut [T]) -> &mut [T] { loop { } }

    #[inline]
    fn index(self, slice: &[T]) -> &[T] { loop { } }

    #[inline]
    fn index_mut(self, slice: &mut [T]) -> &mut [T] { loop { } }
}

#[stable(feature = "slice_get_slice_impls", since = "1.15.0")]
impl<T> SliceIndex<[T]> for ops::RangeTo<usize> {
    type Output = [T];

    #[inline]
    fn get(self, slice: &[T]) -> Option<&[T]> { loop { } }

    #[inline]
    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> { loop { } }

    #[inline]
    unsafe fn get_unchecked(self, slice: &[T]) -> &[T] { loop { } }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: &mut [T]) -> &mut [T] { loop { } }

    #[inline]
    fn index(self, slice: &[T]) -> &[T] { loop { } }

    #[inline]
    fn index_mut(self, slice: &mut [T]) -> &mut [T] { loop { } }
}

#[stable(feature = "slice_get_slice_impls", since = "1.15.0")]
impl<T> SliceIndex<[T]> for ops::RangeFrom<usize> {
    type Output = [T];

    #[inline]
    fn get(self, slice: &[T]) -> Option<&[T]> { loop { } }

    #[inline]
    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> { loop { } }

    #[inline]
    unsafe fn get_unchecked(self, slice: &[T]) -> &[T] { loop { } }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: &mut [T]) -> &mut [T] { loop { } }

    #[inline]
    fn index(self, slice: &[T]) -> &[T] { loop { } }

    #[inline]
    fn index_mut(self, slice: &mut [T]) -> &mut [T] { loop { } }
}

#[stable(feature = "slice_get_slice_impls", since = "1.15.0")]
impl<T> SliceIndex<[T]> for ops::RangeFull {
    type Output = [T];

    #[inline]
    fn get(self, slice: &[T]) -> Option<&[T]> { loop { } }

    #[inline]
    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> { loop { } }

    #[inline]
    unsafe fn get_unchecked(self, slice: &[T]) -> &[T] { loop { } }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: &mut [T]) -> &mut [T] { loop { } }

    #[inline]
    fn index(self, slice: &[T]) -> &[T] { loop { } }

    #[inline]
    fn index_mut(self, slice: &mut [T]) -> &mut [T] { loop { } }
}


#[stable(feature = "inclusive_range", since = "1.26.0")]
impl<T> SliceIndex<[T]> for ops::RangeInclusive<usize> {
    type Output = [T];

    #[inline]
    fn get(self, slice: &[T]) -> Option<&[T]> { loop { } }

    #[inline]
    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> { loop { } }

    #[inline]
    unsafe fn get_unchecked(self, slice: &[T]) -> &[T] { loop { } }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: &mut [T]) -> &mut [T] { loop { } }

    #[inline]
    fn index(self, slice: &[T]) -> &[T] { loop { } }

    #[inline]
    fn index_mut(self, slice: &mut [T]) -> &mut [T] { loop { } }
}

#[stable(feature = "inclusive_range", since = "1.26.0")]
impl<T> SliceIndex<[T]> for ops::RangeToInclusive<usize> {
    type Output = [T];

    #[inline]
    fn get(self, slice: &[T]) -> Option<&[T]> { loop { } }

    #[inline]
    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> { loop { } }

    #[inline]
    unsafe fn get_unchecked(self, slice: &[T]) -> &[T] { loop { } }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: &mut [T]) -> &mut [T] { loop { } }

    #[inline]
    fn index(self, slice: &[T]) -> &[T] { loop { } }

    #[inline]
    fn index_mut(self, slice: &mut [T]) -> &mut [T] { loop { } }
}


#[stable(feature = "rust1", since = "1.0.0")]
impl<T> Default for &[T] {
    fn default() -> Self { loop { } }
}

#[stable(feature = "mut_slice_default", since = "1.5.0")]
impl<T> Default for &mut [T] {
    fn default() -> Self { loop { } }
}


#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> IntoIterator for &'a [T] {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> IntoIterator for &'a mut [T] {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> { loop { } }
}

#[inline(always)]
fn size_from_ptr<T>(_: *const T) -> usize { loop { } }

macro_rules! iterator {
    (
        struct $name:ident -> $ptr:ty,
        $elem:ty,
        $raw_mut:tt,
        {$( $mut_:tt )*},
        {$($extra:tt)*}
    ) => {
        impl<'a, T> $name<'a, T> {
            #[inline(always)]
            fn make_slice(&self) -> &'a [T] { loop { } }

            #[inline(always)]
            unsafe fn post_inc_start(&mut self, offset: isize) -> * $raw_mut T { loop { } }

            #[inline(always)]
            unsafe fn pre_dec_end(&mut self, offset: isize) -> * $raw_mut T { loop { } }
        }

        #[stable(feature = "rust1", since = "1.0.0")]
        impl<T> ExactSizeIterator for $name<'_, T> {
            #[inline(always)]
            fn len(&self) -> usize { loop { } }

            #[inline(always)]
            fn is_empty(&self) -> bool { loop { } }
        }

        #[stable(feature = "rust1", since = "1.0.0")]
        impl<'a, T> Iterator for $name<'a, T> {
            type Item = $elem;

            #[inline]
            fn next(&mut self) -> Option<$elem> { loop { } }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

            #[inline]
            fn count(self) -> usize { loop { } }

            #[inline]
            fn nth(&mut self, n: usize) -> Option<$elem> { loop { } }

            #[inline]
            fn last(mut self) -> Option<$elem> { loop { } }

            #[inline]
            #[rustc_inherit_overflow_checks]
            fn position<P>(&mut self, mut predicate: P) -> Option<usize> where
                Self: Sized,
                P: FnMut(Self::Item) -> bool,
            { loop { } }

            #[inline]
            fn rposition<P>(&mut self, mut predicate: P) -> Option<usize> where
                P: FnMut(Self::Item) -> bool,
                Self: Sized + ExactSizeIterator + DoubleEndedIterator
            { loop { } }

            $($extra)*
        }

        #[stable(feature = "rust1", since = "1.0.0")]
        impl<'a, T> DoubleEndedIterator for $name<'a, T> {
            #[inline]
            fn next_back(&mut self) -> Option<$elem> { loop { } }

            #[inline]
            fn nth_back(&mut self, n: usize) -> Option<$elem> { loop { } }
        }

        #[stable(feature = "fused", since = "1.26.0")]
        impl<T> FusedIterator for $name<'_, T> {}

        #[unstable(feature = "trusted_len", issue = "37572")]
        unsafe impl<T> TrustedLen for $name<'_, T> {}
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
pub struct Iter<'a, T: 'a> {
    ptr: *const T,
    end: *const T, // If T is a ZST, this is actually ptr+len.  This encoding is picked so that
    _marker: marker::PhantomData<&'a T>,
}

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<T: fmt::Debug> fmt::Debug for Iter<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<T: Sync> Sync for Iter<'_, T> {}
#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<T: Sync> Send for Iter<'_, T> {}

impl<'a, T> Iter<'a, T> {
    #[stable(feature = "iter_to_slice", since = "1.4.0")]
    pub fn as_slice(&self) -> &'a [T] { loop { } }
}

iterator!{struct Iter -> *const T, &'a T, const, {/* no mut */}, {
    fn is_sorted_by<F>(self, mut compare: F) -> bool
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Option<Ordering>,
    { loop { } }
}}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> Clone for Iter<'_, T> {
    fn clone(&self) -> Self { loop { } }
}

#[stable(feature = "slice_iter_as_ref", since = "1.13.0")]
impl<T> AsRef<[T]> for Iter<'_, T> {
    fn as_ref(&self) -> &[T] { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
pub struct IterMut<'a, T: 'a> {
    ptr: *mut T,
    end: *mut T, // If T is a ZST, this is actually ptr+len.  This encoding is picked so that
    _marker: marker::PhantomData<&'a mut T>,
}

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<T: fmt::Debug> fmt::Debug for IterMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<T: Sync> Sync for IterMut<'_, T> {}
#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<T: Send> Send for IterMut<'_, T> {}

impl<'a, T> IterMut<'a, T> {
    #[stable(feature = "iter_to_slice", since = "1.4.0")]
    pub fn into_slice(self) -> &'a mut [T] { loop { } }

    #[unstable(feature = "slice_iter_mut_as_slice", reason = "recently added", issue = "58957")]
    pub fn as_slice(&self) -> &[T] { loop { } }
}

iterator!{struct IterMut -> *mut T, &'a mut T, mut, {mut}, {}}

#[doc(hidden)]
trait SplitIter: DoubleEndedIterator {
    fn finish(&mut self) -> Option<Self::Item>;
}

#[stable(feature = "rust1", since = "1.0.0")]
pub struct Split<'a, T:'a, P> where P: FnMut(&T) -> bool { inner: &'a (T, P) }

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<T: fmt::Debug, P> fmt::Debug for Split<'_, T, P> where P: FnMut(&T) -> bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T, P> Clone for Split<'_, T, P> where P: Clone + FnMut(&T) -> bool {
    fn clone(&self) -> Self { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T, P> Iterator for Split<'a, T, P> where P: FnMut(&T) -> bool {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T, P> DoubleEndedIterator for Split<'a, T, P> where P: FnMut(&T) -> bool {
    #[inline]
    fn next_back(&mut self) -> Option<&'a [T]> { loop { } }
}

impl<'a, T, P> SplitIter for Split<'a, T, P> where P: FnMut(&T) -> bool {
    #[inline]
    fn finish(&mut self) -> Option<&'a [T]> { loop { } }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<T, P> FusedIterator for Split<'_, T, P> where P: FnMut(&T) -> bool {}

#[stable(feature = "rust1", since = "1.0.0")]
pub struct SplitMut<'a, T:'a, P> where P: FnMut(&T) -> bool {
    v: &'a mut [T],
    pred: P,
    finished: bool
}

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<T: fmt::Debug, P> fmt::Debug for SplitMut<'_, T, P> where P: FnMut(&T) -> bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

impl<'a, T, P> SplitIter for SplitMut<'a, T, P> where P: FnMut(&T) -> bool {
    #[inline]
    fn finish(&mut self) -> Option<&'a mut [T]> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T, P> Iterator for SplitMut<'a, T, P> where P: FnMut(&T) -> bool {
    type Item = &'a mut [T];

    #[inline]
    fn next(&mut self) -> Option<&'a mut [T]> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T, P> DoubleEndedIterator for SplitMut<'a, T, P> where
    P: FnMut(&T) -> bool,
{
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut [T]> { loop { } }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<T, P> FusedIterator for SplitMut<'_, T, P> where P: FnMut(&T) -> bool {}

#[stable(feature = "slice_rsplit", since = "1.27.0")]
#[derive(Clone)] // Is this correct, or does it incorrectly require `T: Clone`?
pub struct RSplit<'a, T:'a, P> where P: FnMut(&T) -> bool {
    inner: Split<'a, T, P>
}

#[stable(feature = "slice_rsplit", since = "1.27.0")]
impl<T: fmt::Debug, P> fmt::Debug for RSplit<'_, T, P> where P: FnMut(&T) -> bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "slice_rsplit", since = "1.27.0")]
impl<'a, T, P> Iterator for RSplit<'a, T, P> where P: FnMut(&T) -> bool {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }
}

#[stable(feature = "slice_rsplit", since = "1.27.0")]
impl<'a, T, P> DoubleEndedIterator for RSplit<'a, T, P> where P: FnMut(&T) -> bool {
    #[inline]
    fn next_back(&mut self) -> Option<&'a [T]> { loop { } }
}

#[stable(feature = "slice_rsplit", since = "1.27.0")]
impl<'a, T, P> SplitIter for RSplit<'a, T, P> where P: FnMut(&T) -> bool {
    #[inline]
    fn finish(&mut self) -> Option<&'a [T]> { loop { } }
}

#[stable(feature = "slice_rsplit", since = "1.27.0")]
impl<T, P> FusedIterator for RSplit<'_, T, P> where P: FnMut(&T) -> bool {}

#[stable(feature = "slice_rsplit", since = "1.27.0")]
pub struct RSplitMut<'a, T:'a, P> where P: FnMut(&T) -> bool {
    inner: SplitMut<'a, T, P>
}

#[stable(feature = "slice_rsplit", since = "1.27.0")]
impl<T: fmt::Debug, P> fmt::Debug for RSplitMut<'_, T, P> where P: FnMut(&T) -> bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "slice_rsplit", since = "1.27.0")]
impl<'a, T, P> SplitIter for RSplitMut<'a, T, P> where P: FnMut(&T) -> bool {
    #[inline]
    fn finish(&mut self) -> Option<&'a mut [T]> { loop { } }
}

#[stable(feature = "slice_rsplit", since = "1.27.0")]
impl<'a, T, P> Iterator for RSplitMut<'a, T, P> where P: FnMut(&T) -> bool {
    type Item = &'a mut [T];

    #[inline]
    fn next(&mut self) -> Option<&'a mut [T]> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }
}

#[stable(feature = "slice_rsplit", since = "1.27.0")]
impl<'a, T, P> DoubleEndedIterator for RSplitMut<'a, T, P> where
    P: FnMut(&T) -> bool,
{
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut [T]> { loop { } }
}

#[stable(feature = "slice_rsplit", since = "1.27.0")]
impl<T, P> FusedIterator for RSplitMut<'_, T, P> where P: FnMut(&T) -> bool {}

#[derive(Debug)]
struct GenericSplitN<I> {
    iter: I,
    count: usize,
}

impl<T, I: SplitIter<Item=T>> Iterator for GenericSplitN<I> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
pub struct SplitN<'a, T: 'a, P> where P: FnMut(&T) -> bool {
    inner: GenericSplitN<Split<'a, T, P>>
}

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<T: fmt::Debug, P> fmt::Debug for SplitN<'_, T, P> where P: FnMut(&T) -> bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
pub struct RSplitN<'a, T: 'a, P> where P: FnMut(&T) -> bool {
    inner: GenericSplitN<RSplit<'a, T, P>>
}

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<T: fmt::Debug, P> fmt::Debug for RSplitN<'_, T, P> where P: FnMut(&T) -> bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
pub struct SplitNMut<'a, T: 'a, P> where P: FnMut(&T) -> bool {
    inner: GenericSplitN<SplitMut<'a, T, P>>
}

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<T: fmt::Debug, P> fmt::Debug for SplitNMut<'_, T, P> where P: FnMut(&T) -> bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
pub struct RSplitNMut<'a, T: 'a, P> where P: FnMut(&T) -> bool {
    inner: GenericSplitN<RSplitMut<'a, T, P>>
}

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<T: fmt::Debug, P> fmt::Debug for RSplitNMut<'_, T, P> where P: FnMut(&T) -> bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

macro_rules! forward_iterator {
    ($name:ident: $elem:ident, $iter_of:ty) => {
        #[stable(feature = "rust1", since = "1.0.0")]
        impl<'a, $elem, P> Iterator for $name<'a, $elem, P> where
            P: FnMut(&T) -> bool
        {
            type Item = $iter_of;

            #[inline]
            fn next(&mut self) -> Option<$iter_of> { loop { } }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) { loop { } }
        }

        #[stable(feature = "fused", since = "1.26.0")]
        impl<'a, $elem, P> FusedIterator for $name<'a, $elem, P>
            where P: FnMut(&T) -> bool {}
    }
}

forward_iterator! { SplitN: T, &'a [T] }
forward_iterator! { RSplitN: T, &'a [T] }
forward_iterator! { SplitNMut: T, &'a mut [T] }
forward_iterator! { RSplitNMut: T, &'a mut [T] }

#[derive(Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Windows<'a, T:'a> {
    v: &'a [T],
    size: usize
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> Clone for Windows<'_, T> {
    fn clone(&self) -> Self { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> Iterator for Windows<'a, T> {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn count(self) -> usize { loop { } }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> { loop { } }

    #[inline]
    fn last(self) -> Option<Self::Item> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> DoubleEndedIterator for Windows<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a [T]> { loop { } }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> ExactSizeIterator for Windows<'_, T> {}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<T> TrustedLen for Windows<'_, T> {}

#[stable(feature = "fused", since = "1.26.0")]
impl<T> FusedIterator for Windows<'_, T> {}

#[doc(hidden)]
unsafe impl<'a, T> TrustedRandomAccess for Windows<'a, T> {
    unsafe fn get_unchecked(&mut self, i: usize) -> &'a [T] { loop { } }
    fn may_have_side_effect() -> bool { loop { } }
}

#[derive(Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Chunks<'a, T:'a> {
    v: &'a [T],
    chunk_size: usize
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> Clone for Chunks<'_, T> {
    fn clone(&self) -> Self { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> Iterator for Chunks<'a, T> {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn count(self) -> usize { loop { } }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> { loop { } }

    #[inline]
    fn last(self) -> Option<Self::Item> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> DoubleEndedIterator for Chunks<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a [T]> { loop { } }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> ExactSizeIterator for Chunks<'_, T> {}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<T> TrustedLen for Chunks<'_, T> {}

#[stable(feature = "fused", since = "1.26.0")]
impl<T> FusedIterator for Chunks<'_, T> {}

#[doc(hidden)]
unsafe impl<'a, T> TrustedRandomAccess for Chunks<'a, T> {
    unsafe fn get_unchecked(&mut self, i: usize) -> &'a [T] { loop { } }
    fn may_have_side_effect() -> bool { loop { } }
}

#[derive(Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct ChunksMut<'a, T:'a> {
    v: &'a mut [T],
    chunk_size: usize
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> Iterator for ChunksMut<'a, T> {
    type Item = &'a mut [T];

    #[inline]
    fn next(&mut self) -> Option<&'a mut [T]> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn count(self) -> usize { loop { } }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<&'a mut [T]> { loop { } }

    #[inline]
    fn last(self) -> Option<Self::Item> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> DoubleEndedIterator for ChunksMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut [T]> { loop { } }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> ExactSizeIterator for ChunksMut<'_, T> {}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<T> TrustedLen for ChunksMut<'_, T> {}

#[stable(feature = "fused", since = "1.26.0")]
impl<T> FusedIterator for ChunksMut<'_, T> {}

#[doc(hidden)]
unsafe impl<'a, T> TrustedRandomAccess for ChunksMut<'a, T> {
    unsafe fn get_unchecked(&mut self, i: usize) -> &'a mut [T] { loop { } }
    fn may_have_side_effect() -> bool { loop { } }
}

#[derive(Debug)]
#[stable(feature = "chunks_exact", since = "1.31.0")]
pub struct ChunksExact<'a, T:'a> {
    v: &'a [T],
    rem: &'a [T],
    chunk_size: usize
}

impl<'a, T> ChunksExact<'a, T> {
    #[stable(feature = "chunks_exact", since = "1.31.0")]
    pub fn remainder(&self) -> &'a [T] { loop { } }
}

#[stable(feature = "chunks_exact", since = "1.31.0")]
impl<T> Clone for ChunksExact<'_, T> {
    fn clone(&self) -> Self { loop { } }
}

#[stable(feature = "chunks_exact", since = "1.31.0")]
impl<'a, T> Iterator for ChunksExact<'a, T> {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn count(self) -> usize { loop { } }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> { loop { } }

    #[inline]
    fn last(mut self) -> Option<Self::Item> { loop { } }
}

#[stable(feature = "chunks_exact", since = "1.31.0")]
impl<'a, T> DoubleEndedIterator for ChunksExact<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a [T]> { loop { } }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> { loop { } }
}

#[stable(feature = "chunks_exact", since = "1.31.0")]
impl<T> ExactSizeIterator for ChunksExact<'_, T> {
    fn is_empty(&self) -> bool { loop { } }
}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<T> TrustedLen for ChunksExact<'_, T> {}

#[stable(feature = "chunks_exact", since = "1.31.0")]
impl<T> FusedIterator for ChunksExact<'_, T> {}

#[doc(hidden)]
#[stable(feature = "chunks_exact", since = "1.31.0")]
unsafe impl<'a, T> TrustedRandomAccess for ChunksExact<'a, T> {
    unsafe fn get_unchecked(&mut self, i: usize) -> &'a [T] { loop { } }
    fn may_have_side_effect() -> bool { loop { } }
}

#[derive(Debug)]
#[stable(feature = "chunks_exact", since = "1.31.0")]
pub struct ChunksExactMut<'a, T:'a> {
    v: &'a mut [T],
    rem: &'a mut [T],
    chunk_size: usize
}

impl<'a, T> ChunksExactMut<'a, T> {
    #[stable(feature = "chunks_exact", since = "1.31.0")]
    pub fn into_remainder(self) -> &'a mut [T] { loop { } }
}

#[stable(feature = "chunks_exact", since = "1.31.0")]
impl<'a, T> Iterator for ChunksExactMut<'a, T> {
    type Item = &'a mut [T];

    #[inline]
    fn next(&mut self) -> Option<&'a mut [T]> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn count(self) -> usize { loop { } }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<&'a mut [T]> { loop { } }

    #[inline]
    fn last(mut self) -> Option<Self::Item> { loop { } }
}

#[stable(feature = "chunks_exact", since = "1.31.0")]
impl<'a, T> DoubleEndedIterator for ChunksExactMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut [T]> { loop { } }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> { loop { } }
}

#[stable(feature = "chunks_exact", since = "1.31.0")]
impl<T> ExactSizeIterator for ChunksExactMut<'_, T> {
    fn is_empty(&self) -> bool { loop { } }
}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<T> TrustedLen for ChunksExactMut<'_, T> {}

#[stable(feature = "chunks_exact", since = "1.31.0")]
impl<T> FusedIterator for ChunksExactMut<'_, T> {}

#[doc(hidden)]
#[stable(feature = "chunks_exact", since = "1.31.0")]
unsafe impl<'a, T> TrustedRandomAccess for ChunksExactMut<'a, T> {
    unsafe fn get_unchecked(&mut self, i: usize) -> &'a mut [T] { loop { } }
    fn may_have_side_effect() -> bool { loop { } }
}

#[derive(Debug)]
#[stable(feature = "rchunks", since = "1.31.0")]
pub struct RChunks<'a, T:'a> {
    v: &'a [T],
    chunk_size: usize
}

#[stable(feature = "rchunks", since = "1.31.0")]
impl<T> Clone for RChunks<'_, T> {
    fn clone(&self) -> Self { loop { } }
}

#[stable(feature = "rchunks", since = "1.31.0")]
impl<'a, T> Iterator for RChunks<'a, T> {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn count(self) -> usize { loop { } }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> { loop { } }

    #[inline]
    fn last(self) -> Option<Self::Item> { loop { } }
}

#[stable(feature = "rchunks", since = "1.31.0")]
impl<'a, T> DoubleEndedIterator for RChunks<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a [T]> { loop { } }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> { loop { } }
}

#[stable(feature = "rchunks", since = "1.31.0")]
impl<T> ExactSizeIterator for RChunks<'_, T> {}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<T> TrustedLen for RChunks<'_, T> {}

#[stable(feature = "rchunks", since = "1.31.0")]
impl<T> FusedIterator for RChunks<'_, T> {}

#[doc(hidden)]
#[stable(feature = "rchunks", since = "1.31.0")]
unsafe impl<'a, T> TrustedRandomAccess for RChunks<'a, T> {
    unsafe fn get_unchecked(&mut self, i: usize) -> &'a [T] { loop { } }
    fn may_have_side_effect() -> bool { loop { } }
}

#[derive(Debug)]
#[stable(feature = "rchunks", since = "1.31.0")]
pub struct RChunksMut<'a, T:'a> {
    v: &'a mut [T],
    chunk_size: usize
}

#[stable(feature = "rchunks", since = "1.31.0")]
impl<'a, T> Iterator for RChunksMut<'a, T> {
    type Item = &'a mut [T];

    #[inline]
    fn next(&mut self) -> Option<&'a mut [T]> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn count(self) -> usize { loop { } }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<&'a mut [T]> { loop { } }

    #[inline]
    fn last(self) -> Option<Self::Item> { loop { } }
}

#[stable(feature = "rchunks", since = "1.31.0")]
impl<'a, T> DoubleEndedIterator for RChunksMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut [T]> { loop { } }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> { loop { } }
}

#[stable(feature = "rchunks", since = "1.31.0")]
impl<T> ExactSizeIterator for RChunksMut<'_, T> {}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<T> TrustedLen for RChunksMut<'_, T> {}

#[stable(feature = "rchunks", since = "1.31.0")]
impl<T> FusedIterator for RChunksMut<'_, T> {}

#[doc(hidden)]
#[stable(feature = "rchunks", since = "1.31.0")]
unsafe impl<'a, T> TrustedRandomAccess for RChunksMut<'a, T> {
    unsafe fn get_unchecked(&mut self, i: usize) -> &'a mut [T] { loop { } }
    fn may_have_side_effect() -> bool { loop { } }
}

#[derive(Debug)]
#[stable(feature = "rchunks", since = "1.31.0")]
pub struct RChunksExact<'a, T:'a> {
    v: &'a [T],
    rem: &'a [T],
    chunk_size: usize
}

impl<'a, T> RChunksExact<'a, T> {
    #[stable(feature = "rchunks", since = "1.31.0")]
    pub fn remainder(&self) -> &'a [T] { loop { } }
}

#[stable(feature = "rchunks", since = "1.31.0")]
impl<'a, T> Clone for RChunksExact<'a, T> {
    fn clone(&self) -> RChunksExact<'a, T> { loop { } }
}

#[stable(feature = "rchunks", since = "1.31.0")]
impl<'a, T> Iterator for RChunksExact<'a, T> {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn count(self) -> usize { loop { } }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> { loop { } }

    #[inline]
    fn last(mut self) -> Option<Self::Item> { loop { } }
}

#[stable(feature = "rchunks", since = "1.31.0")]
impl<'a, T> DoubleEndedIterator for RChunksExact<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a [T]> { loop { } }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> { loop { } }
}

#[stable(feature = "rchunks", since = "1.31.0")]
impl<'a, T> ExactSizeIterator for RChunksExact<'a, T> {
    fn is_empty(&self) -> bool { loop { } }
}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<T> TrustedLen for RChunksExact<'_, T> {}

#[stable(feature = "rchunks", since = "1.31.0")]
impl<T> FusedIterator for RChunksExact<'_, T> {}

#[doc(hidden)]
#[stable(feature = "rchunks", since = "1.31.0")]
unsafe impl<'a, T> TrustedRandomAccess for RChunksExact<'a, T> {
    unsafe fn get_unchecked(&mut self, i: usize) -> &'a [T] { loop { } }
    fn may_have_side_effect() -> bool { loop { } }
}

#[derive(Debug)]
#[stable(feature = "rchunks", since = "1.31.0")]
pub struct RChunksExactMut<'a, T:'a> {
    v: &'a mut [T],
    rem: &'a mut [T],
    chunk_size: usize
}

impl<'a, T> RChunksExactMut<'a, T> {
    #[stable(feature = "rchunks", since = "1.31.0")]
    pub fn into_remainder(self) -> &'a mut [T] { loop { } }
}

#[stable(feature = "rchunks", since = "1.31.0")]
impl<'a, T> Iterator for RChunksExactMut<'a, T> {
    type Item = &'a mut [T];

    #[inline]
    fn next(&mut self) -> Option<&'a mut [T]> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn count(self) -> usize { loop { } }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<&'a mut [T]> { loop { } }

    #[inline]
    fn last(mut self) -> Option<Self::Item> { loop { } }
}

#[stable(feature = "rchunks", since = "1.31.0")]
impl<'a, T> DoubleEndedIterator for RChunksExactMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut [T]> { loop { } }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> { loop { } }
}

#[stable(feature = "rchunks", since = "1.31.0")]
impl<T> ExactSizeIterator for RChunksExactMut<'_, T> {
    fn is_empty(&self) -> bool { loop { } }
}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<T> TrustedLen for RChunksExactMut<'_, T> {}

#[stable(feature = "rchunks", since = "1.31.0")]
impl<T> FusedIterator for RChunksExactMut<'_, T> {}

#[doc(hidden)]
#[stable(feature = "rchunks", since = "1.31.0")]
unsafe impl<'a, T> TrustedRandomAccess for RChunksExactMut<'a, T> {
    unsafe fn get_unchecked(&mut self, i: usize) -> &'a mut [T] { loop { } }
    fn may_have_side_effect() -> bool { loop { } }
}


#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
pub unsafe fn from_raw_parts<'a, T>(data: *const T, len: usize) -> &'a [T] { loop { } }

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
pub unsafe fn from_raw_parts_mut<'a, T>(data: *mut T, len: usize) -> &'a mut [T] { loop { } }

#[stable(feature = "from_ref", since = "1.28.0")]
pub fn from_ref<T>(s: &T) -> &[T] { loop { } }

#[stable(feature = "from_ref", since = "1.28.0")]
pub fn from_mut<T>(s: &mut T) -> &mut [T] { loop { } }

#[unstable(feature = "sort_internals", reason = "internal to sort module", issue = "0")]
#[doc(hidden)]
pub fn heapsort<T, F>(v: &mut [T], mut is_less: F)
    where F: FnMut(&T, &T) -> bool
{ loop { } }


extern {
    fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32;
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<A, B> PartialEq<[B]> for [A] where A: PartialEq<B> {
    fn eq(&self, other: &[B]) -> bool { loop { } }

    fn ne(&self, other: &[B]) -> bool { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: Eq> Eq for [T] {}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: Ord> Ord for [T] {
    fn cmp(&self, other: &[T]) -> Ordering { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: PartialOrd> PartialOrd for [T] {
    fn partial_cmp(&self, other: &[T]) -> Option<Ordering> { loop { } }
}

#[doc(hidden)]
trait SlicePartialEq<B> {
    fn equal(&self, other: &[B]) -> bool;

    fn not_equal(&self, other: &[B]) -> bool { loop { } }
}

impl<A, B> SlicePartialEq<B> for [A]
    where A: PartialEq<B>
{
    default fn equal(&self, other: &[B]) -> bool { loop { } }
}

impl<A> SlicePartialEq<A> for [A]
    where A: PartialEq<A> + Eq
{
    default fn equal(&self, other: &[A]) -> bool { loop { } }
}

impl<A> SlicePartialEq<A> for [A]
    where A: PartialEq<A> + BytewiseEquality
{
    fn equal(&self, other: &[A]) -> bool { loop { } }
}

#[doc(hidden)]
trait SlicePartialOrd<B> {
    fn partial_compare(&self, other: &[B]) -> Option<Ordering>;
}

impl<A> SlicePartialOrd<A> for [A]
    where A: PartialOrd
{
    default fn partial_compare(&self, other: &[A]) -> Option<Ordering> { loop { } }
}

impl<A> SlicePartialOrd<A> for [A]
    where A: Ord
{
    default fn partial_compare(&self, other: &[A]) -> Option<Ordering> { loop { } }
}

#[doc(hidden)]
trait SliceOrd<B> {
    fn compare(&self, other: &[B]) -> Ordering;
}

impl<A> SliceOrd<A> for [A]
    where A: Ord
{
    default fn compare(&self, other: &[A]) -> Ordering { loop { } }
}

impl SliceOrd<u8> for [u8] {
    #[inline]
    fn compare(&self, other: &[u8]) -> Ordering { loop { } }
}

#[doc(hidden)]
trait BytewiseEquality: Eq + Copy { }

macro_rules! impl_marker_for {
    ($traitname:ident, $($ty:ty)*) => {
        $(
            impl $traitname for $ty { }
        )*
    }
}

impl_marker_for!(BytewiseEquality,
                 u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize char bool);

#[doc(hidden)]
unsafe impl<'a, T> TrustedRandomAccess for Iter<'a, T> {
    unsafe fn get_unchecked(&mut self, i: usize) -> &'a T { loop { } }
    fn may_have_side_effect() -> bool { loop { } }
}

#[doc(hidden)]
unsafe impl<'a, T> TrustedRandomAccess for IterMut<'a, T> {
    unsafe fn get_unchecked(&mut self, i: usize) -> &'a mut T { loop { } }
    fn may_have_side_effect() -> bool { loop { } }
}

trait SliceContains: Sized {
    fn slice_contains(&self, x: &[Self]) -> bool;
}

impl<T> SliceContains for T where T: PartialEq {
    default fn slice_contains(&self, x: &[Self]) -> bool { loop { } }
}

impl SliceContains for u8 {
    fn slice_contains(&self, x: &[Self]) -> bool { loop { } }
}

impl SliceContains for i8 {
    fn slice_contains(&self, x: &[Self]) -> bool { loop { } }
}
