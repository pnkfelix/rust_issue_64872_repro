/// Conversion from an `Iterator`.
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_on_unimplemented(
    message="a collection of type `{Self}` cannot be built from an iterator \
             over elements of type `{A}`",
    label="a collection of type `{Self}` cannot be built from `std::iter::Iterator<Item={A}>`",
)]
pub trait FromIterator<A>: Sized {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn from_iter<T: IntoIterator<Item=A>>(iter: T) -> Self;
}

#[stable(feature = "rust1", since = "1.0.0")]
pub trait IntoIterator {
    #[stable(feature = "rust1", since = "1.0.0")]
    type Item;

    #[stable(feature = "rust1", since = "1.0.0")]
    type IntoIter: Iterator<Item=Self::Item>;

    #[stable(feature = "rust1", since = "1.0.0")]
    fn into_iter(self) -> Self::IntoIter;
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I: Iterator> IntoIterator for I {
    type Item = I::Item;
    type IntoIter = I;

    fn into_iter(self) -> I { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
pub trait Extend<A> {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn extend<T: IntoIterator<Item=A>>(&mut self, iter: T);
}

#[stable(feature = "extend_for_unit", since = "1.28.0")]
impl Extend<()> for () {
    fn extend<T: IntoIterator<Item = ()>>(&mut self, iter: T) { loop { } }
}
