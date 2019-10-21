#![stable(feature = "rust1", since = "1.0.0")]

use crate::fmt;

#[stable(feature = "convert_id", since = "1.33.0")]
#[inline]
pub const fn identity<T>(x: T) -> T { x }

#[stable(feature = "rust1", since = "1.0.0")]
pub trait AsRef<T: ?Sized> {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn as_ref(&self) -> &T;
}

#[stable(feature = "rust1", since = "1.0.0")]
pub trait AsMut<T: ?Sized> {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn as_mut(&mut self) -> &mut T;
}

#[stable(feature = "rust1", since = "1.0.0")]
pub trait Into<T>: Sized {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn into(self) -> T;
}

#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_on_unimplemented(
    on(
        all(_Self="&str", T="std::string::String"),
        note="to coerce a `{T}` into a `{Self}`, use `&*` as a prefix",
    )
)]
pub trait From<T>: Sized {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn from(_: T) -> Self;
}

#[stable(feature = "try_from", since = "1.34.0")]
pub trait TryInto<T>: Sized {
    #[stable(feature = "try_from", since = "1.34.0")]
    type Error;

    #[stable(feature = "try_from", since = "1.34.0")]
    fn try_into(self) -> Result<T, Self::Error>;
}

#[stable(feature = "try_from", since = "1.34.0")]
pub trait TryFrom<T>: Sized {
    #[stable(feature = "try_from", since = "1.34.0")]
    type Error;

    #[stable(feature = "try_from", since = "1.34.0")]
    fn try_from(value: T) -> Result<Self, Self::Error>;
}


#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized, U: ?Sized> AsRef<U> for &T where T: AsRef<U>
{
    fn as_ref(&self) -> &U { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized, U: ?Sized> AsRef<U> for &mut T where T: AsRef<U>
{
    fn as_ref(&self) -> &U { loop { } }
}


#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized, U: ?Sized> AsMut<U> for &mut T where T: AsMut<U>
{
    fn as_mut(&mut self) -> &mut U { loop { } }
}


#[stable(feature = "rust1", since = "1.0.0")]
impl<T, U> Into<U> for T where U: From<T>
{
    fn into(self) -> U { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> From<T> for T {
    fn from(_t: T) -> T { loop { } }
}

#[stable(feature = "try_from", since = "1.34.0")]
impl<T, U> TryInto<U> for T where U: TryFrom<T>
{
    type Error = U::Error;

    fn try_into(self) -> Result<U, U::Error> { loop { } }
}

#[stable(feature = "try_from", since = "1.34.0")]
impl<T, U> TryFrom<U> for T where U: Into<T> {
    type Error = Infallible;

    fn try_from(_value: U) -> Result<Self, Self::Error> { loop { } }
}


#[stable(feature = "rust1", since = "1.0.0")]
impl<T> AsRef<[T]> for [T] {
    fn as_ref(&self) -> &[T] { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> AsMut<[T]> for [T] {
    fn as_mut(&mut self) -> &mut [T] { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl AsRef<str> for str {
    #[inline]
    fn as_ref(&self) -> &str { loop { } }
}


#[stable(feature = "convert_infallible", since = "1.34.0")]
#[derive(Copy)]
pub enum Infallible {}

#[stable(feature = "convert_infallible", since = "1.34.0")]
impl Clone for Infallible {
    fn clone(&self) -> Infallible { loop { } }
}

#[stable(feature = "convert_infallible", since = "1.34.0")]
impl fmt::Debug for Infallible {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "convert_infallible", since = "1.34.0")]
impl fmt::Display for Infallible {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "convert_infallible", since = "1.34.0")]
impl PartialEq for Infallible {
    fn eq(&self, _: &Infallible) -> bool { loop { } }
}

#[stable(feature = "convert_infallible", since = "1.34.0")]
impl Eq for Infallible {}

#[stable(feature = "convert_infallible", since = "1.34.0")]
impl PartialOrd for Infallible {
    fn partial_cmp(&self, _other: &Self) -> Option<crate::cmp::Ordering> { loop { } }
}

#[stable(feature = "convert_infallible", since = "1.34.0")]
impl Ord for Infallible {
    fn cmp(&self, _other: &Self) -> crate::cmp::Ordering { loop { } }
}

#[stable(feature = "convert_infallible", since = "1.34.0")]
impl From<!> for Infallible {
    fn from(_x: !) -> Self { loop { } }
}
