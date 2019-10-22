//! Overloadable operators.

#![stable(feature = "rust1", since = "1.0.0")]

mod deref {
    #[lang = "receiver"]
    #[unstable(feature = "receiver_trait", issue = "0")]
    #[doc(hidden)]
    pub trait Receiver { }

    #[unstable(feature = "receiver_trait", issue = "0")]
    impl<T: ?Sized> Receiver for &T {}

    #[unstable(feature = "receiver_trait", issue = "0")]
    impl<T: ?Sized> Receiver for &mut T {}
}

mod function;
mod unsize;

#[stable(feature = "rust1", since = "1.0.0")]
pub use self::function::{Fn, FnMut, FnOnce};
