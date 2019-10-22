use crate::marker::Unsize;

#[unstable(feature = "coerce_unsized", issue = "27732")]
#[lang = "coerce_unsized"]
pub trait CoerceUnsized<T: ?Sized> {
}

#[unstable(feature = "coerce_unsized", issue = "27732")]
impl<'a, T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<&'a mut U> for &'a mut T {}
#[unstable(feature = "coerce_unsized", issue = "27732")]
impl<'a, 'b: 'a, T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<&'a U> for &'b mut T {}
#[unstable(feature = "coerce_unsized", issue = "27732")]
impl<'a, T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<*mut U> for &'a mut T {}
#[unstable(feature = "coerce_unsized", issue = "27732")]
impl<'a, T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<*const U> for &'a mut T {}

#[unstable(feature = "coerce_unsized", issue = "27732")]
impl<'a, 'b: 'a, T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<&'a U> for &'b T {}
#[unstable(feature = "coerce_unsized", issue = "27732")]
impl<'a, T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<*const U> for &'a T {}

#[unstable(feature = "coerce_unsized", issue = "27732")]
impl<T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<*mut U> for *mut T {}
#[unstable(feature = "coerce_unsized", issue = "27732")]
impl<T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<*const U> for *mut T {}

#[unstable(feature = "coerce_unsized", issue = "27732")]
impl<T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<*const U> for *const T {}


#[unstable(feature = "dispatch_from_dyn", issue = "0")]
#[lang = "dispatch_from_dyn"]
pub trait DispatchFromDyn<T> {
}

#[unstable(feature = "dispatch_from_dyn", issue = "0")]
impl<'a, T: ?Sized+Unsize<U>, U: ?Sized> DispatchFromDyn<&'a U> for &'a T {}
#[unstable(feature = "dispatch_from_dyn", issue = "0")]
impl<'a, T: ?Sized+Unsize<U>, U: ?Sized> DispatchFromDyn<&'a mut U> for &'a mut T {}
#[unstable(feature = "dispatch_from_dyn", issue = "0")]
impl<T: ?Sized+Unsize<U>, U: ?Sized> DispatchFromDyn<*const U> for *const T {}
#[unstable(feature = "dispatch_from_dyn", issue = "0")]
impl<T: ?Sized+Unsize<U>, U: ?Sized> DispatchFromDyn<*mut U> for *mut T {}
