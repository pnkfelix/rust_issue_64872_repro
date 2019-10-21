//! Slice sorting

use crate::cmp;
use crate::mem::{self, MaybeUninit};
use crate::ptr;

struct CopyOnDrop<T> {
    src: *mut T,
    dest: *mut T,
}

impl<T> Drop for CopyOnDrop<T> {
    fn drop(&mut self) { loop { } }
}

fn shift_head<T, F>(v: &mut [T], is_less: &mut F)
    where F: FnMut(&T, &T) -> bool
{ loop { } }

fn shift_tail<T, F>(v: &mut [T], is_less: &mut F)
    where F: FnMut(&T, &T) -> bool
{ loop { } }

#[cold]
fn partial_insertion_sort<T, F>(v: &mut [T], is_less: &mut F) -> bool
    where F: FnMut(&T, &T) -> bool
{ loop { } }

fn insertion_sort<T, F>(v: &mut [T], is_less: &mut F)
    where F: FnMut(&T, &T) -> bool
{ loop { } }

#[cold]
pub fn heapsort<T, F>(v: &mut [T], is_less: &mut F)
    where F: FnMut(&T, &T) -> bool
{ loop { } }

fn partition_in_blocks<T, F>(v: &mut [T], pivot: &T, is_less: &mut F) -> usize
    where F: FnMut(&T, &T) -> bool
{ loop { } }

fn partition<T, F>(v: &mut [T], pivot: usize, is_less: &mut F) -> (usize, bool)
    where F: FnMut(&T, &T) -> bool
{ loop { } }

fn partition_equal<T, F>(v: &mut [T], pivot: usize, is_less: &mut F) -> usize
    where F: FnMut(&T, &T) -> bool
{ loop { } }

#[cold]
fn break_patterns<T>(v: &mut [T]) { loop { } }

fn choose_pivot<T, F>(v: &mut [T], is_less: &mut F) -> (usize, bool)
    where F: FnMut(&T, &T) -> bool
{ loop { } }

fn recurse<'a, T, F>(mut v: &'a mut [T], is_less: &mut F, mut pred: Option<&'a T>, mut limit: usize)
    where F: FnMut(&T, &T) -> bool
{ loop { } }

pub fn quicksort<T, F>(v: &mut [T], mut is_less: F)
    where F: FnMut(&T, &T) -> bool
{ loop { } }

fn partition_at_index_loop<'a, T, F>( mut v: &'a mut [T], mut index: usize, is_less: &mut F
                                    , mut pred: Option<&'a T>) where F: FnMut(&T, &T) -> bool
{ loop { } }

pub fn partition_at_index<T, F>(v: &mut [T], index: usize, mut is_less: F)
                                -> (&mut [T], &mut T, &mut [T]) where F: FnMut(&T, &T) -> bool
{ loop { } }
