#[lang = "not"]
#[stable(feature = "rust1", since = "1.0.0")]
pub trait Not {
    #[stable(feature = "rust1", since = "1.0.0")]
    type Output;

    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn not(self) -> Self::Output;
}

#[lang = "bitand"]
#[doc(alias = "&")]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_on_unimplemented(message="no implementation for `{Self} & {Rhs}`",
                         label="no implementation for `{Self} & {Rhs}`")]
pub trait BitAnd<Rhs=Self> {
    #[stable(feature = "rust1", since = "1.0.0")]
    type Output;

    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn bitand(self, rhs: Rhs) -> Self::Output;
}

#[lang = "bitor"]
#[doc(alias = "|")]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_on_unimplemented(message="no implementation for `{Self} | {Rhs}`",
                         label="no implementation for `{Self} | {Rhs}`")]
pub trait BitOr<Rhs=Self> {
    #[stable(feature = "rust1", since = "1.0.0")]
    type Output;

    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn bitor(self, rhs: Rhs) -> Self::Output;
}

#[lang = "bitxor"]
#[doc(alias = "^")]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_on_unimplemented(message="no implementation for `{Self} ^ {Rhs}`",
                         label="no implementation for `{Self} ^ {Rhs}`")]
pub trait BitXor<Rhs=Self> {
    #[stable(feature = "rust1", since = "1.0.0")]
    type Output;

    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn bitxor(self, rhs: Rhs) -> Self::Output;
}

#[lang = "shl"]
#[doc(alias = "<<")]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_on_unimplemented(message="no implementation for `{Self} << {Rhs}`",
                         label="no implementation for `{Self} << {Rhs}`")]
pub trait Shl<Rhs=Self> {
    #[stable(feature = "rust1", since = "1.0.0")]
    type Output;

    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn shl(self, rhs: Rhs) -> Self::Output;
}

#[lang = "shr"]
#[doc(alias = ">>")]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_on_unimplemented(message="no implementation for `{Self} >> {Rhs}`",
                         label="no implementation for `{Self} >> {Rhs}`")]
pub trait Shr<Rhs=Self> {
    #[stable(feature = "rust1", since = "1.0.0")]
    type Output;

    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn shr(self, rhs: Rhs) -> Self::Output;
}

#[lang = "bitand_assign"]
#[doc(alias = "&=")]
#[stable(feature = "op_assign_traits", since = "1.8.0")]
#[rustc_on_unimplemented(message="no implementation for `{Self} &= {Rhs}`",
                         label="no implementation for `{Self} &= {Rhs}`")]
pub trait BitAndAssign<Rhs=Self> {
    #[stable(feature = "op_assign_traits", since = "1.8.0")]
    fn bitand_assign(&mut self, rhs: Rhs);
}

#[lang = "bitor_assign"]
#[doc(alias = "|=")]
#[stable(feature = "op_assign_traits", since = "1.8.0")]
#[rustc_on_unimplemented(message="no implementation for `{Self} |= {Rhs}`",
                         label="no implementation for `{Self} |= {Rhs}`")]
pub trait BitOrAssign<Rhs=Self> {
    #[stable(feature = "op_assign_traits", since = "1.8.0")]
    fn bitor_assign(&mut self, rhs: Rhs);
}

#[lang = "bitxor_assign"]
#[doc(alias = "^=")]
#[stable(feature = "op_assign_traits", since = "1.8.0")]
#[rustc_on_unimplemented(message="no implementation for `{Self} ^= {Rhs}`",
                         label="no implementation for `{Self} ^= {Rhs}`")]
pub trait BitXorAssign<Rhs=Self> {
    #[stable(feature = "op_assign_traits", since = "1.8.0")]
    fn bitxor_assign(&mut self, rhs: Rhs);
}

#[lang = "shl_assign"]
#[doc(alias = "<<=")]
#[stable(feature = "op_assign_traits", since = "1.8.0")]
#[rustc_on_unimplemented(message="no implementation for `{Self} <<= {Rhs}`",
                         label="no implementation for `{Self} <<= {Rhs}`")]
pub trait ShlAssign<Rhs=Self> {
    #[stable(feature = "op_assign_traits", since = "1.8.0")]
    fn shl_assign(&mut self, rhs: Rhs);
}

#[lang = "shr_assign"]
#[doc(alias = ">>=")]
#[stable(feature = "op_assign_traits", since = "1.8.0")]
#[rustc_on_unimplemented(message="no implementation for `{Self} >>= {Rhs}`",
                         label="no implementation for `{Self} >>= {Rhs}`")]
pub trait ShrAssign<Rhs=Self> {
    #[stable(feature = "op_assign_traits", since = "1.8.0")]
    fn shr_assign(&mut self, rhs: Rhs);
}

