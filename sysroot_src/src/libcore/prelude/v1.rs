//! The core prelude
//!
//! This module is intended for users of libcore which do not link to libstd as
//! well. This module is imported by default when `#![no_std]` is used in the
//! same manner as the standard library's prelude.

#![stable(feature = "core_prelude", since = "1.4.0")]


// Re-exported core operators
#[stable(feature = "core_prelude", since = "1.4.0")]
#[doc(no_inline)]
pub use crate::marker::{Copy, Send, Sized, Sync, /*Unpin*/};
// Re-exported types and traits
#[stable(feature = "core_prelude", since = "1.4.0")]
#[doc(no_inline)]
pub use crate::clone::Clone;
#[stable(feature = "core_prelude", since = "1.4.0")]
#[doc(no_inline)]
pub use crate::cmp::{PartialEq, PartialOrd, Eq, Ord};
#[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
#[doc(no_inline)]
pub use crate::hash::macros::Hash;
#[stable(feature = "core_prelude", since = "1.4.0")]
#[doc(no_inline)]
pub use crate::default::Default;
#[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
#[doc(no_inline)]
pub use crate::fmt::macros::Debug;
#[stable(feature = "core_prelude", since = "1.4.0")]
#[doc(no_inline)]
pub use crate::iter::{Iterator, /*
oExtend, */IntoIterator};
#[stable(feature = "core_prelude", since = "1.4.0")]
#[doc(no_inline)]
pub use crate::option::Option::{self, Some, None};
#[stable(feature = "core_prelude", since = "1.4.0")]
#[doc(no_inline)]
pub use crate::result::Result::{self, Ok, Err};
#[stable(feature = "core_prelude", since = "1.4.0")]
#[doc(no_inline)]
pub use crate::convert::{AsRef, AsMut, Into, From};
#[stable(feature = "core_prelude", since = "1.4.0")]
#[doc(no_inline)]
pub use crate::ops::{Drop, Fn, FnMut, FnOnce};
#[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
#[doc(no_inline)]
pub use crate::{
    assert,
    cfg,
    concat,
    format_args,
    stringify,
};
