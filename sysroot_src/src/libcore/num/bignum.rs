//! Custom arbitrary-precision number (bignum) implementation.

#![doc(hidden)]
#![unstable(feature = "core_private_bignum",
            reason = "internal routines only exposed for testing",
            issue = "0")]
#![macro_use]

pub trait FullOps: Sized {
    fn full_add(self, other: Self, carry: bool) -> (bool /* carry */, Self);

    fn full_mul(self, other: Self, carry: Self) -> (Self /* carry */, Self);

    fn full_mul_add(self, other: Self, other2: Self, carry: Self) -> (Self /* carry */, Self);

    fn full_div_rem(self,
                    other: Self,
                    borrow: Self)
                    -> (Self /* quotient */, Self /* remainder */);
}

macro_rules! impl_full_ops {
    ($($ty:ty: add($addfn:path), mul/div($bigty:ident);)*) => (
        $(
            impl FullOps for $ty {
                fn full_add(self, other: $ty, carry: bool) -> (bool, $ty) { loop { } }

                fn full_mul(self, other: $ty, carry: $ty) -> ($ty, $ty) { loop { } }

                fn full_mul_add(self, other: $ty, other2: $ty, carry: $ty) -> ($ty, $ty) { loop { } }

                fn full_div_rem(self, other: $ty, borrow: $ty) -> ($ty, $ty) { loop { } }
            }
        )*
    )
}

impl_full_ops! {
    u8:  add(intrinsics::u8_add_with_overflow),  mul/div(u16);
    u16: add(intrinsics::u16_add_with_overflow), mul/div(u32);
    u32: add(intrinsics::u32_add_with_overflow), mul/div(u64);
}

const SMALL_POW5: [(u64, usize); 3] = [(125, 3), (15625, 6), (1_220_703_125, 13)];

macro_rules! define_bignum {
    ($name:ident: type=$ty:ty, n=$n:expr) => (
        pub struct $name {
            size: usize,
            base: [$ty; $n]
        }

        impl $name {
            pub fn from_small(v: $ty) -> $name { loop { } }

            pub fn from_u64(mut v: u64) -> $name { loop { } }

            pub fn digits(&self) -> &[$ty] { loop { } }

            pub fn get_bit(&self, i: usize) -> u8 { loop { } }

            pub fn is_zero(&self) -> bool { loop { } }

            pub fn bit_length(&self) -> usize { loop { } }

            pub fn add<'a>(&'a mut self, other: &$name) -> &'a mut $name { loop { } }

            pub fn add_small(&mut self, other: $ty) -> &mut $name { loop { } }

            pub fn sub<'a>(&'a mut self, other: &$name) -> &'a mut $name { loop { } }

            pub fn mul_small(&mut self, other: $ty) -> &mut $name { loop { } }

            pub fn mul_pow2(&mut self, bits: usize) -> &mut $name { loop { } }

            pub fn mul_pow5(&mut self, mut e: usize) -> &mut $name { loop { } }


            pub fn mul_digits<'a>(&'a mut self, other: &[$ty]) -> &'a mut $name { loop { } }

            pub fn div_rem_small(&mut self, other: $ty) -> (&mut $name, $ty) { loop { } }

            pub fn div_rem(&self, d: &$name, q: &mut $name, r: &mut $name) { loop { } }
        }

        impl crate::fmt::Debug for $name {
            fn fmt(&self, f: &mut crate::fmt::Formatter<'_>) -> crate::fmt::Result { loop { } }
        }
    )
}

pub type Digit32 = u32;

define_bignum!(Big32x40: type=Digit32, n=40);

#[doc(hidden)]
pub mod tests {
    define_bignum!(Big8x3: type=u8, n=3);
}
