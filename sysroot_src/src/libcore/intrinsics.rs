//! Compiler intrinsics.

#![unstable(feature = "core_intrinsics",
            reason = "intrinsics are unlikely to ever be stabilized, instead \
                      they should be used through stabilized interfaces \
                      in the rest of the standard library",
            issue = "0")]
#![allow(missing_docs)]

extern "rust-intrinsic" {

    pub fn atomic_cxchg<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchg_acq<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchg_rel<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchg_acqrel<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchg_relaxed<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchg_failrelaxed<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchg_failacq<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchg_acq_failrelaxed<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchg_acqrel_failrelaxed<T>(dst: *mut T, old: T, src: T) -> (T, bool);

    pub fn atomic_cxchgweak<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchgweak_acq<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchgweak_rel<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchgweak_acqrel<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchgweak_relaxed<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchgweak_failrelaxed<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchgweak_failacq<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchgweak_acq_failrelaxed<T>(dst: *mut T, old: T, src: T) -> (T, bool);
    pub fn atomic_cxchgweak_acqrel_failrelaxed<T>(dst: *mut T, old: T, src: T) -> (T, bool);

    pub fn atomic_load<T>(src: *const T) -> T;
    pub fn atomic_load_acq<T>(src: *const T) -> T;
    pub fn atomic_load_relaxed<T>(src: *const T) -> T;
    pub fn atomic_load_unordered<T>(src: *const T) -> T;

    pub fn atomic_store<T>(dst: *mut T, val: T);
    pub fn atomic_store_rel<T>(dst: *mut T, val: T);
    pub fn atomic_store_relaxed<T>(dst: *mut T, val: T);
    pub fn atomic_store_unordered<T>(dst: *mut T, val: T);

    pub fn atomic_xchg<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xchg_acq<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xchg_rel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xchg_acqrel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xchg_relaxed<T>(dst: *mut T, src: T) -> T;

    pub fn atomic_xadd<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xadd_acq<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xadd_rel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xadd_acqrel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xadd_relaxed<T>(dst: *mut T, src: T) -> T;

    pub fn atomic_xsub<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xsub_acq<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xsub_rel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xsub_acqrel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xsub_relaxed<T>(dst: *mut T, src: T) -> T;

    pub fn atomic_and<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_and_acq<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_and_rel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_and_acqrel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_and_relaxed<T>(dst: *mut T, src: T) -> T;

    pub fn atomic_nand<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_nand_acq<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_nand_rel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_nand_acqrel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_nand_relaxed<T>(dst: *mut T, src: T) -> T;

    pub fn atomic_or<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_or_acq<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_or_rel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_or_acqrel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_or_relaxed<T>(dst: *mut T, src: T) -> T;

    pub fn atomic_xor<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xor_acq<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xor_rel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xor_acqrel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xor_relaxed<T>(dst: *mut T, src: T) -> T;

    pub fn atomic_max<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_max_acq<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_max_rel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_max_acqrel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_max_relaxed<T>(dst: *mut T, src: T) -> T;

    pub fn atomic_min<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_min_acq<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_min_rel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_min_acqrel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_min_relaxed<T>(dst: *mut T, src: T) -> T;

    pub fn atomic_umin<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_umin_acq<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_umin_rel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_umin_acqrel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_umin_relaxed<T>(dst: *mut T, src: T) -> T;

    pub fn atomic_umax<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_umax_acq<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_umax_rel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_umax_acqrel<T>(dst: *mut T, src: T) -> T;
    pub fn atomic_umax_relaxed<T>(dst: *mut T, src: T) -> T;

    pub fn prefetch_read_data<T>(data: *const T, locality: i32);
    pub fn prefetch_write_data<T>(data: *const T, locality: i32);
    pub fn prefetch_read_instruction<T>(data: *const T, locality: i32);
    pub fn prefetch_write_instruction<T>(data: *const T, locality: i32);
}

extern "rust-intrinsic" {

    pub fn atomic_fence();
    pub fn atomic_fence_acq();
    pub fn atomic_fence_rel();
    pub fn atomic_fence_acqrel();

    pub fn atomic_singlethreadfence();
    pub fn atomic_singlethreadfence_acq();
    pub fn atomic_singlethreadfence_rel();
    pub fn atomic_singlethreadfence_acqrel();

    pub fn rustc_peek<T>(_: T) -> T;

    pub fn abort() -> !;

    pub fn unreachable() -> !;

    pub fn assume(b: bool);

    pub fn likely(b: bool) -> bool;

    pub fn unlikely(b: bool) -> bool;

    pub fn breakpoint();

    pub fn size_of<T>() -> usize;

    pub fn move_val_init<T>(dst: *mut T, src: T);

    pub fn min_align_of<T>() -> usize;
    pub fn pref_align_of<T>() -> usize;

    pub fn size_of_val<T: ?Sized>(_: &T) -> usize;
    pub fn min_align_of_val<T: ?Sized>(_: &T) -> usize;

    pub fn type_name<T: ?Sized>() -> &'static str;

    pub fn type_id<T: ?Sized + 'static>() -> u64;

    pub fn panic_if_uninhabited<T>();

    #[unstable(feature = "core_intrinsics",
               reason = "intrinsics are unlikely to ever be stabilized, instead \
                         they should be used through stabilized interfaces \
                         in the rest of the standard library",
               issue = "0")]
    #[rustc_deprecated(reason = "superseded by MaybeUninit, removal planned",
                       since = "1.38.0")]
    pub fn init<T>() -> T;

    #[unstable(feature = "core_intrinsics",
               reason = "intrinsics are unlikely to ever be stabilized, instead \
                         they should be used through stabilized interfaces \
                         in the rest of the standard library",
               issue = "0")]
    #[rustc_deprecated(reason = "superseded by MaybeUninit, removal planned",
                       since = "1.38.0")]
    pub fn uninit<T>() -> T;

    pub fn forget<T: ?Sized>(_: T);

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn transmute<T, U>(e: T) -> U;

    pub fn needs_drop<T>() -> bool;

    pub fn offset<T>(dst: *const T, offset: isize) -> *const T;

    pub fn arith_offset<T>(dst: *const T, offset: isize) -> *const T;

    pub fn volatile_copy_nonoverlapping_memory<T>(dst: *mut T, src: *const T,
                                                  count: usize);
    pub fn volatile_copy_memory<T>(dst: *mut T, src: *const T, count: usize);
    pub fn volatile_set_memory<T>(dst: *mut T, val: u8, count: usize);

    pub fn volatile_load<T>(src: *const T) -> T;
    pub fn volatile_store<T>(dst: *mut T, val: T);

    pub fn unaligned_volatile_load<T>(src: *const T) -> T;
    pub fn unaligned_volatile_store<T>(dst: *mut T, val: T);

    pub fn sqrtf32(x: f32) -> f32;
    pub fn sqrtf64(x: f64) -> f64;

    pub fn powif32(a: f32, x: i32) -> f32;
    pub fn powif64(a: f64, x: i32) -> f64;

    pub fn sinf32(x: f32) -> f32;
    pub fn sinf64(x: f64) -> f64;

    pub fn cosf32(x: f32) -> f32;
    pub fn cosf64(x: f64) -> f64;

    pub fn powf32(a: f32, x: f32) -> f32;
    pub fn powf64(a: f64, x: f64) -> f64;

    pub fn expf32(x: f32) -> f32;
    pub fn expf64(x: f64) -> f64;

    pub fn exp2f32(x: f32) -> f32;
    pub fn exp2f64(x: f64) -> f64;

    pub fn logf32(x: f32) -> f32;
    pub fn logf64(x: f64) -> f64;

    pub fn log10f32(x: f32) -> f32;
    pub fn log10f64(x: f64) -> f64;

    pub fn log2f32(x: f32) -> f32;
    pub fn log2f64(x: f64) -> f64;

    pub fn fmaf32(a: f32, b: f32, c: f32) -> f32;
    pub fn fmaf64(a: f64, b: f64, c: f64) -> f64;

    pub fn fabsf32(x: f32) -> f32;
    pub fn fabsf64(x: f64) -> f64;

    pub fn minnumf32(x: f32, y: f32) -> f32;
    pub fn minnumf64(x: f64, y: f64) -> f64;
    pub fn maxnumf32(x: f32, y: f32) -> f32;
    pub fn maxnumf64(x: f64, y: f64) -> f64;

    pub fn copysignf32(x: f32, y: f32) -> f32;
    pub fn copysignf64(x: f64, y: f64) -> f64;

    pub fn floorf32(x: f32) -> f32;
    pub fn floorf64(x: f64) -> f64;

    pub fn ceilf32(x: f32) -> f32;
    pub fn ceilf64(x: f64) -> f64;

    pub fn truncf32(x: f32) -> f32;
    pub fn truncf64(x: f64) -> f64;

    pub fn rintf32(x: f32) -> f32;
    pub fn rintf64(x: f64) -> f64;

    pub fn nearbyintf32(x: f32) -> f32;
    pub fn nearbyintf64(x: f64) -> f64;

    pub fn roundf32(x: f32) -> f32;
    pub fn roundf64(x: f64) -> f64;

    pub fn fadd_fast<T>(a: T, b: T) -> T;

    pub fn fsub_fast<T>(a: T, b: T) -> T;

    pub fn fmul_fast<T>(a: T, b: T) -> T;

    pub fn fdiv_fast<T>(a: T, b: T) -> T;

    pub fn frem_fast<T>(a: T, b: T) -> T;


    pub fn ctpop<T>(x: T) -> T;

    pub fn ctlz<T>(x: T) -> T;

    pub fn ctlz_nonzero<T>(x: T) -> T;

    pub fn cttz<T>(x: T) -> T;

    pub fn cttz_nonzero<T>(x: T) -> T;

    pub fn bswap<T>(x: T) -> T;

    pub fn bitreverse<T>(x: T) -> T;

    pub fn add_with_overflow<T>(x: T, y: T) -> (T, bool);

    pub fn sub_with_overflow<T>(x: T, y: T) -> (T, bool);

    pub fn mul_with_overflow<T>(x: T, y: T) -> (T, bool);

    pub fn exact_div<T>(x: T, y: T) -> T;

    pub fn unchecked_div<T>(x: T, y: T) -> T;
    pub fn unchecked_rem<T>(x: T, y: T) -> T;

    pub fn unchecked_shl<T>(x: T, y: T) -> T;
    pub fn unchecked_shr<T>(x: T, y: T) -> T;

    pub fn unchecked_add<T>(x: T, y: T) -> T;

    pub fn unchecked_sub<T>(x: T, y: T) -> T;

    pub fn unchecked_mul<T>(x: T, y: T) -> T;

    pub fn rotate_left<T>(x: T, y: T) -> T;

    pub fn rotate_right<T>(x: T, y: T) -> T;

    pub fn wrapping_add<T>(a: T, b: T) -> T;
    pub fn wrapping_sub<T>(a: T, b: T) -> T;
    pub fn wrapping_mul<T>(a: T, b: T) -> T;

    pub fn saturating_add<T>(a: T, b: T) -> T;
    pub fn saturating_sub<T>(a: T, b: T) -> T;

    pub fn discriminant_value<T>(v: &T) -> u64;

    pub fn r#try(f: fn(*mut u8), data: *mut u8, local_ptr: *mut u8) -> i32;

    pub fn nontemporal_store<T>(ptr: *mut T, val: T);
}


pub(crate) fn is_aligned_and_not_null<T>(ptr: *const T) -> bool { loop { } }

fn overlaps<T>(src: *const T, dst: *const T, count: usize) -> bool { loop { } }

#[stable(feature = "rust1", since = "1.0.0")]
#[inline]
pub unsafe fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize) { loop { } }

#[stable(feature = "rust1", since = "1.0.0")]
#[inline]
pub unsafe fn copy<T>(src: *const T, dst: *mut T, count: usize) { loop { } }

#[stable(feature = "rust1", since = "1.0.0")]
#[inline]
pub unsafe fn write_bytes<T>(dst: *mut T, val: u8, count: usize) { loop { } }
