#![no_std]
#![compiler_builtins]
#![crate_name = "compiler_builtins"]
#![crate_type = "rlib"]
#![feature(compiler_builtins)]
#![feature(naked_functions)]
pub mod probestack {
    #[naked]
    #[no_mangle]
    pub unsafe extern "C" fn __rust_probestack() { }
}
