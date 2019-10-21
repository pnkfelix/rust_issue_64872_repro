// See src/libstd/primitive_docs.rs for documentation.

use crate::cmp::*;

macro_rules! tuple_impls {
    ($(
        $Tuple:ident {
            $(($idx:tt) -> $T:ident)+
        }
    )+) => {
        $(
            #[stable(feature = "rust1", since = "1.0.0")]
            impl<$($T:PartialEq),+> PartialEq for ($($T,)+) where last_type!($($T,)+): ?Sized {
                #[inline]
                fn eq(&self, other: &($($T,)+)) -> bool { loop { } }
                #[inline]
                fn ne(&self, other: &($($T,)+)) -> bool { loop { } }
            }

            #[stable(feature = "rust1", since = "1.0.0")]
            impl<$($T:Eq),+> Eq for ($($T,)+) where last_type!($($T,)+): ?Sized {}

            #[stable(feature = "rust1", since = "1.0.0")]
            impl<$($T:PartialOrd + PartialEq),+> PartialOrd for ($($T,)+)
                    where last_type!($($T,)+): ?Sized {
                #[inline]
                fn partial_cmp(&self, other: &($($T,)+)) -> Option<Ordering> { loop { } }
                #[inline]
                fn lt(&self, other: &($($T,)+)) -> bool { loop { } }
                #[inline]
                fn le(&self, other: &($($T,)+)) -> bool { loop { } }
                #[inline]
                fn ge(&self, other: &($($T,)+)) -> bool { loop { } }
                #[inline]
                fn gt(&self, other: &($($T,)+)) -> bool { loop { } }
            }

            #[stable(feature = "rust1", since = "1.0.0")]
            impl<$($T:Ord),+> Ord for ($($T,)+) where last_type!($($T,)+): ?Sized {
                #[inline]
                fn cmp(&self, other: &($($T,)+)) -> Ordering { loop { } }
            }

            #[stable(feature = "rust1", since = "1.0.0")]
            impl<$($T:Default),+> Default for ($($T,)+) {
                #[inline]
                fn default() -> ($($T,)+) { loop { } }
            }
        )+
    }
}

macro_rules! last_type {
    ($a:ident,) => { $a };
    ($a:ident, $($rest_a:ident,)+) => { last_type!($($rest_a,)+) };
}

tuple_impls! {
    Tuple1 {
        (0) -> A
    }
    Tuple2 {
        (0) -> A
        (1) -> B
    }
    Tuple3 {
        (0) -> A
        (1) -> B
        (2) -> C
    }
    Tuple4 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
    }
    Tuple5 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
    }
    Tuple6 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
    }
    Tuple7 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
    }
    Tuple8 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
    }
    Tuple9 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
    }
    Tuple10 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
    }
    Tuple11 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
    }
    Tuple12 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
        (11) -> L
    }
}
