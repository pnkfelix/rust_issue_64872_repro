use crate::num::Wrapping;

#[stable(feature = "iter_arith_traits", since = "1.12.0")]
pub trait Sum<A = Self>: Sized {
    #[stable(feature = "iter_arith_traits", since = "1.12.0")]
    fn sum<I: Iterator<Item=A>>(iter: I) -> Self;
}

#[stable(feature = "iter_arith_traits", since = "1.12.0")]
pub trait Product<A = Self>: Sized {
    #[stable(feature = "iter_arith_traits", since = "1.12.0")]
    fn product<I: Iterator<Item=A>>(iter: I) -> Self;
}

macro_rules! integer_sum_product {
    (@impls $zero:expr, $one:expr, #[$attr:meta], $($a:ty)*) => ($(
        #[$attr]
        impl Sum for $a {
            fn sum<I: Iterator<Item=$a>>(iter: I) -> $a { loop { } }
        }

        #[$attr]
        impl Product for $a {
            fn product<I: Iterator<Item=$a>>(iter: I) -> $a { loop { } }
        }

        #[$attr]
        impl<'a> Sum<&'a $a> for $a {
            fn sum<I: Iterator<Item=&'a $a>>(iter: I) -> $a { loop { } }
        }

        #[$attr]
        impl<'a> Product<&'a $a> for $a {
            fn product<I: Iterator<Item=&'a $a>>(iter: I) -> $a { loop { } }
        }
    )*);
    ($($a:ty)*) => (
        integer_sum_product!(@impls 0, 1,
                #[stable(feature = "iter_arith_traits", since = "1.12.0")],
                $($a)*);
        integer_sum_product!(@impls Wrapping(0), Wrapping(1),
                #[stable(feature = "wrapping_iter_arith", since = "1.14.0")],
                $(Wrapping<$a>)*);
    );
}

macro_rules! float_sum_product {
    ($($a:ident)*) => ($(
        #[stable(feature = "iter_arith_traits", since = "1.12.0")]
        impl Sum for $a {
            fn sum<I: Iterator<Item=$a>>(iter: I) -> $a { loop { } }
        }

        #[stable(feature = "iter_arith_traits", since = "1.12.0")]
        impl Product for $a {
            fn product<I: Iterator<Item=$a>>(iter: I) -> $a { loop { } }
        }

        #[stable(feature = "iter_arith_traits", since = "1.12.0")]
        impl<'a> Sum<&'a $a> for $a {
            fn sum<I: Iterator<Item=&'a $a>>(iter: I) -> $a { loop { } }
        }

        #[stable(feature = "iter_arith_traits", since = "1.12.0")]
        impl<'a> Product<&'a $a> for $a {
            fn product<I: Iterator<Item=&'a $a>>(iter: I) -> $a { loop { } }
        }
    )*)
}

integer_sum_product! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
float_sum_product! { f32 f64 }

#[stable(feature = "iter_arith_traits_result", since="1.16.0")]
impl<T, U, E> Sum<Result<U, E>> for Result<T, E>
    where T: Sum<U>,
{
    fn sum<I>(iter: I) -> Result<T, E>
        where I: Iterator<Item = Result<U, E>>,
    { loop { } }
}

#[stable(feature = "iter_arith_traits_result", since="1.16.0")]
impl<T, U, E> Product<Result<U, E>> for Result<T, E>
    where T: Product<U>,
{
    fn product<I>(iter: I) -> Result<T, E>
        where I: Iterator<Item = Result<U, E>>,
    { loop { } }
}

#[stable(feature = "iter_arith_traits_option", since = "1.37.0")]
impl<T, U> Sum<Option<U>> for Option<T>
where
    T: Sum<U>,
{
    fn sum<I>(iter: I) -> Option<T>
    where
        I: Iterator<Item = Option<U>>,
    { loop { } }
}

#[stable(feature = "iter_arith_traits_option", since = "1.37.0")]
impl<T, U> Product<Option<U>> for Option<T>
where
    T: Product<U>,
{
    fn product<I>(iter: I) -> Option<T>
    where
        I: Iterator<Item = Option<U>>,
    { loop { } }
}
