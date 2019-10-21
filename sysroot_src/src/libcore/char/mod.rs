//! A character type.

#![allow(non_snake_case)]
#![stable(feature = "core_char", since = "1.2.0")]

mod convert;
mod decode;
mod methods;

#[stable(feature = "rust1", since = "1.0.0")]
pub use self::convert::{from_u32, from_digit};
#[stable(feature = "char_from_unchecked", since = "1.5.0")]
pub use self::convert::from_u32_unchecked;
#[stable(feature = "char_from_str", since = "1.20.0")]
pub use self::convert::ParseCharError;
#[stable(feature = "try_from", since = "1.34.0")]
pub use self::convert::CharTryFromError;
#[stable(feature = "decode_utf16", since = "1.9.0")]
pub use self::decode::{decode_utf16, DecodeUtf16, DecodeUtf16Error};

use crate::fmt::{self};

const TAG_CONT: u8     = 0b1000_0000;
const TAG_TWO_B: u8    = 0b1100_0000;
const TAG_THREE_B: u8  = 0b1110_0000;
const TAG_FOUR_B: u8   = 0b1111_0000;
const MAX_ONE_B: u32   =     0x80;
const MAX_TWO_B: u32   =    0x800;
const MAX_THREE_B: u32 =  0x10000;

/*
    Lu  Uppercase_Letter        an uppercase letter
    Ll  Lowercase_Letter        a lowercase letter
    Lt  Titlecase_Letter        a digraphic character, with first part uppercase
    Lm  Modifier_Letter         a modifier letter
    Lo  Other_Letter            other letters, including syllables and ideographs
    Mn  Nonspacing_Mark         a nonspacing combining mark (zero advance width)
    Mc  Spacing_Mark            a spacing combining mark (positive advance width)
    Me  Enclosing_Mark          an enclosing combining mark
    Nd  Decimal_Number          a decimal digit
    Nl  Letter_Number           a letterlike numeric character
    No  Other_Number            a numeric character of other type
    Pc  Connector_Punctuation   a connecting punctuation mark, like a tie
    Pd  Dash_Punctuation        a dash or hyphen punctuation mark
    Ps  Open_Punctuation        an opening punctuation mark (of a pair)
    Pe  Close_Punctuation       a closing punctuation mark (of a pair)
    Pi  Initial_Punctuation     an initial quotation mark
    Pf  Final_Punctuation       a final quotation mark
    Po  Other_Punctuation       a punctuation mark of other type
    Sm  Math_Symbol             a symbol of primarily mathematical use
    Sc  Currency_Symbol         a currency sign
    Sk  Modifier_Symbol         a non-letterlike modifier symbol
    So  Other_Symbol            a symbol of other type
    Zs  Space_Separator         a space character (of various non-zero widths)
    Zl  Line_Separator          U+2028 LINE SEPARATOR only
    Zp  Paragraph_Separator     U+2029 PARAGRAPH SEPARATOR only
    Cc  Control                 a C0 or C1 control code
    Cf  Format                  a format control character
    Cs  Surrogate               a surrogate code point
    Co  Private_Use             a private-use character
    Cn  Unassigned              a reserved unassigned code point or a noncharacter
*/

#[stable(feature = "rust1", since = "1.0.0")]
pub const MAX: char = '\u{10ffff}';

#[stable(feature = "decode_utf16", since = "1.9.0")]
pub const REPLACEMENT_CHARACTER: char = '\u{FFFD}';

#[derive(Clone, Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct EscapeUnicode {
    c: char,
    state: EscapeUnicodeState,

    hex_digit_idx: usize,
}

#[derive(Clone, Debug)]
enum EscapeUnicodeState {
    Done,
    RightBrace,
    Value,
    LeftBrace,
    Type,
    Backslash,
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Iterator for EscapeUnicode {
    type Item = char;

    fn next(&mut self) -> Option<char> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn count(self) -> usize { loop { } }

    fn last(self) -> Option<char> { loop { } }
}

#[stable(feature = "char_struct_display", since = "1.16.0")]
impl fmt::Display for EscapeUnicode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[derive(Clone, Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct EscapeDefault {
    state: EscapeDefaultState
}

#[derive(Clone, Debug)]
enum EscapeDefaultState {
    Done,
    Char(char),
    Backslash(char),
    Unicode(EscapeUnicode),
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Iterator for EscapeDefault {
    type Item = char;

    fn next(&mut self) -> Option<char> { loop { } }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }

    #[inline]
    fn count(self) -> usize { loop { } }

    fn nth(&mut self, n: usize) -> Option<char> { loop { } }

    fn last(self) -> Option<char> { loop { } }
}


#[stable(feature = "char_struct_display", since = "1.16.0")]
impl fmt::Display for EscapeDefault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "char_escape_debug", since = "1.20.0")]
#[derive(Clone, Debug)]
pub struct EscapeDebug(EscapeDefault);

#[stable(feature = "char_escape_debug", since = "1.20.0")]
impl Iterator for EscapeDebug {
    type Item = char;
    fn next(&mut self) -> Option<char> { loop { } }
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }
}

#[stable(feature = "char_escape_debug", since = "1.20.0")]
impl fmt::Display for EscapeDebug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Debug, Clone)]
pub struct ToLowercase(CaseMappingIter);

#[stable(feature = "rust1", since = "1.0.0")]
impl Iterator for ToLowercase {
    type Item = char;
    fn next(&mut self) -> Option<char> { loop { } }
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }
}


#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Debug, Clone)]
pub struct ToUppercase(CaseMappingIter);

#[stable(feature = "rust1", since = "1.0.0")]
impl Iterator for ToUppercase {
    type Item = char;
    fn next(&mut self) -> Option<char> { loop { } }
    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }
}

#[derive(Debug, Clone)]
enum CaseMappingIter {
    Three(char, char, char),
    Two(char, char),
    One(char),
    Zero,
}

impl CaseMappingIter {
    fn new(chars: [char; 3]) -> CaseMappingIter { loop { } }
}

impl Iterator for CaseMappingIter {
    type Item = char;
    fn next(&mut self) -> Option<char> { loop { } }

    fn size_hint(&self) -> (usize, Option<usize>) { loop { } }
}

impl fmt::Display for CaseMappingIter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "char_struct_display", since = "1.16.0")]
impl fmt::Display for ToLowercase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "char_struct_display", since = "1.16.0")]
impl fmt::Display for ToUppercase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}
