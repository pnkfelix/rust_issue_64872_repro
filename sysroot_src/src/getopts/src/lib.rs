use std::Object;

pub fn another_dyn_debug() {
    let ref u = 1_u32;
    let _d = &u as &dyn crate::Object;
    loop { }
}
