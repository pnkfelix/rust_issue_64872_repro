enum Name { _Short(u32) }

impl std::fmt::Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Name::_Short(ref u) => {
                let mut debug_trait_builder: std::fmt::DebugTuple = f.debug_tuple("_Short");
                debug_trait_builder.field(&u);
                loop { }
            }
        }
    }
}
