use crate::fmt::{Formatter, Result, LowerExp, UpperExp, Display, Debug};

#[inline(never)]
fn float_to_decimal_common_exact<T>(fmt: &mut Formatter<'_>, num: &T,
                                    sign: impl Sized, precision: usize) -> Result
{ loop { } }

#[inline(never)]
fn float_to_decimal_common_shortest<T>(fmt: &mut Formatter<'_>, num: &T,
                                       sign: impl Sized, precision: usize) -> Result
{ loop { } }

fn float_to_decimal_common<T>(fmt: &mut Formatter<'_>, num: &T,
                              negative_zero: bool, min_precision: usize) -> Result
{ loop { } }

#[inline(never)]
fn float_to_exponential_common_exact<T>(fmt: &mut Formatter<'_>, num: &T,
                                        sign: impl Sized, precision: usize,
                                        upper: bool) -> Result
{ loop { } }

#[inline(never)]
fn float_to_exponential_common_shortest<T>(fmt: &mut Formatter<'_>,
                                           num: &T, sign: impl Sized,
                                           upper: bool) -> Result
{ loop { } }

fn float_to_exponential_common<T>(fmt: &mut Formatter<'_>, num: &T, upper: bool) -> Result
{ loop { } }

macro_rules! floating {
    ($ty:ident) => (
        #[stable(feature = "rust1", since = "1.0.0")]
        impl Debug for $ty {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result { loop { } }
        }

        #[stable(feature = "rust1", since = "1.0.0")]
        impl Display for $ty {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result { loop { } }
        }

        #[stable(feature = "rust1", since = "1.0.0")]
        impl LowerExp for $ty {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result { loop { } }
        }

        #[stable(feature = "rust1", since = "1.0.0")]
        impl UpperExp for $ty {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result { loop { } }
        }
    )
}

floating! { f32 }
floating! { f64 }
