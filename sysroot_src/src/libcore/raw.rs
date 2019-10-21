#![allow(missing_docs)]
#![unstable(feature = "raw", issue = "27751")]


#[repr(C)]
#[derive(Copy, Clone)]
#[allow(missing_debug_implementations)]
pub struct TraitObject {
    pub data: *mut (),
    pub vtable: *mut (),
}
