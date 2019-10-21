use crate::fmt;
use crate::hash::{Hash, Hasher};

#[doc(alias = "..")]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct RangeFull;

#[stable(feature = "rust1", since = "1.0.0")]
impl fmt::Debug for RangeFull {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[doc(alias = "..")]
#[derive(Clone, PartialEq, Eq, Hash)] // not Copy -- see #27186
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Range<Idx> {
    #[stable(feature = "rust1", since = "1.0.0")]
    pub start: Idx,
    #[stable(feature = "rust1", since = "1.0.0")]
    pub end: Idx,
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<Idx: fmt::Debug> fmt::Debug for Range<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

impl<Idx: PartialOrd<Idx>> Range<Idx> {
    #[stable(feature = "range_contains", since = "1.35.0")]
    pub fn contains<U>(&self, item: &U) -> bool
    where
        Idx: PartialOrd<U>,
        U: ?Sized + PartialOrd<Idx>,
    { loop { } }

    #[unstable(feature = "range_is_empty", reason = "recently added", issue = "48111")]
    pub fn is_empty(&self) -> bool { loop { } }
}

#[doc(alias = "..")]
#[derive(Clone, PartialEq, Eq, Hash)] // not Copy -- see #27186
#[stable(feature = "rust1", since = "1.0.0")]
pub struct RangeFrom<Idx> {
    #[stable(feature = "rust1", since = "1.0.0")]
    pub start: Idx,
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<Idx: fmt::Debug> fmt::Debug for RangeFrom<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

impl<Idx: PartialOrd<Idx>> RangeFrom<Idx> {
    #[stable(feature = "range_contains", since = "1.35.0")]
    pub fn contains<U>(&self, item: &U) -> bool
    where
        Idx: PartialOrd<U>,
        U: ?Sized + PartialOrd<Idx>,
    { loop { } }
}

#[doc(alias = "..")]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct RangeTo<Idx> {
    #[stable(feature = "rust1", since = "1.0.0")]
    pub end: Idx,
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<Idx: fmt::Debug> fmt::Debug for RangeTo<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

impl<Idx: PartialOrd<Idx>> RangeTo<Idx> {
    #[stable(feature = "range_contains", since = "1.35.0")]
    pub fn contains<U>(&self, item: &U) -> bool
    where
        Idx: PartialOrd<U>,
        U: ?Sized + PartialOrd<Idx>,
    { loop { } }
}

#[doc(alias = "..=")]
#[derive(Clone)] // not Copy -- see #27186
#[stable(feature = "inclusive_range", since = "1.26.0")]
pub struct RangeInclusive<Idx> {
    pub(crate) start: Idx,
    pub(crate) end: Idx,
    pub(crate) is_empty: Option<bool>,
}

trait RangeInclusiveEquality: Sized {
    fn canonicalized_is_empty(range: &RangeInclusive<Self>) -> bool;
}

impl<T> RangeInclusiveEquality for T {
    #[inline]
    default fn canonicalized_is_empty(range: &RangeInclusive<Self>) -> bool { loop { } }
}

impl<T: PartialOrd> RangeInclusiveEquality for T {
    #[inline]
    fn canonicalized_is_empty(range: &RangeInclusive<Self>) -> bool { loop { } }
}

#[stable(feature = "inclusive_range", since = "1.26.0")]
impl<Idx: PartialEq> PartialEq for RangeInclusive<Idx> {
    #[inline]
    fn eq(&self, other: &Self) -> bool { loop { } }
}

#[stable(feature = "inclusive_range", since = "1.26.0")]
impl<Idx: Eq> Eq for RangeInclusive<Idx> {}

#[stable(feature = "inclusive_range", since = "1.26.0")]
impl<Idx: Hash> Hash for RangeInclusive<Idx> {
    fn hash<H: Hasher>(&self, state: &mut H) { loop { } }
}

impl<Idx> RangeInclusive<Idx> {
    #[stable(feature = "inclusive_range_methods", since = "1.27.0")]
    #[inline]
    #[rustc_promotable]
    pub const fn new(start: Idx, end: Idx) -> Self {
        Self {
            start,
            end,
            is_empty: None,
        }
    }

    #[stable(feature = "inclusive_range_methods", since = "1.27.0")]
    #[inline]
    pub const fn start(&self) -> &Idx {
        &self.start
    }

    #[stable(feature = "inclusive_range_methods", since = "1.27.0")]
    #[inline]
    pub const fn end(&self) -> &Idx {
        &self.end
    }

    #[stable(feature = "inclusive_range_methods", since = "1.27.0")]
    #[inline]
    pub fn into_inner(self) -> (Idx, Idx) { loop { } }
}

#[stable(feature = "inclusive_range", since = "1.26.0")]
impl<Idx: fmt::Debug> fmt::Debug for RangeInclusive<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

impl<Idx: PartialOrd<Idx>> RangeInclusive<Idx> {
    #[stable(feature = "range_contains", since = "1.35.0")]
    pub fn contains<U>(&self, item: &U) -> bool
    where
        Idx: PartialOrd<U>,
        U: ?Sized + PartialOrd<Idx>,
    { loop { } }

    #[unstable(feature = "range_is_empty", reason = "recently added", issue = "48111")]
    #[inline]
    pub fn is_empty(&self) -> bool { loop { } }

    #[inline]
    pub(crate) fn compute_is_empty(&mut self) { loop { } }
}

#[doc(alias = "..=")]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[stable(feature = "inclusive_range", since = "1.26.0")]
pub struct RangeToInclusive<Idx> {
    #[stable(feature = "inclusive_range", since = "1.26.0")]
    pub end: Idx,
}

#[stable(feature = "inclusive_range", since = "1.26.0")]
impl<Idx: fmt::Debug> fmt::Debug for RangeToInclusive<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

impl<Idx: PartialOrd<Idx>> RangeToInclusive<Idx> {
    #[stable(feature = "range_contains", since = "1.35.0")]
    pub fn contains<U>(&self, item: &U) -> bool
    where
        Idx: PartialOrd<U>,
        U: ?Sized + PartialOrd<Idx>,
    { loop { } }
}


#[stable(feature = "collections_bound", since = "1.17.0")]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Bound<T> {
    #[stable(feature = "collections_bound", since = "1.17.0")]
    Included(#[stable(feature = "collections_bound", since = "1.17.0")] T),
    #[stable(feature = "collections_bound", since = "1.17.0")]
    Excluded(#[stable(feature = "collections_bound", since = "1.17.0")] T),
    #[stable(feature = "collections_bound", since = "1.17.0")]
    Unbounded,
}

impl<T: Clone> Bound<&T> {
    #[unstable(feature = "bound_cloned", issue = "61356")]
    pub fn cloned(self) -> Bound<T> { loop { } }
}

#[stable(feature = "collections_range", since = "1.28.0")]
pub trait RangeBounds<T: ?Sized> {
    #[stable(feature = "collections_range", since = "1.28.0")]
    fn start_bound(&self) -> Bound<&T>;

    #[stable(feature = "collections_range", since = "1.28.0")]
    fn end_bound(&self) -> Bound<&T>;

    #[stable(feature = "range_contains", since = "1.35.0")]
    fn contains<U>(&self, item: &U) -> bool
    where
        T: PartialOrd<U>,
        U: ?Sized + PartialOrd<T>,
    { loop { } }
}

#[stable(feature = "collections_range", since = "1.28.0")]
impl<T: ?Sized> RangeBounds<T> for RangeFull {
    fn start_bound(&self) -> Bound<&T> { loop { } }
    fn end_bound(&self) -> Bound<&T> { loop { } }
}

#[stable(feature = "collections_range", since = "1.28.0")]
impl<T> RangeBounds<T> for RangeFrom<T> {
    fn start_bound(&self) -> Bound<&T> { loop { } }
    fn end_bound(&self) -> Bound<&T> { loop { } }
}

#[stable(feature = "collections_range", since = "1.28.0")]
impl<T> RangeBounds<T> for RangeTo<T> {
    fn start_bound(&self) -> Bound<&T> { loop { } }
    fn end_bound(&self) -> Bound<&T> { loop { } }
}

#[stable(feature = "collections_range", since = "1.28.0")]
impl<T> RangeBounds<T> for Range<T> {
    fn start_bound(&self) -> Bound<&T> { loop { } }
    fn end_bound(&self) -> Bound<&T> { loop { } }
}

#[stable(feature = "collections_range", since = "1.28.0")]
impl<T> RangeBounds<T> for RangeInclusive<T> {
    fn start_bound(&self) -> Bound<&T> { loop { } }
    fn end_bound(&self) -> Bound<&T> { loop { } }
}

#[stable(feature = "collections_range", since = "1.28.0")]
impl<T> RangeBounds<T> for RangeToInclusive<T> {
    fn start_bound(&self) -> Bound<&T> { loop { } }
    fn end_bound(&self) -> Bound<&T> { loop { } }
}

#[stable(feature = "collections_range", since = "1.28.0")]
impl<T> RangeBounds<T> for (Bound<T>, Bound<T>) {
    fn start_bound(&self) -> Bound<&T> { loop { } }

    fn end_bound(&self) -> Bound<&T> { loop { } }
}

#[stable(feature = "collections_range", since = "1.28.0")]
impl<'a, T: ?Sized + 'a> RangeBounds<T> for (Bound<&'a T>, Bound<&'a T>) {
    fn start_bound(&self) -> Bound<&T> { loop { } }

    fn end_bound(&self) -> Bound<&T> { loop { } }
}

#[stable(feature = "collections_range", since = "1.28.0")]
impl<T> RangeBounds<T> for RangeFrom<&T> {
    fn start_bound(&self) -> Bound<&T> { loop { } }
    fn end_bound(&self) -> Bound<&T> { loop { } }
}

#[stable(feature = "collections_range", since = "1.28.0")]
impl<T> RangeBounds<T> for RangeTo<&T> {
    fn start_bound(&self) -> Bound<&T> { loop { } }
    fn end_bound(&self) -> Bound<&T> { loop { } }
}

#[stable(feature = "collections_range", since = "1.28.0")]
impl<T> RangeBounds<T> for Range<&T> {
    fn start_bound(&self) -> Bound<&T> { loop { } }
    fn end_bound(&self) -> Bound<&T> { loop { } }
}

#[stable(feature = "collections_range", since = "1.28.0")]
impl<T> RangeBounds<T> for RangeInclusive<&T> {
    fn start_bound(&self) -> Bound<&T> { loop { } }
    fn end_bound(&self) -> Bound<&T> { loop { } }
}

#[stable(feature = "collections_range", since = "1.28.0")]
impl<T> RangeBounds<T> for RangeToInclusive<&T> {
    fn start_bound(&self) -> Bound<&T> { loop { } }
    fn end_bound(&self) -> Bound<&T> { loop { } }
}
