//! impl bool {}

#[lang = "bool"]
impl bool {
    #[unstable(feature = "bool_to_option", issue = "64260")]
    #[inline]
    pub fn then<T>(self, t: T) -> Option<T> { loop { } }

    #[unstable(feature = "bool_to_option", issue = "64260")]
    #[inline]
    pub fn then_with<T, F: FnOnce() -> T>(self, f: F) -> Option<T> { loop { } }
}
