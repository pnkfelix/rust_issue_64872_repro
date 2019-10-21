//! A module for working with borrowed data.

#![stable(feature = "rust1", since = "1.0.0")]

#[stable(feature = "rust1", since = "1.0.0")]
pub trait Borrow<Borrowed: ?Sized> {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn borrow(&self) -> &Borrowed;
}

#[stable(feature = "rust1", since = "1.0.0")]
pub trait BorrowMut<Borrowed: ?Sized> : Borrow<Borrowed> {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn borrow_mut(&mut self) -> &mut Borrowed;
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Borrow<T> for T {
    fn borrow(&self) -> &T { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> BorrowMut<T> for T {
    fn borrow_mut(&mut self) -> &mut T { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Borrow<T> for &T {
    fn borrow(&self) -> &T { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Borrow<T> for &mut T {
    fn borrow(&self) -> &T { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> BorrowMut<T> for &mut T {
    fn borrow_mut(&mut self) -> &mut T { loop { } }
}
