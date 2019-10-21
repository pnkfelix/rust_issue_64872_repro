//! impl char {}

use super::*;

#[lang = "char"]
impl char {
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn is_digit(self, radix: u32) -> bool { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn to_digit(self, radix: u32) -> Option<u32> { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn escape_unicode(self) -> EscapeUnicode { loop { } }

    #[inline]
    pub(crate) fn escape_debug_ext(self, escape_grapheme_extended: bool) -> EscapeDebug { loop { } }

    #[stable(feature = "char_escape_debug", since = "1.20.0")]
    #[inline]
    pub fn escape_debug(self) -> EscapeDebug { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn escape_default(self) -> EscapeDefault { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn len_utf8(self) -> usize { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn len_utf16(self) -> usize { loop { } }

    #[stable(feature = "unicode_encode_char", since = "1.15.0")]
    #[inline]
    pub fn encode_utf8(self, dst: &mut [u8]) -> &mut str { loop { } }

    #[stable(feature = "unicode_encode_char", since = "1.15.0")]
    #[inline]
    pub fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16] { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn is_alphabetic(self) -> bool { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn is_lowercase(self) -> bool { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn is_uppercase(self) -> bool { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn is_whitespace(self) -> bool { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn is_alphanumeric(self) -> bool { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn is_control(self) -> bool { loop { } }

    #[inline]
    pub(crate) fn is_grapheme_extended(self) -> bool { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn is_numeric(self) -> bool { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn to_lowercase(self) -> ToLowercase { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn to_uppercase(self) -> ToUppercase { loop { } }

    #[stable(feature = "ascii_methods_on_intrinsics", since = "1.23.0")]
    #[inline]
    pub const fn is_ascii(&self) -> bool { false }

    #[stable(feature = "ascii_methods_on_intrinsics", since = "1.23.0")]
    #[inline]
    pub fn to_ascii_uppercase(&self) -> char { loop { } }

    #[stable(feature = "ascii_methods_on_intrinsics", since = "1.23.0")]
    #[inline]
    pub fn to_ascii_lowercase(&self) -> char { loop { } }

    #[stable(feature = "ascii_methods_on_intrinsics", since = "1.23.0")]
    #[inline]
    pub fn eq_ignore_ascii_case(&self, other: &char) -> bool { loop { } }

    #[stable(feature = "ascii_methods_on_intrinsics", since = "1.23.0")]
    #[inline]
    pub fn make_ascii_uppercase(&mut self) { loop { } }

    #[stable(feature = "ascii_methods_on_intrinsics", since = "1.23.0")]
    #[inline]
    pub fn make_ascii_lowercase(&mut self) { loop { } }

    #[stable(feature = "ascii_ctype_on_intrinsics", since = "1.24.0")]
    #[inline]
    pub fn is_ascii_alphabetic(&self) -> bool { loop { } }

    #[stable(feature = "ascii_ctype_on_intrinsics", since = "1.24.0")]
    #[inline]
    pub fn is_ascii_uppercase(&self) -> bool { loop { } }

    #[stable(feature = "ascii_ctype_on_intrinsics", since = "1.24.0")]
    #[inline]
    pub fn is_ascii_lowercase(&self) -> bool { loop { } }

    #[stable(feature = "ascii_ctype_on_intrinsics", since = "1.24.0")]
    #[inline]
    pub fn is_ascii_alphanumeric(&self) -> bool { loop { } }

    #[stable(feature = "ascii_ctype_on_intrinsics", since = "1.24.0")]
    #[inline]
    pub fn is_ascii_digit(&self) -> bool { loop { } }

    #[stable(feature = "ascii_ctype_on_intrinsics", since = "1.24.0")]
    #[inline]
    pub fn is_ascii_hexdigit(&self) -> bool { loop { } }

    #[stable(feature = "ascii_ctype_on_intrinsics", since = "1.24.0")]
    #[inline]
    pub fn is_ascii_punctuation(&self) -> bool { loop { } }

    #[stable(feature = "ascii_ctype_on_intrinsics", since = "1.24.0")]
    #[inline]
    pub fn is_ascii_graphic(&self) -> bool { loop { } }

    #[stable(feature = "ascii_ctype_on_intrinsics", since = "1.24.0")]
    #[inline]
    pub fn is_ascii_whitespace(&self) -> bool { loop { } }

    #[stable(feature = "ascii_ctype_on_intrinsics", since = "1.24.0")]
    #[inline]
    pub fn is_ascii_control(&self) -> bool { loop { } }
}
