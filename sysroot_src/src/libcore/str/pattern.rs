//! The string Pattern API.

#![unstable(feature = "pattern",
            reason = "API not fully fleshed out and ready to be stabilized",
            issue = "27721")]

use crate::fmt;
use crate::usize;


pub trait Pattern<'a>: Sized {
    type Searcher: Searcher<'a>;

    fn into_searcher(self, haystack: &'a str) -> Self::Searcher;

    #[inline]
    fn is_contained_in(self, haystack: &'a str) -> bool { loop { } }

    #[inline]
    fn is_prefix_of(self, haystack: &'a str) -> bool { loop { } }

    #[inline]
    fn is_suffix_of(self, haystack: &'a str) -> bool
        where Self::Searcher: ReverseSearcher<'a>
    { loop { } }
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SearchStep {
    Match(usize, usize),
    Reject(usize, usize),
    Done
}

pub unsafe trait Searcher<'a> {
    fn haystack(&self) -> &'a str;

    fn next(&mut self) -> SearchStep;

    #[inline]
    fn next_match(&mut self) -> Option<(usize, usize)> { loop { } }

    #[inline]
    fn next_reject(&mut self) -> Option<(usize, usize)> { loop { } }
}

pub unsafe trait ReverseSearcher<'a>: Searcher<'a> {
    fn next_back(&mut self) -> SearchStep;

    #[inline]
    fn next_match_back(&mut self) -> Option<(usize, usize)>{ loop { } }

    #[inline]
    fn next_reject_back(&mut self) -> Option<(usize, usize)>{ loop { } }
}

pub trait DoubleEndedSearcher<'a>: ReverseSearcher<'a> {}



#[derive(Clone, Debug)]
pub struct CharSearcher<'a> {
    haystack: &'a str,

    finger: usize,
    finger_back: usize,
    needle: char,

    utf8_size: usize,
    utf8_encoded: [u8; 4],
}

unsafe impl<'a> Searcher<'a> for CharSearcher<'a> {
    #[inline]
    fn haystack(&self) -> &'a str { loop { } }
    #[inline]
    fn next(&mut self) -> SearchStep { loop { } }
    #[inline]
    fn next_match(&mut self) -> Option<(usize, usize)> { loop { } }

}

unsafe impl<'a> ReverseSearcher<'a> for CharSearcher<'a> {
    #[inline]
    fn next_back(&mut self) -> SearchStep { loop { } }
    #[inline]
    fn next_match_back(&mut self) -> Option<(usize, usize)> { loop { } }

}

impl<'a> DoubleEndedSearcher<'a> for CharSearcher<'a> {}

impl<'a> Pattern<'a> for char {
    type Searcher = CharSearcher<'a>;

    #[inline]
    fn into_searcher(self, haystack: &'a str) -> Self::Searcher { loop { } }

    #[inline]
    fn is_contained_in(self, haystack: &'a str) -> bool { loop { } }

    #[inline]
    fn is_prefix_of(self, haystack: &'a str) -> bool { loop { } }

    #[inline]
    fn is_suffix_of(self, haystack: &'a str) -> bool where Self::Searcher: ReverseSearcher<'a>
    { loop { } }
}


#[doc(hidden)]
trait MultiCharEq {
    fn matches(&mut self, c: char) -> bool;
}

impl<F> MultiCharEq for F where F: FnMut(char) -> bool {
    #[inline]
    fn matches(&mut self, c: char) -> bool { loop { } }
}

impl MultiCharEq for &[char] {
    #[inline]
    fn matches(&mut self, c: char) -> bool { loop { } }
}

struct MultiCharEqPattern<C: MultiCharEq>(C);

#[derive(Clone, Debug)]
struct MultiCharEqSearcher<'a, C: MultiCharEq> {
    char_eq: C,
    haystack: &'a str,
    char_indices: super::CharIndices<'a>,
}

impl<'a, C: MultiCharEq> Pattern<'a> for MultiCharEqPattern<C> {
    type Searcher = MultiCharEqSearcher<'a, C>;

    #[inline]
    fn into_searcher(self, haystack: &'a str) -> MultiCharEqSearcher<'a, C> { loop { } }
}

unsafe impl<'a, C: MultiCharEq> Searcher<'a> for MultiCharEqSearcher<'a, C> {
    #[inline]
    fn haystack(&self) -> &'a str { loop { } }

    #[inline]
    fn next(&mut self) -> SearchStep { loop { } }
}

unsafe impl<'a, C: MultiCharEq> ReverseSearcher<'a> for MultiCharEqSearcher<'a, C> {
    #[inline]
    fn next_back(&mut self) -> SearchStep { loop { } }
}

impl<'a, C: MultiCharEq> DoubleEndedSearcher<'a> for MultiCharEqSearcher<'a, C> {}


macro_rules! pattern_methods {
    ($t:ty, $pmap:expr, $smap:expr) => {
        type Searcher = $t;

        #[inline]
        fn into_searcher(self, haystack: &'a str) -> $t { loop { } }

        #[inline]
        fn is_contained_in(self, haystack: &'a str) -> bool { loop { } }

        #[inline]
        fn is_prefix_of(self, haystack: &'a str) -> bool { loop { } }

        #[inline]
        fn is_suffix_of(self, haystack: &'a str) -> bool
            where $t: ReverseSearcher<'a>
        { loop { } }
    }
}

macro_rules! searcher_methods {
    (forward) => {
        #[inline]
        fn haystack(&self) -> &'a str { loop { } }
        #[inline]
        fn next(&mut self) -> SearchStep { loop { } }
        #[inline]
        fn next_match(&mut self) -> Option<(usize, usize)> { loop { } }
        #[inline]
        fn next_reject(&mut self) -> Option<(usize, usize)> { loop { } }
    };
    (reverse) => {
        #[inline]
        fn next_back(&mut self) -> SearchStep { loop { } }
        #[inline]
        fn next_match_back(&mut self) -> Option<(usize, usize)> { loop { } }
        #[inline]
        fn next_reject_back(&mut self) -> Option<(usize, usize)> { loop { } }
    }
}



#[derive(Clone, Debug)]
pub struct CharSliceSearcher<'a, 'b>(<MultiCharEqPattern<&'b [char]> as Pattern<'a>>::Searcher);

unsafe impl<'a, 'b> Searcher<'a> for CharSliceSearcher<'a, 'b> {
    searcher_methods!(forward);
}

unsafe impl<'a, 'b> ReverseSearcher<'a> for CharSliceSearcher<'a, 'b> {
    searcher_methods!(reverse);
}

impl<'a, 'b> DoubleEndedSearcher<'a> for CharSliceSearcher<'a, 'b> {}

impl<'a, 'b> Pattern<'a> for &'b [char] {
    pattern_methods!(CharSliceSearcher<'a, 'b>, MultiCharEqPattern, CharSliceSearcher);
}


#[derive(Clone)]
pub struct CharPredicateSearcher<'a, F>(<MultiCharEqPattern<F> as Pattern<'a>>::Searcher)
    where F: FnMut(char) -> bool;

impl<F> fmt::Debug for CharPredicateSearcher<'_, F>
    where F: FnMut(char) -> bool
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}
unsafe impl<'a, F> Searcher<'a> for CharPredicateSearcher<'a, F>
    where F: FnMut(char) -> bool
{
    searcher_methods!(forward);
}

unsafe impl<'a, F> ReverseSearcher<'a> for CharPredicateSearcher<'a, F>
    where F: FnMut(char) -> bool
{
    searcher_methods!(reverse);
}

impl<'a, F> DoubleEndedSearcher<'a> for CharPredicateSearcher<'a, F>
    where F: FnMut(char) -> bool {}

impl<'a, F> Pattern<'a> for F where F: FnMut(char) -> bool {
    pattern_methods!(CharPredicateSearcher<'a, F>, MultiCharEqPattern, CharPredicateSearcher);
}


impl<'a, 'b, 'c> Pattern<'a> for &'c &'b str {
    pattern_methods!(StrSearcher<'a, 'b>, |&s| s, |s| s);
}


impl<'a, 'b> Pattern<'a> for &'b str {
    type Searcher = StrSearcher<'a, 'b>;

    #[inline]
    fn into_searcher(self, haystack: &'a str) -> StrSearcher<'a, 'b> { loop { } }

    #[inline]
    fn is_prefix_of(self, haystack: &'a str) -> bool { loop { } }

    #[inline]
    fn is_suffix_of(self, haystack: &'a str) -> bool { loop { } }
}



#[derive(Clone, Debug)]
pub struct StrSearcher<'a, 'b> {
    haystack: &'a str,
    needle: &'b str,

    searcher: StrSearcherImpl,
}

#[derive(Clone, Debug)]
enum StrSearcherImpl {
    Empty(EmptyNeedle),
    TwoWay(TwoWaySearcher),
}

#[derive(Clone, Debug)]
struct EmptyNeedle {
    position: usize,
    end: usize,
    is_match_fw: bool,
    is_match_bw: bool,
}

impl<'a, 'b> StrSearcher<'a, 'b> {
    fn new(haystack: &'a str, needle: &'b str) -> StrSearcher<'a, 'b> { loop { } }
}

unsafe impl<'a, 'b> Searcher<'a> for StrSearcher<'a, 'b> {
    #[inline]
    fn haystack(&self) -> &'a str { loop { } }

    #[inline]
    fn next(&mut self) -> SearchStep { loop { } }

    #[inline]
    fn next_match(&mut self) -> Option<(usize, usize)> { loop { } }
}

unsafe impl<'a, 'b> ReverseSearcher<'a> for StrSearcher<'a, 'b> {
    #[inline]
    fn next_back(&mut self) -> SearchStep { loop { } }

    #[inline]
    fn next_match_back(&mut self) -> Option<(usize, usize)> { loop { } }
}

#[derive(Clone, Debug)]
struct TwoWaySearcher {
    crit_pos: usize,
    crit_pos_back: usize,
    period: usize,
    byteset: u64,

    position: usize,
    end: usize,
    memory: usize,
    memory_back: usize,
}

/*
    This is the Two-Way search algorithm, which was introduced in the paper:
    Crochemore, M., Perrin, D., 1991, Two-way string-matching, Journal of the ACM 38(3):651-675.

    Here's some background information.

    A *word* is a string of symbols. The *length* of a word should be a familiar
    notion, and here we denote it for any word x by |x|.
    (We also allow for the possibility of the *empty word*, a word of length zero).

    If x is any non-empty word, then an integer p with 0 < p <= |x| is said to be a
    *period* for x iff for all i with 0 <= i <= |x| - p - 1, we have x[i] == x[i+p].
    For example, both 1 and 2 are periods for the string "aa". As another example,
    the only period of the string "abcd" is 4.

    We denote by period(x) the *smallest* period of x (provided that x is non-empty).
    This is always well-defined since every non-empty word x has at least one period,
    |x|. We sometimes call this *the period* of x.

    If u, v and x are words such that x = uv, where uv is the concatenation of u and
    v, then we say that (u, v) is a *factorization* of x.

    Let (u, v) be a factorization for a word x. Then if w is a non-empty word such
    that both of the following hold

      - either w is a suffix of u or u is a suffix of w
      - either w is a prefix of v or v is a prefix of w

    then w is said to be a *repetition* for the factorization (u, v).

    Just to unpack this, there are four possibilities here. Let w = "abc". Then we
    might have:

      - w is a suffix of u and w is a prefix of v. ex: ("lolabc", "abcde")
      - w is a suffix of u and v is a prefix of w. ex: ("lolabc", "ab")
      - u is a suffix of w and w is a prefix of v. ex: ("bc", "abchi")
      - u is a suffix of w and v is a prefix of w. ex: ("bc", "a")

    Note that the word vu is a repetition for any factorization (u,v) of x = uv,
    so every factorization has at least one repetition.

    If x is a string and (u, v) is a factorization for x, then a *local period* for
    (u, v) is an integer r such that there is some word w such that |w| = r and w is
    a repetition for (u, v).

    We denote by local_period(u, v) the smallest local period of (u, v). We sometimes
    call this *the local period* of (u, v). Provided that x = uv is non-empty, this
    is well-defined (because each non-empty word has at least one factorization, as
    noted above).

    It can be proven that the following is an equivalent definition of a local period
    for a factorization (u, v): any positive integer r such that x[i] == x[i+r] for
    all i such that |u| - r <= i <= |u| - 1 and such that both x[i] and x[i+r] are
    defined. (i.e., i > 0 and i + r < |x|).

    Using the above reformulation, it is easy to prove that

        1 <= local_period(u, v) <= period(uv)

    A factorization (u, v) of x such that local_period(u,v) = period(x) is called a
    *critical factorization*.

    The algorithm hinges on the following theorem, which is stated without proof:

    **Critical Factorization Theorem** Any word x has at least one critical
    factorization (u, v) such that |u| < period(x).

    The purpose of maximal_suffix is to find such a critical factorization.

    If the period is short, compute another factorization x = u' v' to use
    for reverse search, chosen instead so that |v'| < period(x).

*/
impl TwoWaySearcher {
    fn new(needle: &[u8], end: usize) -> TwoWaySearcher { loop { } }

    #[inline]
    fn byteset_create(bytes: &[u8]) -> u64 { loop { } }

    #[inline]
    fn byteset_contains(&self, byte: u8) -> bool { loop { } }

    #[inline]
    fn next<S>(&mut self, haystack: &[u8], needle: &[u8], long_period: bool)
        -> S::Output
        where S: TwoWayStrategy
    { loop { } }

    #[inline]
    fn next_back<S>(&mut self, haystack: &[u8], needle: &[u8], long_period: bool)
        -> S::Output
        where S: TwoWayStrategy
    { loop { } }

    #[inline]
    fn maximal_suffix(arr: &[u8], order_greater: bool) -> (usize, usize) { loop { } }

    fn reverse_maximal_suffix(arr: &[u8], known_period: usize,
                              order_greater: bool) -> usize
    { loop { } }
}

trait TwoWayStrategy {
    type Output;
    fn use_early_reject() -> bool;
    fn rejecting(a: usize, b: usize) -> Self::Output;
    fn matching(a: usize, b: usize) -> Self::Output;
}

enum MatchOnly { }

impl TwoWayStrategy for MatchOnly {
    type Output = Option<(usize, usize)>;

    #[inline]
    fn use_early_reject() -> bool { loop { } }
    #[inline]
    fn rejecting(_a: usize, _b: usize) -> Self::Output { loop { } }
    #[inline]
    fn matching(a: usize, b: usize) -> Self::Output { loop { } }
}

enum RejectAndMatch { }

impl TwoWayStrategy for RejectAndMatch {
    type Output = SearchStep;

    #[inline]
    fn use_early_reject() -> bool { loop { } }
    #[inline]
    fn rejecting(a: usize, b: usize) -> Self::Output { loop { } }
    #[inline]
    fn matching(a: usize, b: usize) -> Self::Output { loop { } }
}
