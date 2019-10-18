//! Implementation of various bits and pieces of the `panic!` macro and
//! associated runtime pieces.
//!
//! Specifically, this module contains the implementation of:
//!
//! * Panic hooks
//! * Executing a panic up to doing the actual implementation
//! * Shims around "try"

use core::panic::{BoxMeUp, PanicInfo, Location};

use crate::any::Any;
use crate::fmt;
use crate::intrinsics;
use crate::mem;
use crate::ptr;
use crate::raw;
use crate::sync::atomic::{AtomicBool, Ordering};
use crate::sys::stdio::panic_output;
use crate::sys_common::rwlock::RWLock;
use crate::sys_common::{thread_info, util};
use crate::sys_common::backtrace::{self, RustBacktrace};
use crate::thread;

#[cfg(not(test))]
use crate::io::set_panic;
// make sure to use the stderr output configured
// by libtest in the real copy of std
#[cfg(test)]
use realstd::io::set_panic;

// Binary interface to the panic runtime that the standard library depends on.
//
// The standard library is tagged with `#![needs_panic_runtime]` (introduced in
// RFC 1513) to indicate that it requires some other crate tagged with
// `#![panic_runtime]` to exist somewhere. Each panic runtime is intended to
// implement these symbols (with the same signatures) so we can get matched up
// to them.
//
// One day this may look a little less ad-hoc with the compiler helping out to
// hook up these functions, but it is not this day!
#[allow(improper_ctypes)]
extern {
    fn __rust_maybe_catch_panic(f: fn(*mut u8),
                                data: *mut u8,
                                data_ptr: *mut usize,
                                vtable_ptr: *mut usize) -> u32;
    #[unwind(allowed)]
    fn __rust_start_panic(payload: usize) -> u32;
}

#[derive(Copy, Clone)]
enum Hook {
    Default,
    Custom(*mut (dyn Fn(&PanicInfo<'_>) + 'static + Sync + Send)),
}

static HOOK_LOCK: RWLock = RWLock::new();
static mut HOOK: Hook = Hook::Default;

#[stable(feature = "panic_hooks", since = "1.10.0")]
pub fn set_hook(hook: Box<dyn Fn(&PanicInfo<'_>) + 'static + Sync + Send>) {
    if thread::panicking() {
        panic!("cannot modify the panic hook from a panicking thread");
    }

    unsafe {
        HOOK_LOCK.write();
        let old_hook = HOOK;
        HOOK = Hook::Custom(Box::into_raw(hook));
        HOOK_LOCK.write_unlock();

        if let Hook::Custom(ptr) = old_hook {
            #[allow(unused_must_use)] {
                Box::from_raw(ptr);
            }
        }
    }
}

#[stable(feature = "panic_hooks", since = "1.10.0")]
pub fn take_hook() -> Box<dyn Fn(&PanicInfo<'_>) + 'static + Sync + Send> {
    if thread::panicking() {
        panic!("cannot modify the panic hook from a panicking thread");
    }

    unsafe {
        HOOK_LOCK.write();
        let hook = HOOK;
        HOOK = Hook::Default;
        HOOK_LOCK.write_unlock();

        match hook {
            Hook::Default => Box::new(default_hook),
            Hook::Custom(ptr) => Box::from_raw(ptr),
        }
    }
}

fn default_hook(info: &PanicInfo<'_>) {
    // If this is a double panic, make sure that we print a backtrace
    // for this panic. Otherwise only print it if logging is enabled.
    let backtrace_env = if update_panic_count(0) >= 2 {
        RustBacktrace::Print(backtrace_rs::PrintFmt::Full)
    } else {
        backtrace::rust_backtrace_env()
    };

    // The current implementation always returns `Some`.
    let location = info.location().unwrap();

    let msg = match info.payload().downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match info.payload().downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "Box<Any>",
        }
    };
    let thread = thread_info::current_thread();
    let name = thread.as_ref().and_then(|t| t.name()).unwrap_or("<unnamed>");

    let write = |err: &mut dyn crate::io::Write| {
        let _ = writeln!(err, "thread '{}' panicked at '{}', {}",
                         name, msg, location);

        static FIRST_PANIC: AtomicBool = AtomicBool::new(true);

        match backtrace_env {
            RustBacktrace::Print(format) => drop(backtrace::print(err, format)),
            RustBacktrace::Disabled => {}
            RustBacktrace::RuntimeDisabled => {
                if FIRST_PANIC.swap(false, Ordering::SeqCst) {
                    let _ = writeln!(err, "note: run with `RUST_BACKTRACE=1` \
                                           environment variable to display a backtrace.");
                }
            }
        }
    };

    if let Some(mut local) = set_panic(None) {
        // NB. In `cfg(test)` this uses the forwarding impl
        // for `Box<dyn (::realstd::io::Write) + Send>`.
        write(&mut local);
        set_panic(Some(local));
    } else if let Some(mut out) = panic_output() {
        write(&mut out);
    }
}


#[cfg(not(test))]
#[doc(hidden)]
#[unstable(feature = "update_panic_count", issue = "0")]
pub fn update_panic_count(amt: isize) -> usize {
    loop { }
}

#[cfg(test)]
pub use realstd::rt::update_panic_count;

pub unsafe fn r#try<R, F: FnOnce() -> R>(f: F) -> Result<R, Box<dyn Any + Send>> {
    loop { }
}

pub fn panicking() -> bool {
    update_panic_count(0) != 0
}

#[cfg(not(test))]
#[panic_handler]
#[unwind(allowed)]
pub fn rust_begin_panic(info: &PanicInfo<'_>) -> ! {
    loop { }
}

#[unstable(feature = "libstd_sys_internals",
           reason = "used by the panic! macro",
           issue = "0")]
#[cold]
// If panic_immediate_abort, inline the abort call,
// otherwise avoid inlining because of it is cold path.
#[cfg_attr(not(feature="panic_immediate_abort"),inline(never))]
#[cfg_attr(    feature="panic_immediate_abort" ,inline)]
pub fn begin_panic_fmt(msg: &fmt::Arguments<'_>,
                       file_line_col: &(&'static str, u32, u32)) -> ! {
    loop { }
}

fn continue_panic_fmt(info: &PanicInfo<'_>) -> ! {
    loop { }
}

#[unstable(feature = "libstd_sys_internals",
           reason = "used by the panic! macro",
           issue = "0")]
#[cfg_attr(not(test), lang = "begin_panic")]
// never inline unless panic_immediate_abort to avoid code
// bloat at the call sites as much as possible
#[cfg_attr(not(feature="panic_immediate_abort"),inline(never))]
#[cold]
pub fn begin_panic<M: Any + Send>(msg: M, file_line_col: &(&'static str, u32, u32)) -> ! {
    loop { }
}

fn rust_panic_with_hook(payload: &mut dyn BoxMeUp,
                        message: Option<&fmt::Arguments<'_>>,
                        file_line_col: &(&str, u32, u32)) -> ! {
    loop { }
}

pub fn update_count_then_panic(msg: Box<dyn Any + Send>) -> ! {
    loop { }
}

#[inline(never)]
#[cfg_attr(not(test), rustc_std_internal_symbol)]
fn rust_panic(mut msg: &mut dyn BoxMeUp) -> ! {
    loop { }
}
