//! Panic support in the standard library.

#![unstable(feature = "core_panic_info",
            reason = "newly available in libcore",
            issue = "44489")]

use crate::any::Any;
use crate::fmt;

#[lang = "panic_info"]
#[stable(feature = "panic_hooks", since = "1.10.0")]
#[derive(Debug)]
pub struct PanicInfo<'a> {
    payload: &'a (dyn Any + Send),
    message: Option<&'a fmt::Arguments<'a>>,
    location: Location<'a>,
}

impl<'a> PanicInfo<'a> {
    #![unstable(feature = "panic_internals",
                reason = "internal details of the implementation of the `panic!` \
                          and related macros",
                issue = "0")]
    #[doc(hidden)]
    #[inline]
    pub fn internal_constructor(message: Option<&'a fmt::Arguments<'a>>,
                                location: Location<'a>)
                                -> Self { loop { } }

    #[doc(hidden)]
    #[inline]
    pub fn set_payload(&mut self, info: &'a (dyn Any + Send)) { loop { } }

    #[stable(feature = "panic_hooks", since = "1.10.0")]
    pub fn payload(&self) -> &(dyn Any + Send) { loop { } }

    #[unstable(feature = "panic_info_message", issue = "44489")]
    pub fn message(&self) -> Option<&fmt::Arguments<'_>> { loop { } }

    #[stable(feature = "panic_hooks", since = "1.10.0")]
    pub fn location(&self) -> Option<&Location<'_>> { loop { } }
}

#[stable(feature = "panic_hook_display", since = "1.26.0")]
impl fmt::Display for PanicInfo<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[derive(Debug)]
#[stable(feature = "panic_hooks", since = "1.10.0")]
pub struct Location<'a> {
    file: &'a str,
    line: u32,
    col: u32,
}

impl<'a> Location<'a> {
    #![unstable(feature = "panic_internals",
                reason = "internal details of the implementation of the `panic!` \
                          and related macros",
                issue = "0")]
    #[doc(hidden)]
    pub fn internal_constructor(file: &'a str, line: u32, col: u32) -> Self { loop { } }

    #[stable(feature = "panic_hooks", since = "1.10.0")]
    pub fn file(&self) -> &str { loop { } }

    #[stable(feature = "panic_hooks", since = "1.10.0")]
    pub fn line(&self) -> u32 { loop { } }

    #[stable(feature = "panic_col", since = "1.25.0")]
    pub fn column(&self) -> u32 { loop { } }
}

#[stable(feature = "panic_hook_display", since = "1.26.0")]
impl fmt::Display for Location<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[unstable(feature = "std_internals", issue = "0")]
#[doc(hidden)]
pub unsafe trait BoxMeUp {
    fn box_me_up(&mut self) -> *mut (dyn Any + Send);
    fn get(&mut self) -> &(dyn Any + Send);
}
