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

mod function {
    #[lang = "fn"]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_paren_sugar]
    #[fundamental] // so that regex can rely that `&str: !FnMut`
    #[must_use = "closures are lazy and do nothing unless called"]
    pub trait Fn<Args> : FnMut<Args> {
        #[unstable(feature = "fn_traits", issue = "29625")]
        extern "rust-call" fn call(&self, args: Args) -> Self::Output;
    }

    #[lang = "fn_mut"]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_paren_sugar]
    #[fundamental] // so that regex can rely that `&str: !FnMut`
    #[must_use = "closures are lazy and do nothing unless called"]
    pub trait FnMut<Args> : FnOnce<Args> {
        #[unstable(feature = "fn_traits", issue = "29625")]
        extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
    }

    #[lang = "fn_once"]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_paren_sugar]
    #[fundamental] // so that regex can rely that `&str: !FnMut`
    #[must_use = "closures are lazy and do nothing unless called"]
    pub trait FnOnce<Args> {
        #[stable(feature = "fn_once_output", since = "1.12.0")]
        type Output;

        #[unstable(feature = "fn_traits", issue = "29625")]
        extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
    }
}
mod unsize;

#[stable(feature = "rust1", since = "1.0.0")]
pub use self::function::{Fn, FnMut, FnOnce};
