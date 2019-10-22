// ignore-tidy-filelength


#![stable(feature = "rust1", since = "1.0.0")]

use self::pattern::Pattern;
use self::pattern::{ReverseSearcher};

use crate::char;
use crate::fmt::{self};
// use crate::iter::{Map, Cloned, FusedIterator, Filter};
// use crate::iter::{Flatten, FlatMap, Chain};
use crate::slice::{self, SliceIndex};

pub mod pattern;

#[unstable(feature = "str_internals", issue = "0")]
#[allow(missing_docs)]
pub mod lossy;

#[stable(feature = "rust1", since = "1.0.0")]
pub trait FromStr: Sized {
    #[stable(feature = "rust1", since = "1.0.0")]
    type Err;

    #[stable(feature = "rust1", since = "1.0.0")]
    fn from_str(s: &str) -> Result<Self, Self::Err>;
}

#[stable(feature = "rust1", since = "1.0.0")]
impl FromStr for bool {
    type Err = ParseBoolError;

    #[inline]
    fn from_str(s: &str) -> Result<bool, ParseBoolError> { loop { } }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct ParseBoolError { _priv: () }

#[stable(feature = "rust1", since = "1.0.0")]
impl fmt::Display for ParseBoolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

/*
Section: Creating a string
*/

#[derive(Copy, Eq, PartialEq, Clone, Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Utf8Error {
    valid_up_to: usize,
    error_len: Option<u8>,
}

impl Utf8Error {
    #[stable(feature = "utf8_error", since = "1.5.0")]
    pub fn valid_up_to(&self) -> usize { loop { } }

    #[stable(feature = "utf8_error_error_len", since = "1.20.0")]
    pub fn error_len(&self) -> Option<usize> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
pub fn from_utf8(v: &[u8]) -> Result<&str, Utf8Error> { loop { } }

#[stable(feature = "str_mut_extras", since = "1.20.0")]
pub fn from_utf8_mut(v: &mut [u8]) -> Result<&mut str, Utf8Error> { loop { } }

#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
pub unsafe fn from_utf8_unchecked(v: &[u8]) -> &str { loop { } }

#[inline]
#[stable(feature = "str_mut_extras", since = "1.20.0")]
pub unsafe fn from_utf8_unchecked_mut(v: &mut [u8]) -> &mut str { loop { } }

#[stable(feature = "rust1", since = "1.0.0")]
impl fmt::Display for Utf8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

/*
Section: Iterators
*/

#[derive(Clone)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Chars<'a> {
    iter: slice::Iter<'a, u8>
}

#[inline]
fn utf8_first_byte(byte: u8, width: u32) -> u32 { loop { } }

#[inline]
fn utf8_acc_cont_byte(ch: u32, byte: u8) -> u32 { loop { } }

#[inline]
fn utf8_is_cont_byte(byte: u8) -> bool { loop { } }

#[inline]
fn unwrap_or_0(opt: Option<&u8>) -> u8 { loop { } }

#[unstable(feature = "str_internals", issue = "0")]
#[inline]
pub fn next_code_point<'a, I: Iterator<Item = &'a u8>>(bytes: &mut I) -> Option<u32> { loop { } }

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a> Iterator for Chars<'a> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> { loop { } }
}

#[stable(feature = "chars_debug_impl", since = "1.38.0")]
impl fmt::Debug for Chars<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

impl<'a> Chars<'a> {
    #[stable(feature = "iter_to_slice", since = "1.4.0")]
    #[inline]
    pub fn as_str(&self) -> &'a str { loop { } }
}

#[derive(Clone, Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct CharIndices<'a> {
    front_offset: usize,
    iter: Chars<'a>,
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a> Iterator for CharIndices<'a> {
    type Item = (usize, char);

    #[inline]
    fn next(&mut self) -> Option<(usize, char)> { loop { } }
}

impl<'a> CharIndices<'a> {
    #[stable(feature = "iter_to_slice", since = "1.4.0")]
    #[inline]
    pub fn as_str(&self) -> &'a str { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Clone, Debug)]
pub struct Bytes<'a>(&'a ());

#[stable(feature = "rust1", since = "1.0.0")]
impl Iterator for Bytes<'_> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<u8> { loop { } }
}

macro_rules! derive_pattern_clone {
    (clone $t:ident with |$s:ident| $e:expr) => {
        impl<'a, P> Clone for $t<'a, P>
        where
            P: Pattern<'a, Searcher: Clone>,
        {
            fn clone(&self) -> Self { loop { } }
        }
    }
}

derive_pattern_clone!{
    clone SplitInternal
    with |s| SplitInternal { matcher: s.matcher.clone(), ..*s }
}

struct SplitInternal<'a, P: Pattern<'a>> {
    start: usize,
    end: usize,
    matcher: P::Searcher,
    allow_trailing_empty: bool,
    finished: bool,
}

impl<'a, P> fmt::Debug for SplitInternal<'a, P>
where
    P: Pattern<'a, Searcher: fmt::Debug>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

impl<'a, P: Pattern<'a>> SplitInternal<'a, P> {
    #[inline]
    fn get_end(&mut self) -> Option<&'a str> { loop { } }

    #[inline]
    fn next(&mut self) -> Option<&'a str> { loop { } }

    #[inline]
    fn next_back(&mut self) -> Option<&'a str>
        where P::Searcher: ReverseSearcher<'a>
    { loop { } }
}

derive_pattern_clone!{
    clone SplitNInternal
    with |s| SplitNInternal { iter: s.iter.clone(), ..*s }
}

struct SplitNInternal<'a, P: Pattern<'a>> {
    iter: SplitInternal<'a, P>,
    count: usize,
}

impl<'a, P> fmt::Debug for SplitNInternal<'a, P>
where
    P: Pattern<'a, Searcher: fmt::Debug>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

impl<'a, P: Pattern<'a>> SplitNInternal<'a, P> {
    #[inline]
    fn next(&mut self) -> Option<&'a str> { loop { } }

    #[inline]
    fn next_back(&mut self) -> Option<&'a str>
        where P::Searcher: ReverseSearcher<'a>
    { loop { } }
}


derive_pattern_clone!{
    clone MatchIndicesInternal
    with |s| MatchIndicesInternal(s.0.clone())
}

struct MatchIndicesInternal<'a, P: Pattern<'a>>(P::Searcher);

impl<'a, P> fmt::Debug for MatchIndicesInternal<'a, P>
where
    P: Pattern<'a, Searcher: fmt::Debug>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

impl<'a, P: Pattern<'a>> MatchIndicesInternal<'a, P> {
    #[inline]
    fn next(&mut self) -> Option<(usize, &'a str)> { loop { } }

    #[inline]
    fn next_back(&mut self) -> Option<(usize, &'a str)>
        where P::Searcher: ReverseSearcher<'a>
    { loop { } }
}

derive_pattern_clone!{
    clone MatchesInternal
    with |s| MatchesInternal(s.0.clone())
}

struct MatchesInternal<'a, P: Pattern<'a>>(P::Searcher);

impl<'a, P> fmt::Debug for MatchesInternal<'a, P>
where
    P: Pattern<'a, Searcher: fmt::Debug>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

impl<'a, P: Pattern<'a>> MatchesInternal<'a, P> {
    #[inline]
    fn next(&mut self) -> Option<&'a str> { loop { } }

    #[inline]
    fn next_back(&mut self) -> Option<&'a str>
        where P::Searcher: ReverseSearcher<'a>
    { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Clone, Debug)]
pub struct Lines<'a>(&'a ());

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a> Iterator for Lines<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<&'a str> { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_deprecated(since = "1.4.0", reason = "use lines()/Lines instead now")]
#[derive(Clone, Debug)]
#[allow(deprecated)]
pub struct LinesAny<'a>(Lines<'a>);

impl_fn_for_zst! {
    #[derive(Clone)]
    struct LinesAnyMap impl<'a> Fn = |line: &'a str| -> &'a str {
        loop { }
    };
}

#[stable(feature = "rust1", since = "1.0.0")]
#[allow(deprecated)]
impl<'a> Iterator for LinesAny<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<&'a str> { loop { } }
}

/*
Section: UTF-8 validation
*/

const NONASCII_MASK: usize = 0x80808080_80808080u64 as usize;

#[inline]
fn contains_nonascii(x: usize) -> bool { loop { } }

#[inline]
fn run_utf8_validation(v: &[u8]) -> Result<(), Utf8Error> { loop { } }

static UTF8_CHAR_WIDTH: [u8; 256] = [
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x1F
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x3F
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x5F
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x7F
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0x9F
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0xBF
0,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,
2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2, // 0xDF
3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3, // 0xEF
4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0, // 0xFF
];

#[unstable(feature = "str_internals", issue = "0")]
#[inline]
pub fn utf8_char_width(b: u8) -> usize { loop { } }

const CONT_MASK: u8 = 0b0011_1111;
const TAG_CONT_U8: u8 = 0b1000_0000;

/*
Section: Trait implementations
*/

mod traits {
    use crate::cmp::Ordering;
    use crate::ops;
    use crate::slice::{SliceIndex};

    #[stable(feature = "rust1", since = "1.0.0")]
    impl Ord for str {
        #[inline]
        fn cmp(&self, other: &str) -> Ordering { loop { } }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl PartialEq for str {
        #[inline]
        fn eq(&self, other: &str) -> bool { loop { } }
        #[inline]
        fn ne(&self, other: &str) -> bool { loop { } }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl Eq for str {}

    #[stable(feature = "rust1", since = "1.0.0")]
    impl PartialOrd for str {
        #[inline]
        fn partial_cmp(&self, other: &str) -> Option<Ordering> { loop { } }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl<I> ops::Index<I> for str
    where
        I: SliceIndex<str>,
    {
        type Output = I::Output;

        #[inline]
        fn index(&self, index: I) -> &I::Output { loop { } }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl<I> ops::IndexMut<I> for str
    where
        I: SliceIndex<str>,
    {
        #[inline]
        fn index_mut(&mut self, index: I) -> &mut I::Output { loop { } }
    }

    #[inline(never)]
    #[cold]
    fn str_index_overflow_fail() -> ! { loop { } }

    #[stable(feature = "str_checked_slicing", since = "1.20.0")]
    impl SliceIndex<str> for ops::RangeFull {
        type Output = str;
        #[inline]
        fn get(self, slice: &str) -> Option<&Self::Output> { loop { } }
        #[inline]
        fn get_mut(self, slice: &mut str) -> Option<&mut Self::Output> { loop { } }
        #[inline]
        unsafe fn get_unchecked(self, slice: &str) -> &Self::Output { loop { } }
        #[inline]
        unsafe fn get_unchecked_mut(self, slice: &mut str) -> &mut Self::Output { loop { } }
        #[inline]
        fn index(self, slice: &str) -> &Self::Output { loop { } }
        #[inline]
        fn index_mut(self, slice: &mut str) -> &mut Self::Output { loop { } }
    }

    #[stable(feature = "str_checked_slicing", since = "1.20.0")]
    impl SliceIndex<str> for ops::Range<usize> {
        type Output = str;
        #[inline]
        fn get(self, slice: &str) -> Option<&Self::Output> { loop { } }
        #[inline]
        fn get_mut(self, slice: &mut str) -> Option<&mut Self::Output> { loop { } }
        #[inline]
        unsafe fn get_unchecked(self, slice: &str) -> &Self::Output { loop { } }
        #[inline]
        unsafe fn get_unchecked_mut(self, slice: &mut str) -> &mut Self::Output { loop { } }
        #[inline]
        fn index(self, slice: &str) -> &Self::Output { loop { } }
        #[inline]
        fn index_mut(self, slice: &mut str) -> &mut Self::Output { loop { } }
    }

    #[stable(feature = "str_checked_slicing", since = "1.20.0")]
    impl SliceIndex<str> for ops::RangeTo<usize> {
        type Output = str;
        #[inline]
        fn get(self, slice: &str) -> Option<&Self::Output> { loop { } }
        #[inline]
        fn get_mut(self, slice: &mut str) -> Option<&mut Self::Output> { loop { } }
        #[inline]
        unsafe fn get_unchecked(self, slice: &str) -> &Self::Output { loop { } }
        #[inline]
        unsafe fn get_unchecked_mut(self, slice: &mut str) -> &mut Self::Output { loop { } }
        #[inline]
        fn index(self, slice: &str) -> &Self::Output { loop { } }
        #[inline]
        fn index_mut(self, slice: &mut str) -> &mut Self::Output { loop { } }
    }

    #[stable(feature = "str_checked_slicing", since = "1.20.0")]
    impl SliceIndex<str> for ops::RangeFrom<usize> {
        type Output = str;
        #[inline]
        fn get(self, slice: &str) -> Option<&Self::Output> { loop { } }
        #[inline]
        fn get_mut(self, slice: &mut str) -> Option<&mut Self::Output> { loop { } }
        #[inline]
        unsafe fn get_unchecked(self, slice: &str) -> &Self::Output { loop { } }
        #[inline]
        unsafe fn get_unchecked_mut(self, slice: &mut str) -> &mut Self::Output { loop { } }
        #[inline]
        fn index(self, slice: &str) -> &Self::Output { loop { } }
        #[inline]
        fn index_mut(self, slice: &mut str) -> &mut Self::Output { loop { } }
    }

    #[stable(feature = "inclusive_range", since = "1.26.0")]
    impl SliceIndex<str> for ops::RangeInclusive<usize> {
        type Output = str;
        #[inline]
        fn get(self, slice: &str) -> Option<&Self::Output> { loop { } }
        #[inline]
        fn get_mut(self, slice: &mut str) -> Option<&mut Self::Output> { loop { } }
        #[inline]
        unsafe fn get_unchecked(self, slice: &str) -> &Self::Output { loop { } }
        #[inline]
        unsafe fn get_unchecked_mut(self, slice: &mut str) -> &mut Self::Output { loop { } }
        #[inline]
        fn index(self, slice: &str) -> &Self::Output { loop { } }
        #[inline]
        fn index_mut(self, slice: &mut str) -> &mut Self::Output { loop { } }
    }

    #[stable(feature = "inclusive_range", since = "1.26.0")]
    impl SliceIndex<str> for ops::RangeToInclusive<usize> {
        type Output = str;
        #[inline]
        fn get(self, slice: &str) -> Option<&Self::Output> { loop { } }
        #[inline]
        fn get_mut(self, slice: &mut str) -> Option<&mut Self::Output> { loop { } }
        #[inline]
        unsafe fn get_unchecked(self, slice: &str) -> &Self::Output { loop { } }
        #[inline]
        unsafe fn get_unchecked_mut(self, slice: &mut str) -> &mut Self::Output { loop { } }
        #[inline]
        fn index(self, slice: &str) -> &Self::Output { loop { } }
        #[inline]
        fn index_mut(self, slice: &mut str) -> &mut Self::Output { loop { } }
    }
}

fn truncate_to_char_boundary(s: &str, mut max: usize) -> (bool, &str) { loop { } }

#[inline(never)]
#[cold]
fn slice_error_fail(s: &str, begin: usize, end: usize) -> ! { loop { } }

#[lang = "str"]
#[cfg(not(test))]
impl str {
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub const fn len(&self) -> usize {
        0
    }

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub const fn is_empty(&self) -> bool {
        false
    }

    #[stable(feature = "is_char_boundary", since = "1.9.0")]
    #[inline]
    pub fn is_char_boundary(&self, index: usize) -> bool { loop { } }

    /*
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline(always)]
    #[allow(unused_attributes)]
    #[allow_internal_unstable(const_fn_union)]
    pub const fn as_bytes(&self) -> &[u8] {  }

    #[stable(feature = "str_mut_extras", since = "1.20.0")]
    #[inline(always)]
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut []
    }
     */
    
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub const fn as_ptr(&self) -> *const u8 {
        self as *const str as *const u8
    }

    #[stable(feature = "str_as_mut_ptr", since = "1.36.0")]
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 { loop { } }

    #[stable(feature = "str_checked_slicing", since = "1.20.0")]
    #[inline]
    pub fn get<I: SliceIndex<str>>(&self, i: I) -> Option<&I::Output> { loop { } }

    #[stable(feature = "str_checked_slicing", since = "1.20.0")]
    #[inline]
    pub fn get_mut<I: SliceIndex<str>>(&mut self, i: I) -> Option<&mut I::Output> { loop { } }

    #[stable(feature = "str_checked_slicing", since = "1.20.0")]
    #[inline]
    pub unsafe fn get_unchecked<I: SliceIndex<str>>(&self, i: I) -> &I::Output { loop { } }

    #[stable(feature = "str_checked_slicing", since = "1.20.0")]
    #[inline]
    pub unsafe fn get_unchecked_mut<I: SliceIndex<str>>(&mut self, i: I) -> &mut I::Output { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_deprecated(since = "1.29.0", reason = "use `get_unchecked(begin..end)` instead")]
    #[inline]
    pub unsafe fn slice_unchecked(&self, begin: usize, end: usize) -> &str { loop { } }

    #[stable(feature = "str_slice_mut", since = "1.5.0")]
    #[rustc_deprecated(since = "1.29.0", reason = "use `get_unchecked_mut(begin..end)` instead")]
    #[inline]
    pub unsafe fn slice_mut_unchecked(&mut self, begin: usize, end: usize) -> &mut str { loop { } }

    #[inline]
    #[stable(feature = "str_split_at", since = "1.4.0")]
    pub fn split_at(&self, mid: usize) -> (&str, &str) { loop { } }

    #[inline]
    #[stable(feature = "str_split_at", since = "1.4.0")]
    pub fn split_at_mut(&mut self, mid: usize) -> (&mut str, &mut str) { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn chars(&self) -> Chars<'_> { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn char_indices(&self) -> CharIndices<'_> { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn bytes(&self) -> Bytes<'_> { loop { } }

    #[stable(feature = "split_whitespace", since = "1.1.0")]
    #[inline]
    pub fn split_whitespace(&self) -> SplitWhitespace<'_> { loop { } }

    #[stable(feature = "split_ascii_whitespace", since = "1.34.0")]
    #[inline]
    pub fn split_ascii_whitespace(&self) -> SplitAsciiWhitespace<'_> { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn lines(&self) -> Lines<'_> { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_deprecated(since = "1.4.0", reason = "use lines() instead now")]
    #[inline]
    #[allow(deprecated)]
    pub fn lines_any(&self) -> LinesAny<'_> { loop { } }

    #[stable(feature = "encode_utf16", since = "1.8.0")]
    pub fn encode_utf16(&self) -> EncodeUtf16<'_> { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn contains<'a, P: Pattern<'a>>(&'a self, pat: P) -> bool { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn starts_with<'a, P: Pattern<'a>>(&'a self, pat: P) -> bool { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn ends_with<'a, P>(&'a self, pat: P) -> bool
    where
        P: Pattern<'a, Searcher: ReverseSearcher<'a>>,
    { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn find<'a, P: Pattern<'a>>(&'a self, pat: P) -> Option<usize> { loop { } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn rfind<'a, P>(&'a self, pat: P) -> Option<usize>
    where
        P: Pattern<'a, Searcher: ReverseSearcher<'a>>,
    { loop { } }

}

impl_fn_for_zst! {
    #[derive(Clone)]
    struct CharEscapeDebugContinue impl Fn = |c: char| -> char::EscapeDebug { loop { } };

    #[derive(Clone)]
    struct CharEscapeUnicode impl Fn = |c: char| -> char::EscapeUnicode { loop { } };
    #[derive(Clone)]
    struct CharEscapeDefault impl Fn = |c: char| -> char::EscapeDefault { loop { } };
}

#[stable(feature = "rust1", since = "1.0.0")]
impl AsRef<[u8]> for str {
    #[inline]
    fn as_ref(&self) -> &[u8] { loop { } }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Default for &str {
    fn default() -> Self { loop { } }
}

#[stable(feature = "default_mut_str", since = "1.28.0")]
impl Default for &mut str {
    fn default() -> Self { loop { } }
}

#[stable(feature = "split_whitespace", since = "1.1.0")]
#[derive(Clone, Debug)]
pub struct SplitWhitespace<'a> {
    inner: &'a (),
}

#[stable(feature = "split_ascii_whitespace", since = "1.34.0")]
#[derive(Clone, Debug)]
pub struct SplitAsciiWhitespace<'a> {
    inner: &'a (),
}

impl_fn_for_zst! {
    #[derive(Clone)]
    struct IsWhitespace impl Fn = |c: char| -> bool { loop { } };

    #[derive(Clone)]
    struct IsAsciiWhitespace impl Fn = |byte: &u8| -> bool { loop { } };

    #[derive(Clone)]
    struct IsNotEmpty impl<'a, 'b> Fn = |s: &'a &'b str| -> bool { loop { } };

    #[derive(Clone)]
    struct BytesIsNotEmpty impl<'a, 'b> Fn = |s: &'a &'b [u8]| -> bool { loop { } };

    #[derive(Clone)]
    struct UnsafeBytesToStr impl<'a> Fn = |bytes: &'a [u8]| -> &'a str { loop { } };
}

#[stable(feature = "split_whitespace", since = "1.1.0")]
impl<'a> Iterator for SplitWhitespace<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<&'a str> { loop { } }
}

#[stable(feature = "split_ascii_whitespace", since = "1.34.0")]
impl<'a> Iterator for SplitAsciiWhitespace<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<&'a str> { loop { } }
}

#[derive(Clone)]
#[stable(feature = "encode_utf16", since = "1.8.0")]
pub struct EncodeUtf16<'a> {
    chars: Chars<'a>,
    extra: u16,
}

#[stable(feature = "collection_debug", since = "1.17.0")]
impl fmt::Debug for EncodeUtf16<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[stable(feature = "encode_utf16", since = "1.8.0")]
impl<'a> Iterator for EncodeUtf16<'a> {
    type Item = u16;

    #[inline]
    fn next(&mut self) -> Option<u16> { loop { } }
}

#[stable(feature = "str_escape", since = "1.34.0")]
#[derive(Clone, Debug)]
pub struct EscapeDebug<'a> {
    inner: &'a (),
}

#[stable(feature = "str_escape", since = "1.34.0")]
#[derive(Clone, Debug)]
pub struct EscapeDefault<'a> {
    inner: &'a (),
}

#[stable(feature = "str_escape", since = "1.34.0")]
#[derive(Clone, Debug)]
pub struct EscapeUnicode<'a> {
    inner: &'a (),
}

macro_rules! escape_types_impls {
    ($( $Name: ident ),+) => {$(
        #[stable(feature = "str_escape", since = "1.34.0")]
        impl<'a> fmt::Display for $Name<'a> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
        }

        #[stable(feature = "str_escape", since = "1.34.0")]
        impl<'a> Iterator for $Name<'a> {
            type Item = char;

            #[inline]
            fn next(&mut self) -> Option<char> { loop { } }
        }
    )+}
}

escape_types_impls!(EscapeDebug, EscapeDefault, EscapeUnicode);
