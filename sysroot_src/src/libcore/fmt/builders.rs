use crate::fmt;

struct PadAdapter<'buf, 'state> {
    buf: &'buf mut (dyn fmt::Write + 'buf),
    state: &'state mut PadAdapterState,
}

struct PadAdapterState {
    on_newline: bool,
}

impl Default for PadAdapterState {
    fn default() -> Self { loop { } }
}

impl<'buf, 'state> PadAdapter<'buf, 'state> {
    fn wrap<'slot, 'fmt: 'buf+'slot>(fmt: &'fmt mut fmt::Formatter<'_>,
                                     slot: &'slot mut Option<Self>,
                                     state: &'state mut PadAdapterState) -> fmt::Formatter<'slot> { loop { } }
}

impl fmt::Write for PadAdapter<'_, '_> {
    fn write_str(&mut self, mut s: &str) -> fmt::Result { loop { } }
}

#[must_use = "must eventually call `finish()` on Debug builders"]
#[allow(missing_debug_implementations)]
#[stable(feature = "debug_builders", since = "1.2.0")]
pub struct DebugStruct<'a, 'b: 'a> {
    fmt: &'a mut fmt::Formatter<'b>,
    result: fmt::Result,
    has_fields: bool,
}

pub(super) fn debug_struct_new<'a, 'b>(fmt: &'a mut fmt::Formatter<'b>,
                                name: &str)
                                       -> DebugStruct<'a, 'b> { loop { } }

impl<'a, 'b: 'a> DebugStruct<'a, 'b> {
    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn field(&mut self, name: &str, value: &dyn fmt::Debug) -> &mut DebugStruct<'a, 'b> { loop { } }

    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn finish(&mut self) -> fmt::Result { loop { } }

    fn is_pretty(&self) -> bool { loop { } }
}

#[must_use = "must eventually call `finish()` on Debug builders"]
#[allow(missing_debug_implementations)]
#[stable(feature = "debug_builders", since = "1.2.0")]
pub struct DebugTuple<'a, 'b: 'a> {
    fmt: &'a mut fmt::Formatter<'b>,
    result: fmt::Result,
    fields: usize,
    empty_name: bool,
}

impl<'a, 'b: 'a> DebugTuple<'a, 'b> {
    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn field(&mut self, _value: &dyn fmt::Debug) -> &mut DebugTuple<'a, 'b> { loop { } }

    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn finish(&mut self) -> fmt::Result { loop { } }

    fn is_pretty(&self) -> bool { loop { } }
}

struct DebugInner<'a, 'b: 'a> {
    fmt: &'a mut fmt::Formatter<'b>,
    result: fmt::Result,
    has_fields: bool,
}

impl<'a, 'b: 'a> DebugInner<'a, 'b> {
    fn entry(&mut self, entry: &dyn fmt::Debug) { loop { } }

    fn is_pretty(&self) -> bool { loop { } }
}

#[must_use = "must eventually call `finish()` on Debug builders"]
#[allow(missing_debug_implementations)]
#[stable(feature = "debug_builders", since = "1.2.0")]
pub struct DebugSet<'a, 'b: 'a> {
    inner: DebugInner<'a, 'b>,
}

pub(super) fn debug_set_new<'a, 'b>(fmt: &'a mut fmt::Formatter<'b>) -> DebugSet<'a, 'b> { loop { } }

impl<'a, 'b: 'a> DebugSet<'a, 'b> {
    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn entry(&mut self, entry: &dyn fmt::Debug) -> &mut DebugSet<'a, 'b> { loop { } }

    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn entries<D, I>(&mut self, entries: I) -> &mut DebugSet<'a, 'b>
        where D: fmt::Debug,
              I: IntoIterator<Item = D>
    { loop { } }

    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn finish(&mut self) -> fmt::Result { loop { } }
}

#[must_use = "must eventually call `finish()` on Debug builders"]
#[allow(missing_debug_implementations)]
#[stable(feature = "debug_builders", since = "1.2.0")]
pub struct DebugList<'a, 'b: 'a> {
    inner: DebugInner<'a, 'b>,
}

pub(super) fn debug_list_new<'a, 'b>(fmt: &'a mut fmt::Formatter<'b>) -> DebugList<'a, 'b> { loop { } }

impl<'a, 'b: 'a> DebugList<'a, 'b> {
    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn entry(&mut self, entry: &dyn fmt::Debug) -> &mut DebugList<'a, 'b> { loop { } }

    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn entries<D, I>(&mut self, entries: I) -> &mut DebugList<'a, 'b>
        where D: fmt::Debug,
              I: IntoIterator<Item = D>
    { loop { } }

    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn finish(&mut self) -> fmt::Result { loop { } }
}

#[must_use = "must eventually call `finish()` on Debug builders"]
#[allow(missing_debug_implementations)]
#[stable(feature = "debug_builders", since = "1.2.0")]
pub struct DebugMap<'a, 'b: 'a> {
    fmt: &'a mut fmt::Formatter<'b>,
    result: fmt::Result,
    has_fields: bool,
    has_key: bool,
    state: PadAdapterState,
}

pub(super) fn debug_map_new<'a, 'b>(fmt: &'a mut fmt::Formatter<'b>) -> DebugMap<'a, 'b> { loop { } }

impl<'a, 'b: 'a> DebugMap<'a, 'b> {
    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn entry(&mut self, key: &dyn fmt::Debug, value: &dyn fmt::Debug) -> &mut DebugMap<'a, 'b> { loop { } }

    #[unstable(feature = "debug_map_key_value",
               reason = "recently added",
               issue = "62482")]
    pub fn key(&mut self, key: &dyn fmt::Debug) -> &mut DebugMap<'a, 'b> { loop { } }

    #[unstable(feature = "debug_map_key_value",
               reason = "recently added",
               issue = "62482")]
    pub fn value(&mut self, value: &dyn fmt::Debug) -> &mut DebugMap<'a, 'b> { loop { } }

    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn entries<K, V, I>(&mut self, entries: I) -> &mut DebugMap<'a, 'b>
        where K: fmt::Debug,
              V: fmt::Debug,
              I: IntoIterator<Item = (K, V)>
    { loop { } }

    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn finish(&mut self) -> fmt::Result { loop { } }

    fn is_pretty(&self) -> bool { loop { } }
}
