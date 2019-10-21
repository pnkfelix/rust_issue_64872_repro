#![no_std]

#![feature(lang_items)]

#![feature(rustc_attrs)]
#![feature(alloc_error_handler)]
#![feature(allocator_internals)]

#![default_lib_allocator]

mod uw {
    #![allow(non_camel_case_types)]
    pub type _Unwind_Action = ();
    pub type _Unwind_Exception_Class = ();
    pub type _Unwind_Exception = ();
    pub type _Unwind_Context = ();
    pub type _Unwind_Reason_Code = ();
}

#[lang = "eh_personality"]
#[no_mangle]
#[allow(unused)]
unsafe extern "C" fn rust_eh_personality(version: isize,
                                         actions: uw::_Unwind_Action,
                                         exception_class: uw::_Unwind_Exception_Class,
                                         exception_object: *mut uw::_Unwind_Exception,
                                         context: *mut uw::_Unwind_Context)
                                         -> uw::_Unwind_Reason_Code {
    loop { }
}

use core::panic::PanicInfo;

#[panic_handler]
pub fn rust_begin_panic(_info: &PanicInfo<'_>) -> ! {
    loop { }
}

pub mod fmt {
    pub use core::fmt::Debug;
    pub use core::fmt::Formatter;
    pub use core::fmt::Result;
    pub use core::fmt::DebugTuple;
}

pub mod prelude {
    pub mod v1 {
        pub use core::prelude::v1::{Debug};
    }
}

use core::alloc::Layout;

#[alloc_error_handler]
pub fn rust_oom(_layout: Layout) -> ! { loop { } }

#[rustc_std_internal_symbol]
pub unsafe extern fn __rdl_alloc(_size: usize, _align: usize) -> *mut u8 { loop { } }

#[rustc_std_internal_symbol]
pub unsafe extern fn __rdl_dealloc(_ptr: *mut u8,
                                   _size: usize,
                                   _align: usize) { loop { } }

#[rustc_std_internal_symbol]
pub unsafe extern fn __rdl_realloc(_ptr: *mut u8,
                                   _old_size: usize,
                                   _align: usize,
                                   _new_size: usize) -> *mut u8 { loop { } }

#[rustc_std_internal_symbol]
pub unsafe extern fn __rdl_alloc_zeroed(_size: usize, _align: usize) -> *mut u8 { loop { } }
