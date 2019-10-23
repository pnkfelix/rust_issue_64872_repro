struct Name;

impl std::fmt::Debug for Name {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ref u = 1_u32;
        let _d = &u as &dyn std::fmt::Debug;
        loop { }
    }
}
