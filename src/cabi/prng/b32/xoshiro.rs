//! C ABI exports backed by the shared buffered stream. See [`crate::cabi`].
use crate::cabi::stream;
use crate::*;

/// Creates an opaque handle. SIMD variants return null on unsupported CPUs.
#[unsafe(no_mangle)]
pub extern "C" fn xoshiro128pp_new(seed: u32) -> *mut Xoshiro128Pp {
    stream::new::<_, u32, 1>(Xoshiro128Pp::new(seed as u32))
}

/// Frees an opaque handle; null is accepted.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoshiro128pp_free(ptr: *mut Xoshiro128Pp) {
    unsafe { stream::free::<Xoshiro128Pp, u32, 1>(ptr) }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoshiro128pp_next_u32s(
    ptr: *mut Xoshiro128Pp,
    out: *mut u32,
    count: usize,
) {
    unsafe {
        stream::fill::<Xoshiro128Pp, u32, _, 1, _, _>(
            ptr,
            out,
            count,
            |r| [Rng::nextu(r)],
            |r| r.nextu(),
        )
    }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoshiro128pp_next_f32s(
    ptr: *mut Xoshiro128Pp,
    out: *mut f32,
    count: usize,
) {
    unsafe {
        stream::fill::<Xoshiro128Pp, u32, _, 1, _, _>(
            ptr,
            out,
            count,
            |r| [Rng::nextu(r)],
            |r| r.nextf(),
        )
    }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoshiro128pp_rand_i32s(
    ptr: *mut Xoshiro128Pp,
    out: *mut i32,
    count: usize,
    min: i32,
    max: i32,
) {
    if min > max {
        return;
    }
    unsafe {
        stream::fill::<Xoshiro128Pp, u32, _, 1, _, _>(
            ptr,
            out,
            count,
            |r| [Rng::nextu(r)],
            |r| r.randi(min, max),
        )
    }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoshiro128pp_rand_f32s(
    ptr: *mut Xoshiro128Pp,
    out: *mut f32,
    count: usize,
    min: f32,
    max: f32,
) {
    if !min.is_finite() || !max.is_finite() || min >= max {
        return;
    }
    unsafe {
        stream::fill::<Xoshiro128Pp, u32, _, 1, _, _>(
            ptr,
            out,
            count,
            |r| [Rng::nextu(r)],
            |r| r.randf(min, max),
        )
    }
}

/// Creates an opaque handle. SIMD variants return null on unsupported CPUs.
#[unsafe(no_mangle)]
pub extern "C" fn xoshiro128ss_new(seed: u32) -> *mut Xoshiro128Ss {
    stream::new::<_, u32, 1>(Xoshiro128Ss::new(seed as u32))
}

/// Frees an opaque handle; null is accepted.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoshiro128ss_free(ptr: *mut Xoshiro128Ss) {
    unsafe { stream::free::<Xoshiro128Ss, u32, 1>(ptr) }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoshiro128ss_next_u32s(
    ptr: *mut Xoshiro128Ss,
    out: *mut u32,
    count: usize,
) {
    unsafe {
        stream::fill::<Xoshiro128Ss, u32, _, 1, _, _>(
            ptr,
            out,
            count,
            |r| [Rng::nextu(r)],
            |r| r.nextu(),
        )
    }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoshiro128ss_next_f32s(
    ptr: *mut Xoshiro128Ss,
    out: *mut f32,
    count: usize,
) {
    unsafe {
        stream::fill::<Xoshiro128Ss, u32, _, 1, _, _>(
            ptr,
            out,
            count,
            |r| [Rng::nextu(r)],
            |r| r.nextf(),
        )
    }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoshiro128ss_rand_i32s(
    ptr: *mut Xoshiro128Ss,
    out: *mut i32,
    count: usize,
    min: i32,
    max: i32,
) {
    if min > max {
        return;
    }
    unsafe {
        stream::fill::<Xoshiro128Ss, u32, _, 1, _, _>(
            ptr,
            out,
            count,
            |r| [Rng::nextu(r)],
            |r| r.randi(min, max),
        )
    }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoshiro128ss_rand_f32s(
    ptr: *mut Xoshiro128Ss,
    out: *mut f32,
    count: usize,
    min: f32,
    max: f32,
) {
    if !min.is_finite() || !max.is_finite() || min >= max {
        return;
    }
    unsafe {
        stream::fill::<Xoshiro128Ss, u32, _, 1, _, _>(
            ptr,
            out,
            count,
            |r| [Rng::nextu(r)],
            |r| r.randf(min, max),
        )
    }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
/// Creates an opaque handle. SIMD variants return null on unsupported CPUs.
#[unsafe(no_mangle)]
pub extern "C" fn xoshiro128ppx16_new(seed: u32) -> *mut Xoshiro128Ppx16 {
    if !std::is_x86_feature_detected!("avx512f") {
        return std::ptr::null_mut();
    }
    stream::new::<_, u32, 16>(unsafe { Xoshiro128Ppx16::new(seed as u32) })
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
/// Frees an opaque handle; null is accepted.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoshiro128ppx16_free(ptr: *mut Xoshiro128Ppx16) {
    unsafe { stream::free::<Xoshiro128Ppx16, u32, 16>(ptr) }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoshiro128ppx16_next_u32s(
    ptr: *mut Xoshiro128Ppx16,
    out: *mut u32,
    count: usize,
) {
    unsafe {
        stream::fill::<Xoshiro128Ppx16, u32, _, 16, _, _>(
            ptr,
            out,
            count,
            |r| std::mem::transmute(r.nextuv()),
            |r| r.nextu(),
        )
    }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoshiro128ppx16_next_f32s(
    ptr: *mut Xoshiro128Ppx16,
    out: *mut f32,
    count: usize,
) {
    unsafe {
        stream::fill::<Xoshiro128Ppx16, u32, _, 16, _, _>(
            ptr,
            out,
            count,
            |r| std::mem::transmute(r.nextuv()),
            |r| r.nextf(),
        )
    }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoshiro128ppx16_rand_i32s(
    ptr: *mut Xoshiro128Ppx16,
    out: *mut i32,
    count: usize,
    min: i32,
    max: i32,
) {
    if min > max {
        return;
    }
    unsafe {
        stream::fill::<Xoshiro128Ppx16, u32, _, 16, _, _>(
            ptr,
            out,
            count,
            |r| std::mem::transmute(r.nextuv()),
            |r| r.randi(min, max),
        )
    }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoshiro128ppx16_rand_f32s(
    ptr: *mut Xoshiro128Ppx16,
    out: *mut f32,
    count: usize,
    min: f32,
    max: f32,
) {
    if !min.is_finite() || !max.is_finite() || min >= max {
        return;
    }
    unsafe {
        stream::fill::<Xoshiro128Ppx16, u32, _, 16, _, _>(
            ptr,
            out,
            count,
            |r| std::mem::transmute(r.nextuv()),
            |r| r.randf(min, max),
        )
    }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
/// Creates an opaque handle. SIMD variants return null on unsupported CPUs.
#[unsafe(no_mangle)]
pub extern "C" fn xoshiro128ssx16_new(seed: u32) -> *mut Xoshiro128Ssx16 {
    if !std::is_x86_feature_detected!("avx512f") {
        return std::ptr::null_mut();
    }
    stream::new::<_, u32, 16>(unsafe { Xoshiro128Ssx16::new(seed as u32) })
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
/// Frees an opaque handle; null is accepted.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoshiro128ssx16_free(ptr: *mut Xoshiro128Ssx16) {
    unsafe { stream::free::<Xoshiro128Ssx16, u32, 16>(ptr) }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoshiro128ssx16_next_u32s(
    ptr: *mut Xoshiro128Ssx16,
    out: *mut u32,
    count: usize,
) {
    unsafe {
        stream::fill::<Xoshiro128Ssx16, u32, _, 16, _, _>(
            ptr,
            out,
            count,
            |r| std::mem::transmute(r.nextuv()),
            |r| r.nextu(),
        )
    }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoshiro128ssx16_next_f32s(
    ptr: *mut Xoshiro128Ssx16,
    out: *mut f32,
    count: usize,
) {
    unsafe {
        stream::fill::<Xoshiro128Ssx16, u32, _, 16, _, _>(
            ptr,
            out,
            count,
            |r| std::mem::transmute(r.nextuv()),
            |r| r.nextf(),
        )
    }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoshiro128ssx16_rand_i32s(
    ptr: *mut Xoshiro128Ssx16,
    out: *mut i32,
    count: usize,
    min: i32,
    max: i32,
) {
    if min > max {
        return;
    }
    unsafe {
        stream::fill::<Xoshiro128Ssx16, u32, _, 16, _, _>(
            ptr,
            out,
            count,
            |r| std::mem::transmute(r.nextuv()),
            |r| r.randi(min, max),
        )
    }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoshiro128ssx16_rand_f32s(
    ptr: *mut Xoshiro128Ssx16,
    out: *mut f32,
    count: usize,
    min: f32,
    max: f32,
) {
    if !min.is_finite() || !max.is_finite() || min >= max {
        return;
    }
    unsafe {
        stream::fill::<Xoshiro128Ssx16, u32, _, 16, _, _>(
            ptr,
            out,
            count,
            |r| std::mem::transmute(r.nextuv()),
            |r| r.randf(min, max),
        )
    }
}
