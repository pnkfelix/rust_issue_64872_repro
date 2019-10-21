#![stable(feature = "", since = "1.30.0")]

#![allow(non_camel_case_types)]


use crate::fmt;
use crate::marker::PhantomData;
use crate::ops::{Deref, DerefMut};

#[repr(u8)]
#[stable(feature = "raw_os", since = "1.1.0")]
pub enum c_void {
    #[unstable(feature = "c_void_variant", reason = "temporary implementation detail",
               issue = "0")]
    #[doc(hidden)] __variant1,
    #[unstable(feature = "c_void_variant", reason = "temporary implementation detail",
               issue = "0")]
    #[doc(hidden)] __variant2,
}

#[stable(feature = "std_debug", since = "1.16.0")]
impl fmt::Debug for c_void {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[cfg(any(all(not(target_arch = "aarch64"), not(target_arch = "powerpc"),
              not(target_arch = "x86_64"), not(target_arch = "asmjs")),
          all(target_arch = "aarch64", target_os = "ios"),
          windows))]
#[repr(transparent)]
#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
           issue = "44930")]
#[lang = "va_list"]
pub struct VaListImpl<'f> {
    ptr: *mut c_void,

    _marker: PhantomData<&'f mut &'f c_void>,
}

#[cfg(any(all(not(target_arch = "aarch64"), not(target_arch = "powerpc"),
              not(target_arch = "x86_64"), not(target_arch = "asmjs")),
          all(target_arch = "aarch64", target_os = "ios"),
          windows))]
#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
           issue = "44930")]
impl<'f> fmt::Debug for VaListImpl<'f> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[cfg(all(target_arch = "aarch64", not(target_os = "ios"), not(windows)))]
#[repr(C)]
#[derive(Debug)]
#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
           issue = "44930")]
#[lang = "va_list"]
pub struct VaListImpl<'f> {
    stack: *mut c_void,
    gr_top: *mut c_void,
    vr_top: *mut c_void,
    gr_offs: i32,
    vr_offs: i32,
    _marker: PhantomData<&'f mut &'f c_void>,
}

#[cfg(all(target_arch = "powerpc", not(windows)))]
#[repr(C)]
#[derive(Debug)]
#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
           issue = "44930")]
#[lang = "va_list"]
pub struct VaListImpl<'f> {
    gpr: u8,
    fpr: u8,
    reserved: u16,
    overflow_arg_area: *mut c_void,
    reg_save_area: *mut c_void,
    _marker: PhantomData<&'f mut &'f c_void>,
}

#[cfg(all(target_arch = "x86_64", not(windows)))]
#[repr(C)]
#[derive(Debug)]
#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
           issue = "44930")]
#[lang = "va_list"]
pub struct VaListImpl<'f> {
    gp_offset: i32,
    fp_offset: i32,
    overflow_arg_area: *mut c_void,
    reg_save_area: *mut c_void,
    _marker: PhantomData<&'f mut &'f c_void>,
}

#[cfg(all(target_arch = "asmjs", not(windows)))]
#[repr(C)]
#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
           issue = "44930")]
#[lang = "va_list"]
pub struct VaListImpl<'f> {
    inner: [crate::mem::MaybeUninit<i32>; 4],
    _marker: PhantomData<&'f mut &'f c_void>,
}

#[cfg(all(target_arch = "asmjs", not(windows)))]
#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
           issue = "44930")]
impl<'f> fmt::Debug for VaListImpl<'f> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

#[repr(transparent)]
#[derive(Debug)]
#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
           issue = "44930")]
pub struct VaList<'a, 'f: 'a> {
    #[cfg(any(all(not(target_arch = "aarch64"), not(target_arch = "powerpc"),
                  not(target_arch = "x86_64"), not(target_arch = "asmjs")),
              all(target_arch = "aarch64", target_os = "ios"),
              windows))]
    inner: VaListImpl<'f>,

    #[cfg(all(any(target_arch = "aarch64", target_arch = "powerpc",
                  target_arch = "x86_64", target_arch = "asmjs"),
              any(not(target_arch = "aarch64"), not(target_os = "ios")),
              not(windows)))]
    inner: &'a mut VaListImpl<'f>,

    _marker: PhantomData<&'a mut VaListImpl<'f>>,
}

#[cfg(any(all(not(target_arch = "aarch64"), not(target_arch = "powerpc"),
              not(target_arch = "x86_64"), not(target_arch = "asmjs")),
          all(target_arch = "aarch64", target_os = "ios"),
          windows))]
#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
           issue = "44930")]
impl<'f> VaListImpl<'f> {
    #[inline]
    pub fn as_va_list<'a>(&'a mut self) -> VaList<'a, 'f> { loop { } }
}

#[cfg(all(any(target_arch = "aarch64", target_arch = "powerpc",
              target_arch = "x86_64", target_arch = "asmjs"),
          any(not(target_arch = "aarch64"), not(target_os = "ios")),
          not(windows)))]
#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
           issue = "44930")]
impl<'f> VaListImpl<'f> {
    #[inline]
    pub fn as_va_list<'a>(&'a mut self) -> VaList<'a, 'f> { loop { } }
}

#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
           issue = "44930")]
impl<'a, 'f: 'a> Deref for VaList<'a, 'f> {
    type Target = VaListImpl<'f>;

    #[inline]
    fn deref(&self) -> &VaListImpl<'f> { loop { } }
}

#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
           issue = "44930")]
impl<'a, 'f: 'a> DerefMut for VaList<'a, 'f> {
    #[inline]
    fn deref_mut(&mut self) -> &mut VaListImpl<'f> { loop { } }
}

mod sealed_trait {
    #[unstable(feature = "c_variadic",
               reason = "the `c_variadic` feature has not been properly tested on \
                         all supported platforms",
               issue = "44930")]
    pub trait VaArgSafe {}
}

macro_rules! impl_va_arg_safe {
    ($($t:ty),+) => {
        $(
            #[unstable(feature = "c_variadic",
                       reason = "the `c_variadic` feature has not been properly tested on \
                                 all supported platforms",
                       issue = "44930")]
            impl sealed_trait::VaArgSafe for $t {}
        )+
    }
}

impl_va_arg_safe!{i8, i16, i32, i64, usize}
impl_va_arg_safe!{u8, u16, u32, u64, isize}
impl_va_arg_safe!{f64}

#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
           issue = "44930")]
impl<T> sealed_trait::VaArgSafe for *mut T {}
#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
           issue = "44930")]
impl<T> sealed_trait::VaArgSafe for *const T {}

#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
           issue = "44930")]
impl<'f> VaListImpl<'f> {
    #[inline]
    pub unsafe fn arg<T: sealed_trait::VaArgSafe>(&mut self) -> T { loop { } }

    pub unsafe fn with_copy<F, R>(&self, f: F) -> R
    where F: for<'copy> FnOnce(VaList<'copy, 'f>) -> R { loop { } }
}

#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
           issue = "44930")]
impl<'f> Clone for VaListImpl<'f> {
    #[inline]
    fn clone(&self) -> Self { loop { } }
}

#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
           issue = "44930")]
impl<'f> Drop for VaListImpl<'f> {
    fn drop(&mut self) { loop { } }
}

extern "rust-intrinsic" {
    fn va_end(ap: &mut VaListImpl<'_>);

    fn va_copy<'f>(dest: *mut VaListImpl<'f>, src: &VaListImpl<'f>);

    fn va_arg<T: sealed_trait::VaArgSafe>(ap: &mut VaListImpl<'_>) -> T;
}
