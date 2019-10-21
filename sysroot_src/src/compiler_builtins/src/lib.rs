#![cfg_attr(not(stage0), deny(warnings))]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "compiler-builtins", compiler_builtins)]
#![crate_name = "compiler_builtins"]
#![crate_type = "rlib"]
#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
    html_favicon_url = "https://doc.rust-lang.org/favicon.ico",
    html_root_url = "https://doc.rust-lang.org/nightly/",
    html_playground_url = "https://play.rust-lang.org/",
    test(attr(deny(warnings)))
)]
#![feature(asm)]
#![feature(compiler_builtins)]
#![feature(core_intrinsics)]
#![feature(naked_functions)]
#![feature(repr_simd)]
#![feature(abi_unadjusted)]
#![feature(linkage)]
#![feature(lang_items)]
#![allow(unused_features)]
#![no_builtins]
#![cfg_attr(feature = "compiler-builtins", feature(staged_api))]
#![cfg_attr(
    feature = "compiler-builtins",
    unstable(
        feature = "compiler_builtins_lib",
        reason = "Compiler builtins. Will never become stable.",
        issue = "0"
    )
)]
pub mod probestack {
    #![cfg(not(windows))] // Windows already has builtins to do this

    #[naked]
    #[no_mangle]
    pub unsafe extern "C" fn __rust_probestack() { }
}
