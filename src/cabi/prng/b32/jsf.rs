//! C ABI exports backed by the shared buffered stream. See [`crate::cabi`].
use crate::cabi::stream;
use crate::*;

/// Creates an opaque handle. SIMD variants return null on unsupported CPUs.
#[unsafe(no_mangle)]
pub extern "C" fn jsf32_new(seed: u32) -> *mut Jsf32 {
    stream::new::<_, u32, 1>(Jsf32::new(seed as u32))
}

/// Frees an opaque handle; null is accepted.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsf32_free(ptr: *mut Jsf32) {
    unsafe { stream::free::<Jsf32, u32, 1>(ptr) }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsf32_next_u32s(ptr: *mut Jsf32, out: *mut u32, count: usize) {
    unsafe {
        stream::fill::<Jsf32, u32, _, 1, _, _>(ptr, out, count, |r| [Rng::nextu(r)], |r| r.nextu())
    }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsf32_next_f32s(ptr: *mut Jsf32, out: *mut f32, count: usize) {
    unsafe {
        stream::fill::<Jsf32, u32, _, 1, _, _>(ptr, out, count, |r| [Rng::nextu(r)], |r| r.nextf())
    }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsf32_rand_i32s(
    ptr: *mut Jsf32,
    out: *mut i32,
    count: usize,
    min: i32,
    max: i32,
) {
    if min > max {
        return;
    }
    unsafe {
        stream::fill::<Jsf32, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn jsf32_rand_f32s(
    ptr: *mut Jsf32,
    out: *mut f32,
    count: usize,
    min: f32,
    max: f32,
) {
    if !min.is_finite() || !max.is_finite() || min >= max {
        return;
    }
    unsafe {
        stream::fill::<Jsf32, u32, _, 1, _, _>(
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
pub extern "C" fn jsf32x16_new(seed: u32) -> *mut Jsf32x16 {
    if !std::is_x86_feature_detected!("avx512f") {
        return std::ptr::null_mut();
    }
    stream::new::<_, u32, 16>(unsafe { Jsf32x16::new(seed as u32) })
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
/// Frees an opaque handle; null is accepted.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsf32x16_free(ptr: *mut Jsf32x16) {
    unsafe { stream::free::<Jsf32x16, u32, 16>(ptr) }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsf32x16_next_u32s(ptr: *mut Jsf32x16, out: *mut u32, count: usize) {
    unsafe {
        stream::fill::<Jsf32x16, u32, _, 16, _, _>(
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
pub unsafe extern "C" fn jsf32x16_next_f32s(ptr: *mut Jsf32x16, out: *mut f32, count: usize) {
    unsafe {
        stream::fill::<Jsf32x16, u32, _, 16, _, _>(
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
pub unsafe extern "C" fn jsf32x16_rand_i32s(
    ptr: *mut Jsf32x16,
    out: *mut i32,
    count: usize,
    min: i32,
    max: i32,
) {
    if min > max {
        return;
    }
    unsafe {
        stream::fill::<Jsf32x16, u32, _, 16, _, _>(
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
pub unsafe extern "C" fn jsf32x16_rand_f32s(
    ptr: *mut Jsf32x16,
    out: *mut f32,
    count: usize,
    min: f32,
    max: f32,
) {
    if !min.is_finite() || !max.is_finite() || min >= max {
        return;
    }
    unsafe {
        stream::fill::<Jsf32x16, u32, _, 16, _, _>(
            ptr,
            out,
            count,
            |r| std::mem::transmute(r.nextuv()),
            |r| r.randf(min, max),
        )
    }
}
