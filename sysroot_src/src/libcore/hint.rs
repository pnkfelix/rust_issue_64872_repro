#![stable(feature = "core_hint", since = "1.27.0")]

#[stable(feature = "unreachable", since = "1.27.0")]
pub unsafe fn unreachable_unchecked() -> ! { loop { } }

#[unstable(feature = "renamed_spin_loop", issue = "55002")]
pub fn spin_loop() { loop { } }

#[unstable(feature = "test", issue = "50297")]
pub fn black_box<T>(_dummy: T) -> T { loop { } }
