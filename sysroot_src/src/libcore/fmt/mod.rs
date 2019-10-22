
#![stable(feature = "rust1", since = "1.0.0")]

use crate::marker::PhantomData;
// use crate::num::flt2dec;
use crate::result;
use crate::slice;
use crate::str;

mod float;
mod num;
mod builders;

#[stable(feature = "fmt_flags_align", since = "1.28.0")]
#[derive(Debug)]
pub enum Alignment {
    #[stable(feature = "fmt_flags_align", since = "1.28.0")]
    Left,
    #[stable(feature = "fmt_flags_align", since = "1.28.0")]
    Right,
    #[stable(feature = "fmt_flags_align", since = "1.28.0")]
    Center,
}

#[stable(feature = "debug_builders", since = "1.2.0")]
pub use self::builders::{DebugStruct, DebugTuple, DebugSet, DebugList, DebugMap};

#[unstable(feature = "fmt_internals", reason = "internal to format_args!",
           issue = "0")]
#[doc(hidden)]
pub mod rt {
    pub mod v1;
}

#[stable(feature = "rust1", since = "1.0.0")]
pub type Result = result::Result<(), Error>;

#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Error;

#[stable(feature = "rust1", since = "1.0.0")]
pub trait Write {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn write_str(&mut self, s: &str) -> Result;

    #[stable(feature = "fmt_write_char", since = "1.1.0")]
    fn write_char(&mut self, c: char) -> Result { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    fn write_fmt(mut self: &mut Self, args: Arguments<'_>) -> Result { loop { } }
}

#[stable(feature = "fmt_write_blanket_impl", since = "1.4.0")]
impl<W: Write + ?Sized> Write for &mut W {
    fn write_str(&mut self, s: &str) -> Result { loop { } }

    fn write_char(&mut self, c: char) -> Result { loop { } }

    fn write_fmt(&mut self, args: Arguments<'_>) -> Result { loop { } }
}

#[allow(missing_debug_implementations)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Formatter<'a> {
    flags: u32,
    fill: char,
    align: rt::v1::Alignment,
    width: Option<usize>,
    precision: Option<usize>,

    buf: &'a mut (dyn Write+'a),
    curarg: slice::Iter<'a, ArgumentV1<'a>>,
    args: &'a [ArgumentV1<'a>],
}


struct Void {
    _priv: (),
    _oibit_remover: PhantomData<*mut dyn Fn()>,
}

#[derive(Copy, Clone)]
#[allow(missing_debug_implementations)]
#[unstable(feature = "fmt_internals", reason = "internal to format_args!",
           issue = "0")]
#[doc(hidden)]
pub struct ArgumentV1<'a> {
    value: &'a Void,
    formatter: fn(&Void, &mut Formatter<'_>) -> Result,
}

impl<'a> ArgumentV1<'a> {
    #[inline(never)]
    fn show_usize(x: &usize, f: &mut Formatter<'_>) -> Result { loop { } }

    #[doc(hidden)]
    #[unstable(feature = "fmt_internals", reason = "internal to format_args!",
               issue = "0")]
    pub fn new<'b, T>(x: &'b T,
                      f: fn(&T, &mut Formatter<'_>) -> Result) -> ArgumentV1<'b> { loop { } }

    #[doc(hidden)]
    #[unstable(feature = "fmt_internals", reason = "internal to format_args!",
               issue = "0")]
    pub fn from_usize(x: &usize) -> ArgumentV1<'_> { loop { } }

    fn as_usize(&self) -> Option<usize> { loop { } }
}

#[derive(Copy, Clone)]
enum FlagV1 { SignPlus, SignMinus, Alternate, SignAwareZeroPad, DebugLowerHex, DebugUpperHex }

impl<'a> Arguments<'a> {
    #[doc(hidden)] #[inline]
    #[unstable(feature = "fmt_internals", reason = "internal to format_args!",
               issue = "0")]
    pub fn new_v1(pieces: &'a [&'a str],
                  args: &'a [ArgumentV1<'a>]) -> Arguments<'a> { loop { } }

    #[doc(hidden)] #[inline]
    #[unstable(feature = "fmt_internals", reason = "internal to format_args!",
               issue = "0")]
    pub fn new_v1_formatted(pieces: &'a [&'a str],
                            args: &'a [ArgumentV1<'a>],
                            fmt: &'a [rt::v1::Argument]) -> Arguments<'a> { loop { } }

    #[doc(hidden)] #[inline]
    #[unstable(feature = "fmt_internals", reason = "internal to format_args!",
               issue = "0")]
    pub fn estimated_capacity(&self) -> usize { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Copy, Clone)]
pub struct Arguments<'a> {
    pieces: &'a [&'a str],

    fmt: Option<&'a [rt::v1::Argument]>,

    args: &'a [ArgumentV1<'a>],
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Debug for Arguments<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Display for Arguments<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_on_unimplemented(
    on(crate_local, label="`{Self}` cannot be formatted using `{{:?}}`",
                    note="add `#[derive(Debug)]` or manually implement `{Debug}`"),
    message="`{Self}` doesn't implement `{Debug}`",
    label="`{Self}` cannot be formatted using `{{:?}}` because it doesn't implement `{Debug}`",
)]
#[doc(alias = "{:?}")]
#[rustc_diagnostic_item = "debug_trait"]
pub trait Debug {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

pub(crate) mod macros {
    #[rustc_builtin_macro]
    #[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
    #[allow_internal_unstable(core_intrinsics)]
    pub macro Debug($item:item) { /* compiler built-in */ }
}
#[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
#[doc(inline)]
pub use macros::Debug;

#[rustc_on_unimplemented(
    on(
        _Self="std::path::Path",
        label="`{Self}` cannot be formatted with the default formatter; call `.display()` on it",
        note="call `.display()` or `.to_string_lossy()` to safely print paths, \
              as they may contain non-Unicode data"
    ),
    message="`{Self}` doesn't implement `{Display}`",
    label="`{Self}` cannot be formatted with the default formatter",
    note="in format strings you may be able to use `{{:?}}` (or {{:#?}} for pretty-print) instead",
)]
#[doc(alias = "{}")]
#[stable(feature = "rust1", since = "1.0.0")]
pub trait Display {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

#[stable(feature = "rust1", since = "1.0.0")]
pub trait Octal {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

#[stable(feature = "rust1", since = "1.0.0")]
pub trait Binary {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

#[stable(feature = "rust1", since = "1.0.0")]
pub trait LowerHex {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

#[stable(feature = "rust1", since = "1.0.0")]
pub trait UpperHex {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

#[stable(feature = "rust1", since = "1.0.0")]
pub trait Pointer {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

#[stable(feature = "rust1", since = "1.0.0")]
pub trait LowerExp {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

#[stable(feature = "rust1", since = "1.0.0")]
pub trait UpperExp {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

#[stable(feature = "rust1", since = "1.0.0")]
pub fn write(output: &mut dyn Write, args: Arguments<'_>) -> Result { loop { } }

#[must_use = "don't forget to write the post padding"]
struct PostPadding {
    fill: char,
    padding: usize,
}

impl PostPadding {
    fn new(fill: char, padding: usize) -> PostPadding { loop { } }

    fn write(self, buf: &mut dyn Write) -> Result { loop { } }
}

impl<'a> Formatter<'a> {
    fn wrap_buf<'b, 'c, F>(&'b mut self, wrap: F) -> Formatter<'c>
        where 'b: 'c, F: FnOnce(&'b mut (dyn Write+'b)) -> &'c mut (dyn Write+'c)
    { loop { } }

    fn run(&mut self, arg: &rt::v1::Argument) -> Result { loop { } }

    fn getcount(&mut self, cnt: &rt::v1::Count) -> Option<usize> { loop { } }


    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn pad_integral(&mut self,
                        is_nonnegative: bool,
                        prefix: &str,
                        buf: &str)
                        -> Result { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn pad(&mut self, s: &str) -> Result { loop { } }

    fn padding(
        &mut self,
        padding: usize,
        default: rt::v1::Alignment
    ) -> result::Result<PostPadding, Error> { loop { } }

    fn pad_formatted_parts(&mut self, formatted: &impl Sized) -> Result { loop { } }

    fn write_formatted_parts(&mut self, formatted: &impl Sized) -> Result { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn write_str(&mut self, data: &str) -> Result { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_deprecated(since = "1.24.0",
                       reason = "use the `sign_plus`, `sign_minus`, `alternate`, \
                                 or `sign_aware_zero_pad` methods instead")]
    pub fn flags(&self) -> u32 { loop { } }

    #[stable(feature = "fmt_flags", since = "1.5.0")]
    pub fn fill(&self) -> char { loop { } }

    #[stable(feature = "fmt_flags_align", since = "1.28.0")]
    pub fn align(&self) -> Option<Alignment> { loop { } }

    #[stable(feature = "fmt_flags", since = "1.5.0")]
    pub fn width(&self) -> Option<usize> { loop { } }

    #[stable(feature = "fmt_flags", since = "1.5.0")]
    pub fn precision(&self) -> Option<usize> { loop { } }

    #[stable(feature = "fmt_flags", since = "1.5.0")]
    pub fn sign_plus(&self) -> bool { loop { } }

    #[stable(feature = "fmt_flags", since = "1.5.0")]
    pub fn sign_minus(&self) -> bool { loop { } }

    #[stable(feature = "fmt_flags", since = "1.5.0")]
    pub fn alternate(&self) -> bool { loop { } }

    #[stable(feature = "fmt_flags", since = "1.5.0")]
    pub fn sign_aware_zero_pad(&self) -> bool { loop { } }

    fn debug_lower_hex(&self) -> bool { loop { } }

    fn debug_upper_hex(&self) -> bool { loop { } }

    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn debug_struct<'b>(&'b mut self, name: &str) -> DebugStruct<'b, 'a> { loop { } }

    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn debug_tuple<'b>(&'b mut self, _name: &str) -> DebugTuple<'b, 'a> { loop { } }

    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn debug_list<'b>(&'b mut self) -> DebugList<'b, 'a> { loop { } }

    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn debug_set<'b>(&'b mut self) -> DebugSet<'b, 'a> { loop { } }

    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn debug_map<'b>(&'b mut self) -> DebugMap<'b, 'a> { loop { } }
}

#[stable(since = "1.2.0", feature = "formatter_write")]
impl Write for Formatter<'_> {
    fn write_str(&mut self, s: &str) -> Result { loop { } }

    fn write_char(&mut self, c: char) -> Result { loop { } }

    fn write_fmt(&mut self, args: Arguments<'_>) -> Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
}


macro_rules! fmt_refs {
    ($($tr:ident),*) => {
        $(
        #[stable(feature = "rust1", since = "1.0.0")]
        impl<T: ?Sized + $tr> $tr for &T {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
        }
        #[stable(feature = "rust1", since = "1.0.0")]
        impl<T: ?Sized + $tr> $tr for &mut T {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
        }
        )*
    }
}

fmt_refs! { Debug, Display, Octal, Binary, LowerHex, UpperHex, LowerExp, UpperExp }

#[unstable(feature = "never_type", issue = "35121")]
impl Debug for ! {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result { loop { } }
}

#[unstable(feature = "never_type", issue = "35121")]
impl Display for ! {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Debug for bool {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Display for bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Debug for str {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Display for str {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Debug for char {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Display for char {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Pointer for *const T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Pointer for *mut T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Pointer for &T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Pointer for &mut T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
}


#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Debug for *const T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Debug for *mut T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
}

macro_rules! peel {
    ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
}

macro_rules! tuple {
    () => ();
    ( $($name:ident,)+ ) => (
        #[stable(feature = "rust1", since = "1.0.0")]
        impl<$($name:Debug),+> Debug for ($($name,)+) where last_type!($($name,)+): ?Sized {
            #[allow(non_snake_case, unused_assignments)]
            fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
        }
        peel! { $($name,)+ }
    )
}

macro_rules! last_type {
    ($a:ident,) => { $a };
    ($a:ident, $($rest_a:ident,)+) => { last_type!($($rest_a,)+) };
}

tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: Debug> Debug for [T] {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Debug for () {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Debug for PhantomData<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { loop { } }
}
