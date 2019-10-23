enum Name { _Short(u32) }

impl std::fmt::Debug for Name {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Name::_Short(ref u) => {
                let _d = &u as &dyn std::fmt::Debug;
                loop { }
            }
        }
    }
}
