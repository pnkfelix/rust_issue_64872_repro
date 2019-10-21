use crate::fmt::{self};

/// Lossy UTF-8 string.
#[unstable(feature = "str_internals", issue = "0")]
pub struct Utf8Lossy {
    bytes: [u8]
}

impl Utf8Lossy {
    pub fn from_str(s: &str) -> &Utf8Lossy { loop { } }

    pub fn from_bytes(bytes: &[u8]) -> &Utf8Lossy { loop { } }

    pub fn chunks(&self) -> Utf8LossyChunksIter<'_> { loop { } }
}


/// Iterator over lossy UTF-8 string
#[unstable(feature = "str_internals", issue = "0")]
#[allow(missing_debug_implementations)]
pub struct Utf8LossyChunksIter<'a> {
    source: &'a [u8],
}

#[unstable(feature = "str_internals", issue = "0")]
#[derive(PartialEq, Eq, Debug)]
pub struct Utf8LossyChunk<'a> {
    /// Sequence of valid chars.
    /// Can be empty between broken UTF-8 chars.
    pub valid: &'a str,
    /// Single broken char, empty if none.
    /// Empty iff iterator item is last.
    pub broken: &'a [u8],
}

impl<'a> Iterator for Utf8LossyChunksIter<'a> {
    type Item = Utf8LossyChunk<'a>;

    fn next(&mut self) -> Option<Utf8LossyChunk<'a>> { loop { } }
}


impl fmt::Display for Utf8Lossy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

impl fmt::Debug for Utf8Lossy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}
