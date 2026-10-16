//! C ABI exports backed by the shared buffered stream. See [`crate::cabi`].
use crate::cabi::stream;
use crate::*;

/// Creates an opaque handle. SIMD variants return null on unsupported CPUs.
#[unsafe(no_mangle)]
pub extern "C" fn cet64_new(seed: u64) -> *mut Cet64 {
    stream::new::<_, u64, 1>(Cet64::new(seed))
}

/// Frees an opaque handle; null is accepted.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cet64_free(ptr: *mut Cet64) {
    unsafe { stream::free::<Cet64, u64, 1>(ptr) }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cet64_next_u64s(ptr: *mut Cet64, out: *mut u64, count: usize) {
    unsafe {
        stream::fill::<Cet64, u64, _, 1, _, _>(ptr, out, count, |r| [Rng::nextu(r)], |r| r.nextu())
    }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cet64_next_f64s(ptr: *mut Cet64, out: *mut f64, count: usize) {
    unsafe {
        stream::fill::<Cet64, u64, _, 1, _, _>(ptr, out, count, |r| [Rng::nextu(r)], |r| r.nextf())
    }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cet64_rand_i64s(
    ptr: *mut Cet64,
    out: *mut i64,
    count: usize,
    min: i64,
    max: i64,
) {
    if min > max {
        return;
    }
    unsafe {
        stream::fill::<Cet64, u64, _, 1, _, _>(
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
pub unsafe extern "C" fn cet64_rand_f64s(
    ptr: *mut Cet64,
    out: *mut f64,
    count: usize,
    min: f64,
    max: f64,
) {
    if !min.is_finite() || !max.is_finite() || min >= max {
        return;
    }
    unsafe {
        stream::fill::<Cet64, u64, _, 1, _, _>(
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
pub extern "C" fn cet256_new(seed: u64) -> *mut Cet256 {
    stream::new::<_, u64, 1>(Cet256::new(seed))
}

/// Frees an opaque handle; null is accepted.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cet256_free(ptr: *mut Cet256) {
    unsafe { stream::free::<Cet256, u64, 1>(ptr) }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cet256_next_u64s(ptr: *mut Cet256, out: *mut u64, count: usize) {
    unsafe {
        stream::fill::<Cet256, u64, _, 1, _, _>(ptr, out, count, |r| [Rng::nextu(r)], |r| r.nextu())
    }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cet256_next_f64s(ptr: *mut Cet256, out: *mut f64, count: usize) {
    unsafe {
        stream::fill::<Cet256, u64, _, 1, _, _>(ptr, out, count, |r| [Rng::nextu(r)], |r| r.nextf())
    }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cet256_rand_i64s(
    ptr: *mut Cet256,
    out: *mut i64,
    count: usize,
    min: i64,
    max: i64,
) {
    if min > max {
        return;
    }
    unsafe {
        stream::fill::<Cet256, u64, _, 1, _, _>(
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
pub unsafe extern "C" fn cet256_rand_f64s(
    ptr: *mut Cet256,
    out: *mut f64,
    count: usize,
    min: f64,
    max: f64,
) {
    if !min.is_finite() || !max.is_finite() || min >= max {
        return;
    }
    unsafe {
        stream::fill::<Cet256, u64, _, 1, _, _>(
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
pub extern "C" fn cet64x8_new(seed: u64) -> *mut Cet64x8 {
    if !std::is_x86_feature_detected!("avx512dq") || !std::is_x86_feature_detected!("avx512f") {
        return std::ptr::null_mut();
    }
    stream::new::<_, u64, 8>(Cet64x8::new(seed))
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
/// Frees an opaque handle; null is accepted.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cet64x8_free(ptr: *mut Cet64x8) {
    unsafe { stream::free::<Cet64x8, u64, 8>(ptr) }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cet64x8_next_u64s(ptr: *mut Cet64x8, out: *mut u64, count: usize) {
    unsafe {
        stream::fill::<Cet64x8, u64, _, 8, _, _>(ptr, out, count, |r| r.nextu(), |r| r.nextu())
    }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
/// Creates an opaque handle. SIMD variants return null on unsupported CPUs.
#[unsafe(no_mangle)]
pub extern "C" fn cet256x2_new(seed: u64) -> *mut Cet256x2 {
    if !std::is_x86_feature_detected!("avx512dq") || !std::is_x86_feature_detected!("avx512f") {
        return std::ptr::null_mut();
    }
    stream::new::<_, u64, 2>(Cet256x2::new(seed))
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
/// Frees an opaque handle; null is accepted.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cet256x2_free(ptr: *mut Cet256x2) {
    unsafe { stream::free::<Cet256x2, u64, 2>(ptr) }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cet256x2_next_u64s(ptr: *mut Cet256x2, out: *mut u64, count: usize) {
    unsafe {
        stream::fill::<Cet256x2, u64, _, 2, _, _>(ptr, out, count, |r| r.nextu(), |r| r.nextu())
    }
}
