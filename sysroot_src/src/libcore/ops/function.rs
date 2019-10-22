#[lang = "fn"]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_paren_sugar]
#[rustc_on_unimplemented(
    on(Args="()", note="wrap the `{Self}` in a closure with no arguments: `|| {{ /* code */ }}"),
    message="expected a `{Fn}<{Args}>` closure, found `{Self}`",
    label="expected an `Fn<{Args}>` closure, found `{Self}`",
)]
#[fundamental] // so that regex can rely that `&str: !FnMut`
#[must_use = "closures are lazy and do nothing unless called"]
pub trait Fn<Args> : FnMut<Args> {
    #[unstable(feature = "fn_traits", issue = "29625")]
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}

#[lang = "fn_mut"]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_paren_sugar]
#[rustc_on_unimplemented(
    on(Args="()", note="wrap the `{Self}` in a closure with no arguments: `|| {{ /* code */ }}"),
    on(
        all(Args="(char,)", _Self="std::string::String"),
        note="borrowing the `{Self}` might fix the problem"
    ),
    message="expected a `{FnMut}<{Args}>` closure, found `{Self}`",
    label="expected an `FnMut<{Args}>` closure, found `{Self}`",
)]
#[fundamental] // so that regex can rely that `&str: !FnMut`
#[must_use = "closures are lazy and do nothing unless called"]
pub trait FnMut<Args> : FnOnce<Args> {
    #[unstable(feature = "fn_traits", issue = "29625")]
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}

#[lang = "fn_once"]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_paren_sugar]
#[rustc_on_unimplemented(
    on(Args="()", note="wrap the `{Self}` in a closure with no arguments: `|| {{ /* code */ }}"),
    message="expected a `{FnOnce}<{Args}>` closure, found `{Self}`",
    label="expected an `FnOnce<{Args}>` closure, found `{Self}`",
)]
#[fundamental] // so that regex can rely that `&str: !FnMut`
#[must_use = "closures are lazy and do nothing unless called"]
pub trait FnOnce<Args> {
    #[stable(feature = "fn_once_output", since = "1.12.0")]
    type Output;

    #[unstable(feature = "fn_traits", issue = "29625")]
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}

mod impls {
    use super::{Fn,FnMut,FnOnce};

    #[stable(feature = "rust1", since = "1.0.0")]
    impl<A,F:?Sized> Fn<A> for &F
        where F : Fn<A>
    {
        extern "rust-call" fn call(&self, args: A) -> F::Output {
            (**self).call(args)
        }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl<A,F:?Sized> FnMut<A> for &F
        where F : Fn<A>
    {
        extern "rust-call" fn call_mut(&mut self, args: A) -> F::Output {
            (**self).call(args)
        }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl<A,F:?Sized> FnOnce<A> for &F
        where F : Fn<A>
    {
        type Output = F::Output;

        extern "rust-call" fn call_once(self, args: A) -> F::Output {
            (*self).call(args)
        }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl<A,F:?Sized> FnMut<A> for &mut F
        where F : FnMut<A>
    {
        extern "rust-call" fn call_mut(&mut self, args: A) -> F::Output {
            (*self).call_mut(args)
        }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl<A,F:?Sized> FnOnce<A> for &mut F
        where F : FnMut<A>
    {
        type Output = F::Output;
        extern "rust-call" fn call_once(self, args: A) -> F::Output {
            (*self).call_mut(args)
        }
    }
}
