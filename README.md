Repro for https://github.com/rust-lang/rust/issues/64872.

```bash
$ rustup component add rust-src
$ ./prepare_sysroot_src.sh
$ cargo build
```

