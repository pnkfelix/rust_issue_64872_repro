To reproduce the bug (rust-lang/rust#64872), just do `cargo build` in
the root of the repository.

When I do this, I observe a linker failure of the form:

```
   Compiling test v0.0.0 (/Users/felixklock/Dev/Mozilla/issue63438/rust_issue_64872_repro/sysroot_src/src/libtest)
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-m64" "-L" "/Users/felixklock/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib" "/Users/felixklock/Dev/Mozilla/issue63438/rust_issue_64872_repro/target/debug/deps/test.4ovhqzay5djkkv45.rcgu.o" "-o" "/Users/felixklock/Dev/Mozilla/issue63438/rust_issue_64872_repro/target/debug/deps/libtest.dylib" "-Wl,-exported_symbols_list,/var/folders/dp/_9prm0yx5bv5s1wz3k0czhl80000gn/T/rustcreOMNj/list" "/Users/felixklock/Dev/Mozilla/issue63438/rust_issue_64872_repro/target/debug/deps/test.4efq6d1me4ylpya9.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/felixklock/Dev/Mozilla/issue63438/rust_issue_64872_repro/target/debug/deps" "-L" "/Users/felixklock/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib" "-Wl,-force_load" "-Wl,/var/folders/dp/_9prm0yx5bv5s1wz3k0czhl80000gn/T/rustcreOMNj/libgetopts-f6e381d53660d1bc.rlib" "-L" "/Users/felixklock/Dev/Mozilla/issue63438/rust_issue_64872_repro/target/debug/deps" "-lstd" "/var/folders/dp/_9prm0yx5bv5s1wz3k0czhl80000gn/T/rustcreOMNj/libcompiler_builtins-cb6e892db602b578.rlib" "-lSystem" "-lresolv" "-dynamiclib" "-Wl,-dylib"
  = note: Undefined symbols for architecture x86_64:
            "core::Object::method::h3c307b2b614e132d", referenced from:
                l_anon.5a5abdbd2936acfd9b076cabe585352c.1 in libgetopts-f6e381d53660d1bc.rlib(getopts-f6e381d53660d1bc.2ctdgr2di66orgpi.rcgu.o)
          ld: symbol(s) not found for architecture x86_64
          clang: error: linker command failed with exit code 1 (use -v to see invocation)
```

This linker failure is arising due to a mix of:

1. the `&u as &dyn crate::Object` in `sysroot_src/src/libcore/lib.rs`:

```rust
pub trait Object { fn method(&self) { } }

impl Object for u32 { }
impl Object for () { }
impl<T> Object for &T { }

pub fn unused() {
    let ref u = 0_u32;
    let _d = &u as &dyn crate::Object;
    loop { }
}
```

and

2. the `&u as &dyn crate::Object` in `sysroot_src/src/getopts/lib.rs`:

```rust
use std::Object;

pub fn another_dyn_debug() {
    let ref u = 1_u32;
    let _d = &u as &dyn crate::Object;
    loop { }
}
```

If you change either function so that each `u` has differing types
(e.g. do `let ref u = ();` in either one but not both), then the code
will link successfully.
