//! Panic support for libcore

#![allow(dead_code, missing_docs)]
#![unstable(feature = "core_panic",
            reason = "internal details of the implementation of the `panic!` \
                      and related macros",
            issue = "0")]

use crate::fmt;

#[cold]
#[cfg_attr(not(feature="panic_immediate_abort"),inline(never))]
#[lang = "panic"]
pub fn panic(expr_file_line_col: &(&'static str, &'static str, u32, u32)) -> ! { loop { } }

#[cold]
#[cfg_attr(not(feature="panic_immediate_abort"),inline(never))]
#[lang = "panic_bounds_check"]
fn panic_bounds_check(file_line_col: &(&'static str, u32, u32),
                      index: usize, len: usize) -> ! { loop { } }

#[cold]
#[cfg_attr(not(feature="panic_immediate_abort"),inline(never))]
#[cfg_attr(    feature="panic_immediate_abort" ,inline)]
pub fn panic_fmt(fmt: fmt::Arguments<'_>, file_line_col: &(&'static str, u32, u32)) -> ! { loop { } }
