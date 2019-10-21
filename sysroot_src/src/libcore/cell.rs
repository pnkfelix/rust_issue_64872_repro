//! Shareable mutable containers.

#![stable(feature = "rust1", since = "1.0.0")]

use crate::cmp::Ordering;
use crate::fmt::{self, Debug, Display};
use crate::marker::Unsize;
use crate::ops::{Deref, DerefMut, CoerceUnsized};

#[stable(feature = "rust1", since = "1.0.0")]
#[repr(transparent)]
pub struct Cell<T: ?Sized> {
    value: UnsafeCell<T>,
}

#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<T: ?Sized> Send for Cell<T> where T: Send {}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> !Sync for Cell<T> {}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T:Copy> Clone for Cell<T> {
    #[inline]
    fn clone(&self) -> Cell<T> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: Default> Default for Cell<T> {
    #[inline]
    fn default() -> Cell<T> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: PartialEq + Copy> PartialEq for Cell<T> {
    #[inline]
    fn eq(&self, other: &Cell<T>) -> bool { loop { } }
}

#[stable(feature = "cell_eq", since = "1.2.0")]
impl<T: Eq + Copy> Eq for Cell<T> {}

#[stable(feature = "cell_ord", since = "1.10.0")]
impl<T: PartialOrd + Copy> PartialOrd for Cell<T> {
    #[inline]
    fn partial_cmp(&self, other: &Cell<T>) -> Option<Ordering> { loop { } }

    #[inline]
    fn lt(&self, other: &Cell<T>) -> bool { loop { } }

    #[inline]
    fn le(&self, other: &Cell<T>) -> bool { loop { } }

    #[inline]
    fn gt(&self, other: &Cell<T>) -> bool { loop { } }

    #[inline]
    fn ge(&self, other: &Cell<T>) -> bool { loop { } }
}

#[stable(feature = "cell_ord", since = "1.10.0")]
impl<T: Ord + Copy> Ord for Cell<T> {
    #[inline]
    fn cmp(&self, other: &Cell<T>) -> Ordering { loop { } }
}

#[stable(feature = "cell_from", since = "1.12.0")]
impl<T> From<T> for Cell<T> {
    fn from(t: T) -> Cell<T> { loop { } }
}

impl<T> Cell<T> {
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub const fn new(value: T) -> Cell<T> {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn set(&self, val: T) { loop { } }

    #[inline]
    #[stable(feature = "move_cell", since = "1.17.0")]
    pub fn swap(&self, other: &Self) { loop { } }

    #[stable(feature = "move_cell", since = "1.17.0")]
    pub fn replace(&self, val: T) -> T { loop { } }

    #[stable(feature = "move_cell", since = "1.17.0")]
    pub fn into_inner(self) -> T { loop { } }
}

impl<T:Copy> Cell<T> {
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn get(&self) -> T { loop { } }

    #[inline]
    #[unstable(feature = "cell_update", issue = "50186")]
    pub fn update<F>(&self, f: F) -> T
    where
        F: FnOnce(T) -> T,
    { loop { } }
}

impl<T: ?Sized> Cell<T> {
    #[inline]
    #[stable(feature = "cell_as_ptr", since = "1.12.0")]
    pub const fn as_ptr(&self) -> *mut T {
        self.value.get()
    }

    #[inline]
    #[stable(feature = "cell_get_mut", since = "1.11.0")]
    pub fn get_mut(&mut self) -> &mut T { loop { } }

    #[inline]
    #[stable(feature = "as_cell", since = "1.37.0")]
    pub fn from_mut(t: &mut T) -> &Cell<T> { loop { } }
}

impl<T: Default> Cell<T> {
    #[stable(feature = "move_cell", since = "1.17.0")]
    pub fn take(&self) -> T { loop { } }
}

#[unstable(feature = "coerce_unsized", issue = "27732")]
impl<T: CoerceUnsized<U>, U> CoerceUnsized<Cell<U>> for Cell<T> {}

impl<T> Cell<[T]> {
    #[stable(feature = "as_cell", since = "1.37.0")]
    pub fn as_slice_of_cells(&self) -> &[Cell<T>] { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
pub struct RefCell<T: ?Sized> {
    borrow: Cell<BorrowFlag>,
    value: UnsafeCell<T>,
}

#[stable(feature = "try_borrow", since = "1.13.0")]
pub struct BorrowError {
    _private: (),
}

#[stable(feature = "try_borrow", since = "1.13.0")]
impl Debug for BorrowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "try_borrow", since = "1.13.0")]
impl Display for BorrowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "try_borrow", since = "1.13.0")]
pub struct BorrowMutError {
    _private: (),
}

#[stable(feature = "try_borrow", since = "1.13.0")]
impl Debug for BorrowMutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "try_borrow", since = "1.13.0")]
impl Display for BorrowMutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

type BorrowFlag = isize;
const UNUSED: BorrowFlag = 0;

#[inline(always)]
fn is_writing(x: BorrowFlag) -> bool { loop { } }

#[inline(always)]
fn is_reading(x: BorrowFlag) -> bool { loop { } }

impl<T> RefCell<T> {
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub const fn new(value: T) -> RefCell<T> {
        RefCell {
            value: UnsafeCell::new(value),
            borrow: Cell::new(UNUSED),
        }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn into_inner(self) -> T { loop { } }

    #[inline]
    #[stable(feature = "refcell_replace", since="1.24.0")]
    pub fn replace(&self, t: T) -> T { loop { } }

    #[inline]
    #[stable(feature = "refcell_replace_swap", since="1.35.0")]
    pub fn replace_with<F: FnOnce(&mut T) -> T>(&self, f: F) -> T { loop { } }

    #[inline]
    #[stable(feature = "refcell_swap", since="1.24.0")]
    pub fn swap(&self, other: &Self) { loop { } }
}

impl<T: ?Sized> RefCell<T> {
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn borrow(&self) -> Ref<'_, T> { loop { } }

    #[stable(feature = "try_borrow", since = "1.13.0")]
    #[inline]
    pub fn try_borrow(&self) -> Result<Ref<'_, T>, BorrowError> { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn borrow_mut(&self) -> RefMut<'_, T> { loop { } }

    #[stable(feature = "try_borrow", since = "1.13.0")]
    #[inline]
    pub fn try_borrow_mut(&self) -> Result<RefMut<'_, T>, BorrowMutError> { loop { } }

    #[inline]
    #[stable(feature = "cell_as_ptr", since = "1.12.0")]
    pub fn as_ptr(&self) -> *mut T { loop { } }

    #[inline]
    #[stable(feature = "cell_get_mut", since = "1.11.0")]
    pub fn get_mut(&mut self) -> &mut T { loop { } }

    #[stable(feature = "borrow_state", since = "1.37.0")]
    #[inline]
    pub unsafe fn try_borrow_unguarded(&self) -> Result<&T, BorrowError> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<T: ?Sized> Send for RefCell<T> where T: Send {}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> !Sync for RefCell<T> {}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: Clone> Clone for RefCell<T> {
    #[inline]
    fn clone(&self) -> RefCell<T> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: Default> Default for RefCell<T> {
    #[inline]
    fn default() -> RefCell<T> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized + PartialEq> PartialEq for RefCell<T> {
    #[inline]
    fn eq(&self, other: &RefCell<T>) -> bool { loop { } }
}

#[stable(feature = "cell_eq", since = "1.2.0")]
impl<T: ?Sized + Eq> Eq for RefCell<T> {}

#[stable(feature = "cell_ord", since = "1.10.0")]
impl<T: ?Sized + PartialOrd> PartialOrd for RefCell<T> {
    #[inline]
    fn partial_cmp(&self, other: &RefCell<T>) -> Option<Ordering> { loop { } }

    #[inline]
    fn lt(&self, other: &RefCell<T>) -> bool { loop { } }

    #[inline]
    fn le(&self, other: &RefCell<T>) -> bool { loop { } }

    #[inline]
    fn gt(&self, other: &RefCell<T>) -> bool { loop { } }

    #[inline]
    fn ge(&self, other: &RefCell<T>) -> bool { loop { } }
}

#[stable(feature = "cell_ord", since = "1.10.0")]
impl<T: ?Sized + Ord> Ord for RefCell<T> {
    #[inline]
    fn cmp(&self, other: &RefCell<T>) -> Ordering { loop { } }
}

#[stable(feature = "cell_from", since = "1.12.0")]
impl<T> From<T> for RefCell<T> {
    fn from(t: T) -> RefCell<T> { loop { } }
}

#[unstable(feature = "coerce_unsized", issue = "27732")]
impl<T: CoerceUnsized<U>, U> CoerceUnsized<RefCell<U>> for RefCell<T> {}

struct BorrowRef<'b> {
    borrow: &'b Cell<BorrowFlag>,
}

impl<'b> BorrowRef<'b> {
    #[inline]
    fn new(borrow: &'b Cell<BorrowFlag>) -> Option<BorrowRef<'b>> { loop { } }
}

impl Drop for BorrowRef<'_> {
    #[inline]
    fn drop(&mut self) { loop { } }
}

impl Clone for BorrowRef<'_> {
    #[inline]
    fn clone(&self) -> Self { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
pub struct Ref<'b, T: ?Sized + 'b> {
    value: &'b T,
    borrow: BorrowRef<'b>,
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Deref for Ref<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T { loop { } }
}

impl<'b, T: ?Sized> Ref<'b, T> {
    #[stable(feature = "cell_extras", since = "1.15.0")]
    #[inline]
    pub fn clone(orig: &Ref<'b, T>) -> Ref<'b, T> { loop { } }

    #[stable(feature = "cell_map", since = "1.8.0")]
    #[inline]
    pub fn map<U: ?Sized, F>(orig: Ref<'b, T>, f: F) -> Ref<'b, U>
        where F: FnOnce(&T) -> &U
    { loop { } }

    #[stable(feature = "refcell_map_split", since = "1.35.0")]
    #[inline]
    pub fn map_split<U: ?Sized, V: ?Sized, F>(orig: Ref<'b, T>, f: F) -> (Ref<'b, U>, Ref<'b, V>)
        where F: FnOnce(&T) -> (&U, &V)
    { loop { } }
}

#[unstable(feature = "coerce_unsized", issue = "27732")]
impl<'b, T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<Ref<'b, U>> for Ref<'b, T> {}

#[stable(feature = "std_guard_impls", since = "1.20.0")]
impl<T: ?Sized + fmt::Display> fmt::Display for Ref<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

impl<'b, T: ?Sized> RefMut<'b, T> {
    #[stable(feature = "cell_map", since = "1.8.0")]
    #[inline]
    pub fn map<U: ?Sized, F>(orig: RefMut<'b, T>, f: F) -> RefMut<'b, U>
        where F: FnOnce(&mut T) -> &mut U
    { loop { } }

    #[stable(feature = "refcell_map_split", since = "1.35.0")]
    #[inline]
    pub fn map_split<U: ?Sized, V: ?Sized, F>(
        orig: RefMut<'b, T>, f: F
    ) -> (RefMut<'b, U>, RefMut<'b, V>)
        where F: FnOnce(&mut T) -> (&mut U, &mut V)
    { loop { } }
}

struct BorrowRefMut<'b> {
    borrow: &'b Cell<BorrowFlag>,
}

impl Drop for BorrowRefMut<'_> {
    #[inline]
    fn drop(&mut self) { loop { } }
}

impl<'b> BorrowRefMut<'b> {
    #[inline]
    fn new(borrow: &'b Cell<BorrowFlag>) -> Option<BorrowRefMut<'b>> { loop { } }

    #[inline]
    fn clone(&self) -> BorrowRefMut<'b> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
pub struct RefMut<'b, T: ?Sized + 'b> {
    value: &'b mut T,
    borrow: BorrowRefMut<'b>,
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Deref for RefMut<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> DerefMut for RefMut<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T { loop { } }
}

#[unstable(feature = "coerce_unsized", issue = "27732")]
impl<'b, T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<RefMut<'b, U>> for RefMut<'b, T> {}

#[stable(feature = "std_guard_impls", since = "1.20.0")]
impl<T: ?Sized + fmt::Display> fmt::Display for RefMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[lang = "unsafe_cell"]
#[stable(feature = "rust1", since = "1.0.0")]
#[repr(transparent)]
pub struct UnsafeCell<T: ?Sized> {
    value: T,
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> !Sync for UnsafeCell<T> {}

impl<T> UnsafeCell<T> {
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub const fn new(value: T) -> UnsafeCell<T> {
        UnsafeCell { value }
    }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn into_inner(self) -> T { loop { } }
}

impl<T: ?Sized> UnsafeCell<T> {
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub const fn get(&self) -> *mut T {
        self as *const UnsafeCell<T> as *const T as *mut T
    }
}

#[stable(feature = "unsafe_cell_default", since = "1.10.0")]
impl<T: Default> Default for UnsafeCell<T> {
    fn default() -> UnsafeCell<T> { loop { } }
}

#[stable(feature = "cell_from", since = "1.12.0")]
impl<T> From<T> for UnsafeCell<T> {
    fn from(t: T) -> UnsafeCell<T> { loop { } }
}

#[unstable(feature = "coerce_unsized", issue = "27732")]
impl<T: CoerceUnsized<U>, U> CoerceUnsized<UnsafeCell<U>> for UnsafeCell<T> {}

#[allow(unused)]
fn assert_coerce_unsized(a: UnsafeCell<&i32>, b: Cell<&i32>, c: RefCell<&i32>) { loop { } }
