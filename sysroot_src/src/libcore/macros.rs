#[macro_export]
#[allow_internal_unstable(core_panic)]
#[stable(feature = "core", since = "1.6.0")]
macro_rules! panic {
    () => (
        $crate::panic!("explicit panic")
    );
    ($msg:expr) => ({
        $crate::panicking::panic(&($msg, $crate::file!(), $crate::line!(), $crate::column!()))
    });
    ($msg:expr,) => (
        $crate::panic!($msg)
    );
    ($fmt:expr, $($arg:tt)+) => ({
        $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+),
                                     &($crate::file!(), $crate::line!(), $crate::column!()))
    });
}

#[macro_export]
#[stable(feature = "rust1", since = "1.0.0")]
macro_rules! assert_eq {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    panic!(r#"assertion failed: `(left == right)`
  left: `{:?}`,
 right: `{:?}`"#, &*left_val, &*right_val)
                }
            }
        }
    });
    ($left:expr, $right:expr,) => ({
        $crate::assert_eq!($left, $right)
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    panic!(r#"assertion failed: `(left == right)`
  left: `{:?}`,
 right: `{:?}`: {}"#, &*left_val, &*right_val,
                           $crate::format_args!($($arg)+))
                }
            }
        }
    });
}

#[macro_export]
#[stable(feature = "assert_ne", since = "1.13.0")]
macro_rules! assert_ne {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                    panic!(r#"assertion failed: `(left != right)`
  left: `{:?}`,
 right: `{:?}`"#, &*left_val, &*right_val)
                }
            }
        }
    });
    ($left:expr, $right:expr,) => {
        $crate::assert_ne!($left, $right)
    };
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                    panic!(r#"assertion failed: `(left != right)`
  left: `{:?}`,
 right: `{:?}`: {}"#, &*left_val, &*right_val,
                           $crate::format_args!($($arg)+))
                }
            }
        }
    });
}

#[macro_export]
#[stable(feature = "rust1", since = "1.0.0")]
macro_rules! debug_assert {
    ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { $crate::assert!($($arg)*); })
}

#[macro_export]
#[stable(feature = "rust1", since = "1.0.0")]
macro_rules! debug_assert_eq {
    ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { $crate::assert_eq!($($arg)*); })
}

#[macro_export]
#[stable(feature = "assert_ne", since = "1.13.0")]
macro_rules! debug_assert_ne {
    ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { $crate::assert_ne!($($arg)*); })
}

#[macro_export]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_deprecated(since = "1.39.0", reason = "use the `?` operator instead")]
#[doc(alias = "?")]
macro_rules! r#try {
    ($expr:expr) => (match $expr {
        $crate::result::Result::Ok(val) => val,
        $crate::result::Result::Err(err) => {
            return $crate::result::Result::Err($crate::convert::From::from(err))
        }
    });
    ($expr:expr,) => ($crate::r#try!($expr));
}

#[macro_export]
#[stable(feature = "rust1", since = "1.0.0")]
macro_rules! write {
    ($dst:expr, $($arg:tt)*) => ($dst.write_fmt($crate::format_args!($($arg)*)))
}

#[macro_export]
#[stable(feature = "rust1", since = "1.0.0")]
#[allow_internal_unstable(format_args_nl)]
macro_rules! writeln {
    ($dst:expr) => (
        $crate::write!($dst, "\n")
    );
    ($dst:expr,) => (
        $crate::writeln!($dst)
    );
    ($dst:expr, $($arg:tt)*) => (
        $dst.write_fmt($crate::format_args_nl!($($arg)*))
    );
}

#[macro_export]
#[stable(feature = "rust1", since = "1.0.0")]
macro_rules! unreachable {
    () => ({
        panic!("internal error: entered unreachable code")
    });
    ($msg:expr) => ({
        $crate::unreachable!("{}", $msg)
    });
    ($msg:expr,) => ({
        $crate::unreachable!($msg)
    });
    ($fmt:expr, $($arg:tt)*) => ({
        panic!($crate::concat!("internal error: entered unreachable code: ", $fmt), $($arg)*)
    });
}

#[macro_export]
#[stable(feature = "rust1", since = "1.0.0")]
macro_rules! unimplemented {
    () => (panic!("not yet implemented"));
    ($($arg:tt)+) => (panic!("not yet implemented: {}", $crate::format_args!($($arg)+)));
}

#[macro_export]
#[stable(feature = "todo_macro", since = "1.39.0")]
macro_rules! todo {
    () => (panic!("not yet implemented"));
    ($($arg:tt)+) => (panic!("not yet implemented: {}", $crate::format_args!($($arg)+)));
}

pub(crate) mod builtin {

    #[stable(feature = "compile_error_macro", since = "1.20.0")]
    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! compile_error {
        ($msg:expr) => ({ /* compiler built-in */ });
        ($msg:expr,) => ({ /* compiler built-in */ })
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[allow_internal_unstable(fmt_internals)]
    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! format_args {
        ($fmt:expr) => ({ /* compiler built-in */ });
        ($fmt:expr, $($args:tt)*) => ({ /* compiler built-in */ })
    }

    #[unstable(feature = "format_args_nl", issue = "0",
               reason = "`format_args_nl` is only for internal \
                         language use and is subject to change")]
    #[allow_internal_unstable(fmt_internals)]
    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! format_args_nl {
        ($fmt:expr) => ({ /* compiler built-in */ });
        ($fmt:expr, $($args:tt)*) => ({ /* compiler built-in */ })
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! env {
        ($name:expr) => ({ /* compiler built-in */ });
        ($name:expr,) => ({ /* compiler built-in */ })
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! option_env {
        ($name:expr) => ({ /* compiler built-in */ });
        ($name:expr,) => ({ /* compiler built-in */ })
    }

    #[unstable(feature = "concat_idents", issue = "29599",
               reason = "`concat_idents` is not stable enough for use and is subject to change")]
    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! concat_idents {
        ($($e:ident),+) => ({ /* compiler built-in */ });
        ($($e:ident,)+) => ({ /* compiler built-in */ })
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! concat {
        ($($e:expr),*) => ({ /* compiler built-in */ });
        ($($e:expr,)*) => ({ /* compiler built-in */ })
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! line { () => { /* compiler built-in */ } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! column { () => { /* compiler built-in */ } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! file { () => { /* compiler built-in */ } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! stringify { ($($t:tt)*) => { /* compiler built-in */ } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! include_str {
        ($file:expr) => ({ /* compiler built-in */ });
        ($file:expr,) => ({ /* compiler built-in */ })
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! include_bytes {
        ($file:expr) => ({ /* compiler built-in */ });
        ($file:expr,) => ({ /* compiler built-in */ })
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! module_path { () => { /* compiler built-in */ } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! cfg { ($($cfg:tt)*) => { /* compiler built-in */ } }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! include {
        ($file:expr) => ({ /* compiler built-in */ });
        ($file:expr,) => ({ /* compiler built-in */ })
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! assert {
        ($cond:expr) => ({ /* compiler built-in */ });
        ($cond:expr,) => ({ /* compiler built-in */ });
        ($cond:expr, $($arg:tt)+) => ({ /* compiler built-in */ })
    }

    #[unstable(feature = "asm", issue = "29722",
               reason = "inline assembly is not stable enough for use and is subject to change")]
    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! asm { ("assembly template"
                        : $("output"(operand),)*
                        : $("input"(operand),)*
                        : $("clobbers",)*
                        : $("options",)*) => { /* compiler built-in */ } }

    #[unstable(feature = "global_asm", issue = "35119",
               reason = "`global_asm!` is not stable enough for use and is subject to change")]
    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! global_asm { ("assembly") => { /* compiler built-in */ } }

    #[unstable(feature = "log_syntax", issue = "29598",
               reason = "`log_syntax!` is not stable enough for use and is subject to change")]
    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! log_syntax { ($($arg:tt)*) => { /* compiler built-in */ } }

    #[unstable(feature = "trace_macros", issue = "29598",
               reason = "`trace_macros` is not stable enough for use and is subject to change")]
    #[rustc_builtin_macro]
    #[macro_export]
    macro_rules! trace_macros {
        (true) => ({ /* compiler built-in */ });
        (false) => ({ /* compiler built-in */ })
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[allow_internal_unstable(test, rustc_attrs)]
    #[rustc_builtin_macro]
    pub macro test($item:item) { /* compiler built-in */ }

    #[unstable(soft, feature = "test", issue = "50297",
               reason = "`bench` is a part of custom test frameworks which are unstable")]
    #[allow_internal_unstable(test, rustc_attrs)]
    #[rustc_builtin_macro]
    pub macro bench($item:item) { /* compiler built-in */ }

    #[unstable(feature = "custom_test_frameworks", issue = "50297",
               reason = "custom test frameworks are an unstable feature")]
    #[allow_internal_unstable(test, rustc_attrs)]
    #[rustc_builtin_macro]
    pub macro test_case($item:item) { /* compiler built-in */ }

    #[stable(feature = "global_allocator", since = "1.28.0")]
    #[allow_internal_unstable(rustc_attrs)]
    #[rustc_builtin_macro]
    pub macro global_allocator($item:item) { /* compiler built-in */ }

    #[rustc_builtin_macro]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[allow_internal_unstable(core_intrinsics, libstd_sys_internals)]
    pub macro RustcDecodable($item:item) { /* compiler built-in */ }

    #[rustc_builtin_macro]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[allow_internal_unstable(core_intrinsics)]
    pub macro RustcEncodable($item:item) { /* compiler built-in */ }
}
