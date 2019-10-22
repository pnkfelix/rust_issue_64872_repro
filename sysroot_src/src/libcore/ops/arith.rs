#[lang = "add"]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_on_unimplemented(
    on(
        all(_Self="{integer}", Rhs="{float}"),
        message="cannot add a float to an integer",
    ),
    on(
        all(_Self="{float}", Rhs="{integer}"),
        message="cannot add an integer to a float",
    ),
    message="cannot add `{Rhs}` to `{Self}`",
    label="no implementation for `{Self} + {Rhs}`",
)]
#[doc(alias = "+")]
pub trait Add<Rhs=Self> {
    #[stable(feature = "rust1", since = "1.0.0")]
    type Output;

    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn add(self, rhs: Rhs) -> Self::Output;
}

#[lang = "sub"]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_on_unimplemented(message="cannot subtract `{Rhs}` from `{Self}`",
                         label="no implementation for `{Self} - {Rhs}`")]
#[doc(alias = "-")]
pub trait Sub<Rhs=Self> {
    #[stable(feature = "rust1", since = "1.0.0")]
    type Output;

    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn sub(self, rhs: Rhs) -> Self::Output;
}

#[lang = "mul"]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_on_unimplemented(message="cannot multiply `{Rhs}` to `{Self}`",
                         label="no implementation for `{Self} * {Rhs}`")]
#[doc(alias = "*")]
pub trait Mul<Rhs=Self> {
    #[stable(feature = "rust1", since = "1.0.0")]
    type Output;

    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn mul(self, rhs: Rhs) -> Self::Output;
}

#[lang = "div"]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_on_unimplemented(message="cannot divide `{Self}` by `{Rhs}`",
                         label="no implementation for `{Self} / {Rhs}`")]
#[doc(alias = "/")]
pub trait Div<Rhs=Self> {
    #[stable(feature = "rust1", since = "1.0.0")]
    type Output;

    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn div(self, rhs: Rhs) -> Self::Output;
}

#[lang = "rem"]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_on_unimplemented(message="cannot mod `{Self}` by `{Rhs}`",
                         label="no implementation for `{Self} % {Rhs}`")]
#[doc(alias = "%")]
pub trait Rem<Rhs=Self> {
    #[stable(feature = "rust1", since = "1.0.0")]
    type Output;

    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn rem(self, rhs: Rhs) -> Self::Output;
}

#[lang = "neg"]
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(alias = "-")]
pub trait Neg {
    #[stable(feature = "rust1", since = "1.0.0")]
    type Output;

    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn neg(self) -> Self::Output;
}

#[lang = "add_assign"]
#[stable(feature = "op_assign_traits", since = "1.8.0")]
#[rustc_on_unimplemented(message="cannot add-assign `{Rhs}` to `{Self}`",
                         label="no implementation for `{Self} += {Rhs}`")]
#[doc(alias = "+")]
#[doc(alias = "+=")]
pub trait AddAssign<Rhs=Self> {
    #[stable(feature = "op_assign_traits", since = "1.8.0")]
    fn add_assign(&mut self, rhs: Rhs);
}

#[lang = "sub_assign"]
#[stable(feature = "op_assign_traits", since = "1.8.0")]
#[rustc_on_unimplemented(message="cannot subtract-assign `{Rhs}` from `{Self}`",
                         label="no implementation for `{Self} -= {Rhs}`")]
#[doc(alias = "-")]
#[doc(alias = "-=")]
pub trait SubAssign<Rhs=Self> {
    #[stable(feature = "op_assign_traits", since = "1.8.0")]
    fn sub_assign(&mut self, rhs: Rhs);
}

#[lang = "mul_assign"]
#[stable(feature = "op_assign_traits", since = "1.8.0")]
#[rustc_on_unimplemented(message="cannot multiply-assign `{Rhs}` to `{Self}`",
                         label="no implementation for `{Self} *= {Rhs}`")]
#[doc(alias = "*")]
#[doc(alias = "*=")]
pub trait MulAssign<Rhs=Self> {
    #[stable(feature = "op_assign_traits", since = "1.8.0")]
    fn mul_assign(&mut self, rhs: Rhs);
}

#[lang = "div_assign"]
#[stable(feature = "op_assign_traits", since = "1.8.0")]
#[rustc_on_unimplemented(message="cannot divide-assign `{Self}` by `{Rhs}`",
                         label="no implementation for `{Self} /= {Rhs}`")]
#[doc(alias = "/")]
#[doc(alias = "/=")]
pub trait DivAssign<Rhs=Self> {
    #[stable(feature = "op_assign_traits", since = "1.8.0")]
    fn div_assign(&mut self, rhs: Rhs);
}

#[lang = "rem_assign"]
#[stable(feature = "op_assign_traits", since = "1.8.0")]
#[rustc_on_unimplemented(message="cannot mod-assign `{Self}` by `{Rhs}``",
                         label="no implementation for `{Self} %= {Rhs}`")]
#[doc(alias = "%")]
#[doc(alias = "%=")]
pub trait RemAssign<Rhs=Self> {
    #[stable(feature = "op_assign_traits", since = "1.8.0")]
    fn rem_assign(&mut self, rhs: Rhs);
}
