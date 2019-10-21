//! Utilities for formatting and printing strings.

#![stable(feature = "rust1", since = "1.0.0")]

use crate::cell::{UnsafeCell, Cell, RefCell, Ref, RefMut};
use crate::marker::PhantomData;
use crate::mem;
use crate::num::flt2dec;
use crate::ops::Deref;
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
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Error;

#[stable(feature = "rust1", since = "1.0.0")]
pub trait Write {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn write_str(&mut self, s: &str) -> Result;

    #[stable(feature = "fmt_write_char", since = "1.1.0")]
    fn write_char(&mut self, c: char) -> Result {
        self.write_str(c.encode_utf8(&mut [0; 4]))
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    fn write_fmt(mut self: &mut Self, args: Arguments<'_>) -> Result {
        write(&mut self, args)
    }
}

#[stable(feature = "fmt_write_blanket_impl", since = "1.4.0")]
impl<W: Write + ?Sized> Write for &mut W {
    fn write_str(&mut self, s: &str) -> Result {
        (**self).write_str(s)
    }

    fn write_char(&mut self, c: char) -> Result {
        (**self).write_char(c)
    }

    fn write_fmt(&mut self, args: Arguments<'_>) -> Result {
        (**self).write_fmt(args)
    }
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
    fn show_usize(x: &usize, f: &mut Formatter<'_>) -> Result {
        Display::fmt(x, f)
    }

    #[doc(hidden)]
    #[unstable(feature = "fmt_internals", reason = "internal to format_args!",
               issue = "0")]
    pub fn new<'b, T>(x: &'b T,
                      f: fn(&T, &mut Formatter<'_>) -> Result) -> ArgumentV1<'b> {
        unsafe {
            ArgumentV1 {
                formatter: mem::transmute(f),
                value: mem::transmute(x)
            }
        }
    }

    #[doc(hidden)]
    #[unstable(feature = "fmt_internals", reason = "internal to format_args!",
               issue = "0")]
    pub fn from_usize(x: &usize) -> ArgumentV1<'_> {
        ArgumentV1::new(x, ArgumentV1::show_usize)
    }

    fn as_usize(&self) -> Option<usize> {
        if self.formatter as usize == ArgumentV1::show_usize as usize {
            Some(unsafe { *(self.value as *const _ as *const usize) })
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
enum FlagV1 { SignPlus, SignMinus, Alternate, SignAwareZeroPad, DebugLowerHex, DebugUpperHex }

impl<'a> Arguments<'a> {
    #[doc(hidden)] #[inline]
    #[unstable(feature = "fmt_internals", reason = "internal to format_args!",
               issue = "0")]
    pub fn new_v1(pieces: &'a [&'a str],
                  args: &'a [ArgumentV1<'a>]) -> Arguments<'a> {
        Arguments {
            pieces,
            fmt: None,
            args,
        }
    }

    #[doc(hidden)] #[inline]
    #[unstable(feature = "fmt_internals", reason = "internal to format_args!",
               issue = "0")]
    pub fn new_v1_formatted(pieces: &'a [&'a str],
                            args: &'a [ArgumentV1<'a>],
                            fmt: &'a [rt::v1::Argument]) -> Arguments<'a> {
        Arguments {
            pieces,
            fmt: Some(fmt),
            args,
        }
    }

    #[doc(hidden)] #[inline]
    #[unstable(feature = "fmt_internals", reason = "internal to format_args!",
               issue = "0")]
    pub fn estimated_capacity(&self) -> usize {
        let pieces_length: usize = self.pieces.iter()
            .map(|x| x.len()).sum();

        if self.args.is_empty() {
            pieces_length
        } else if self.pieces[0] == "" && pieces_length < 16 {
            0
        } else {
            pieces_length.checked_mul(2).unwrap_or(0)
        }
    }
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
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        Display::fmt(self, fmt)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Display for Arguments<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        write(fmt.buf, *self)
    }
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
pub fn write(output: &mut dyn Write, args: Arguments<'_>) -> Result {
    let mut formatter = Formatter {
        flags: 0,
        width: None,
        precision: None,
        buf: output,
        align: rt::v1::Alignment::Unknown,
        fill: ' ',
        args: args.args,
        curarg: args.args.iter(),
    };

    let mut idx = 0;

    match args.fmt {
        None => {
            for (arg, piece) in args.args.iter().zip(args.pieces.iter()) {
                formatter.buf.write_str(*piece)?;
                (arg.formatter)(arg.value, &mut formatter)?;
                idx += 1;
            }
        }
        Some(fmt) => {
            for (arg, piece) in fmt.iter().zip(args.pieces.iter()) {
                formatter.buf.write_str(*piece)?;
                formatter.run(arg)?;
                idx += 1;
            }
        }
    }

    if let Some(piece) = args.pieces.get(idx) {
        formatter.buf.write_str(*piece)?;
    }

    Ok(())
}

#[must_use = "don't forget to write the post padding"]
struct PostPadding {
    fill: char,
    padding: usize,
}

impl PostPadding {
    fn new(fill: char, padding: usize) -> PostPadding {
        PostPadding { fill, padding }
    }

    fn write(self, buf: &mut dyn Write) -> Result {
        for _ in 0..self.padding {
            buf.write_char(self.fill)?;
        }
        Ok(())
    }
}

impl<'a> Formatter<'a> {
    fn wrap_buf<'b, 'c, F>(&'b mut self, wrap: F) -> Formatter<'c>
        where 'b: 'c, F: FnOnce(&'b mut (dyn Write+'b)) -> &'c mut (dyn Write+'c)
    {
        Formatter {
            buf: wrap(self.buf),

            flags: self.flags,
            fill: self.fill,
            align: self.align,
            width: self.width,
            precision: self.precision,

            curarg: self.curarg.clone(),
            args: self.args,
        }
    }

    fn run(&mut self, arg: &rt::v1::Argument) -> Result {
        self.fill = arg.format.fill;
        self.align = arg.format.align;
        self.flags = arg.format.flags;
        self.width = self.getcount(&arg.format.width);
        self.precision = self.getcount(&arg.format.precision);

        let value = match arg.position {
            rt::v1::Position::Next => { *self.curarg.next().unwrap() }
            rt::v1::Position::At(i) => self.args[i],
        };

        (value.formatter)(value.value, self)
    }

    fn getcount(&mut self, cnt: &rt::v1::Count) -> Option<usize> {
        match *cnt {
            rt::v1::Count::Is(n) => Some(n),
            rt::v1::Count::Implied => None,
            rt::v1::Count::Param(i) => {
                self.args[i].as_usize()
            }
            rt::v1::Count::NextParam => {
                self.curarg.next()?.as_usize()
            }
        }
    }


    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn pad_integral(&mut self,
                        is_nonnegative: bool,
                        prefix: &str,
                        buf: &str)
                        -> Result {
        let mut width = buf.len();

        let mut sign = None;
        if !is_nonnegative {
            sign = Some('-'); width += 1;
        } else if self.sign_plus() {
            sign = Some('+'); width += 1;
        }

        let prefix = if self.alternate() {
            width += prefix.chars().count();
            Some(prefix)
        } else {
            None
        };

        #[inline(never)]
        fn write_prefix(f: &mut Formatter<'_>, sign: Option<char>, prefix: Option<&str>) -> Result {
            if let Some(c) = sign {
                f.buf.write_char(c)?;
            }
            if let Some(prefix) = prefix {
                f.buf.write_str(prefix)
            } else {
                Ok(())
            }
        }

        match self.width {
            None => {
                write_prefix(self, sign, prefix)?;
                self.buf.write_str(buf)
            }
            Some(min) if width >= min => {
                write_prefix(self, sign, prefix)?;
                self.buf.write_str(buf)
            }
            Some(min) if self.sign_aware_zero_pad() => {
                self.fill = '0';
                self.align = rt::v1::Alignment::Right;
                write_prefix(self, sign, prefix)?;
                let post_padding = self.padding(min - width, rt::v1::Alignment::Right)?;
                self.buf.write_str(buf)?;
                post_padding.write(self.buf)
            }
            Some(min) => {
                let post_padding = self.padding(min - width, rt::v1::Alignment::Right)?;
                write_prefix(self, sign, prefix)?;
                self.buf.write_str(buf)?;
                post_padding.write(self.buf)
            }
        }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn pad(&mut self, s: &str) -> Result {
        if self.width.is_none() && self.precision.is_none() {
            return self.buf.write_str(s);
        }
        let s = if let Some(max) = self.precision {
            if let Some((i, _)) = s.char_indices().nth(max) {
                s.get(..i).unwrap_or(&s)
            } else {
                &s
            }
        } else {
            &s
        };
        match self.width {
            None => self.buf.write_str(s),
            Some(width) if s.chars().count() >= width => {
                self.buf.write_str(s)
            }
            Some(width) => {
                let align = rt::v1::Alignment::Left;
                let post_padding = self.padding(width - s.chars().count(), align)?;
                self.buf.write_str(s)?;
                post_padding.write(self.buf)
            }
        }
    }

    fn padding(
        &mut self,
        padding: usize,
        default: rt::v1::Alignment
    ) -> result::Result<PostPadding, Error> {
        let align = match self.align {
            rt::v1::Alignment::Unknown => default,
            _ => self.align
        };

        let (pre_pad, post_pad) = match align {
            rt::v1::Alignment::Left => (0, padding),
            rt::v1::Alignment::Right |
            rt::v1::Alignment::Unknown => (padding, 0),
            rt::v1::Alignment::Center => (padding / 2, (padding + 1) / 2),
        };

        for _ in 0..pre_pad {
            self.buf.write_char(self.fill)?;
        }

        Ok(PostPadding::new(self.fill, post_pad))
    }

    fn pad_formatted_parts(&mut self, formatted: &flt2dec::Formatted<'_>) -> Result {
        if let Some(mut width) = self.width {
            let mut formatted = formatted.clone();
            let old_fill = self.fill;
            let old_align = self.align;
            let mut align = old_align;
            if self.sign_aware_zero_pad() {
                let sign = unsafe { str::from_utf8_unchecked(formatted.sign) };
                self.buf.write_str(sign)?;

                formatted.sign = b"";
                width = width.saturating_sub(sign.len());
                align = rt::v1::Alignment::Right;
                self.fill = '0';
                self.align = rt::v1::Alignment::Right;
            }

            let len = formatted.len();
            let ret = if width <= len { // no padding
                self.write_formatted_parts(&formatted)
            } else {
                let post_padding = self.padding(width - len, align)?;
                self.write_formatted_parts(&formatted)?;
                post_padding.write(self.buf)
            };
            self.fill = old_fill;
            self.align = old_align;
            ret
        } else {
            self.write_formatted_parts(formatted)
        }
    }

    fn write_formatted_parts(&mut self, formatted: &flt2dec::Formatted<'_>) -> Result {
        fn write_bytes(buf: &mut dyn Write, s: &[u8]) -> Result {
            buf.write_str(unsafe { str::from_utf8_unchecked(s) })
        }

        if !formatted.sign.is_empty() {
            write_bytes(self.buf, formatted.sign)?;
        }
        for part in formatted.parts {
            match *part {
                flt2dec::Part::Zero(mut nzeroes) => {
                    const ZEROES: &str = // 64 zeroes
                        "0000000000000000000000000000000000000000000000000000000000000000";
                    while nzeroes > ZEROES.len() {
                        self.buf.write_str(ZEROES)?;
                        nzeroes -= ZEROES.len();
                    }
                    if nzeroes > 0 {
                        self.buf.write_str(&ZEROES[..nzeroes])?;
                    }
                }
                flt2dec::Part::Num(mut v) => {
                    let mut s = [0; 5];
                    let len = part.len();
                    for c in s[..len].iter_mut().rev() {
                        *c = b'0' + (v % 10) as u8;
                        v /= 10;
                    }
                    write_bytes(self.buf, &s[..len])?;
                }
                flt2dec::Part::Copy(buf) => {
                    write_bytes(self.buf, buf)?;
                }
            }
        }
        Ok(())
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn write_str(&mut self, data: &str) -> Result {
        self.buf.write_str(data)
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result {
        write(self.buf, fmt)
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_deprecated(since = "1.24.0",
                       reason = "use the `sign_plus`, `sign_minus`, `alternate`, \
                                 or `sign_aware_zero_pad` methods instead")]
    pub fn flags(&self) -> u32 { self.flags }

    #[stable(feature = "fmt_flags", since = "1.5.0")]
    pub fn fill(&self) -> char { self.fill }

    #[stable(feature = "fmt_flags_align", since = "1.28.0")]
    pub fn align(&self) -> Option<Alignment> {
        match self.align {
            rt::v1::Alignment::Left => Some(Alignment::Left),
            rt::v1::Alignment::Right => Some(Alignment::Right),
            rt::v1::Alignment::Center => Some(Alignment::Center),
            rt::v1::Alignment::Unknown => None,
        }
    }

    #[stable(feature = "fmt_flags", since = "1.5.0")]
    pub fn width(&self) -> Option<usize> { self.width }

    #[stable(feature = "fmt_flags", since = "1.5.0")]
    pub fn precision(&self) -> Option<usize> { self.precision }

    #[stable(feature = "fmt_flags", since = "1.5.0")]
    pub fn sign_plus(&self) -> bool { self.flags & (1 << FlagV1::SignPlus as u32) != 0 }

    #[stable(feature = "fmt_flags", since = "1.5.0")]
    pub fn sign_minus(&self) -> bool { self.flags & (1 << FlagV1::SignMinus as u32) != 0 }

    #[stable(feature = "fmt_flags", since = "1.5.0")]
    pub fn alternate(&self) -> bool { self.flags & (1 << FlagV1::Alternate as u32) != 0 }

    #[stable(feature = "fmt_flags", since = "1.5.0")]
    pub fn sign_aware_zero_pad(&self) -> bool {
        self.flags & (1 << FlagV1::SignAwareZeroPad as u32) != 0
    }

    fn debug_lower_hex(&self) -> bool { self.flags & (1 << FlagV1::DebugLowerHex as u32) != 0 }

    fn debug_upper_hex(&self) -> bool { self.flags & (1 << FlagV1::DebugUpperHex as u32) != 0 }

    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn debug_struct<'b>(&'b mut self, name: &str) -> DebugStruct<'b, 'a> {
        builders::debug_struct_new(self, name)
    }

    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn debug_tuple<'b>(&'b mut self, _name: &str) -> DebugTuple<'b, 'a> {
        loop { }
    }

    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn debug_list<'b>(&'b mut self) -> DebugList<'b, 'a> {
        builders::debug_list_new(self)
    }

    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn debug_set<'b>(&'b mut self) -> DebugSet<'b, 'a> {
        builders::debug_set_new(self)
    }

    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn debug_map<'b>(&'b mut self) -> DebugMap<'b, 'a> {
        builders::debug_map_new(self)
    }
}

#[stable(since = "1.2.0", feature = "formatter_write")]
impl Write for Formatter<'_> {
    fn write_str(&mut self, s: &str) -> Result {
        self.buf.write_str(s)
    }

    fn write_char(&mut self, c: char) -> Result {
        self.buf.write_char(c)
    }

    fn write_fmt(&mut self, args: Arguments<'_>) -> Result {
        write(self.buf, args)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt("an error occurred when formatting an argument", f)
    }
}


macro_rules! fmt_refs {
    ($($tr:ident),*) => {
        $(
        #[stable(feature = "rust1", since = "1.0.0")]
        impl<T: ?Sized + $tr> $tr for &T {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result { $tr::fmt(&**self, f) }
        }
        #[stable(feature = "rust1", since = "1.0.0")]
        impl<T: ?Sized + $tr> $tr for &mut T {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result { $tr::fmt(&**self, f) }
        }
        )*
    }
}

fmt_refs! { Debug, Display, Octal, Binary, LowerHex, UpperHex, LowerExp, UpperExp }

#[unstable(feature = "never_type", issue = "35121")]
impl Debug for ! {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result {
        *self
    }
}

#[unstable(feature = "never_type", issue = "35121")]
impl Display for ! {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result {
        *self
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Debug for bool {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Display for bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(if *self { "true" } else { "false" }, f)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Debug for str {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_char('"')?;
        let mut from = 0;
        for (i, c) in self.char_indices() {
            let esc = c.escape_debug();
            if esc.len() != 1 {
                f.write_str(&self[from..i])?;
                for c in esc {
                    f.write_char(c)?;
                }
                from = i + c.len_utf8();
            }
        }
        f.write_str(&self[from..])?;
        f.write_char('"')
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Display for str {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.pad(self)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Debug for char {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_char('\'')?;
        for c in self.escape_debug() {
            f.write_char(c)?
        }
        f.write_char('\'')
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Display for char {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if f.width.is_none() && f.precision.is_none() {
            f.write_char(*self)
        } else {
            f.pad(self.encode_utf8(&mut [0; 4]))
        }
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Pointer for *const T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let old_width = f.width;
        let old_flags = f.flags;

        if f.alternate() {
            f.flags |= 1 << (FlagV1::SignAwareZeroPad as u32);

            if let None = f.width {
                f.width = Some(((mem::size_of::<usize>() * 8) / 4) + 2);
            }
        }
        f.flags |= 1 << (FlagV1::Alternate as u32);

        let ret = LowerHex::fmt(&(*self as *const () as usize), f);

        f.width = old_width;
        f.flags = old_flags;

        ret
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Pointer for *mut T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Pointer::fmt(&(*self as *const T), f)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Pointer for &T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Pointer::fmt(&(*self as *const T), f)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Pointer for &mut T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Pointer::fmt(&(&**self as *const T), f)
    }
}


#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Debug for *const T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { Pointer::fmt(self, f) }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Debug for *mut T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { Pointer::fmt(self, f) }
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
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                let mut builder = f.debug_tuple("");
                let ($(ref $name,)+) = *self;
                $(
                    builder.field(&$name);
                )+

                builder.finish()
            }
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Debug for () {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.pad("()")
    }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Debug for PhantomData<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.pad("PhantomData")
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: Copy + Debug> Debug for Cell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Cell")
            .field("value", &self.get())
            .finish()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized + Debug> Debug for RefCell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.try_borrow() {
            Ok(borrow) => {
                f.debug_struct("RefCell")
                    .field("value", &borrow)
                    .finish()
            }
            Err(_) => {
                struct BorrowedPlaceholder;

                impl Debug for BorrowedPlaceholder {
                    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                        f.write_str("<borrowed>")
                    }
                }

                f.debug_struct("RefCell")
                    .field("value", &BorrowedPlaceholder)
                    .finish()
            }
        }
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized + Debug> Debug for Ref<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(&**self, f)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized + Debug> Debug for RefMut<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(&*(self.deref()), f)
    }
}

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<T: ?Sized + Debug> Debug for UnsafeCell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.pad("UnsafeCell")
    }
}

