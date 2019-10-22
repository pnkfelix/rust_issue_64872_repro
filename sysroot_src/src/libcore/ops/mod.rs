//! Overloadable operators.

#![stable(feature = "rust1", since = "1.0.0")]

mod deref;
mod function;
mod unsize;

#[stable(feature = "rust1", since = "1.0.0")]
pub use self::deref::{Deref, DerefMut};

#[unstable(feature = "receiver_trait", issue = "0")]
pub use self::deref::Receiver;

#[stable(feature = "rust1", since = "1.0.0")]
pub use self::function::{Fn, FnMut, FnOnce};
