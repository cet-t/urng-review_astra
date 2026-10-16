//! C ABI exports backed by the shared buffered stream. See [`crate::cabi`].
use crate::cabi::stream;
use crate::*;

/// Creates an opaque handle. SIMD variants return null on unsupported CPUs.
#[unsafe(no_mangle)]
pub extern "C" fn xoroshiro64ss_new(seed: u32) -> *mut Xoroshiro64Ss {
    stream::new::<_, u32, 1>(Xoroshiro64Ss::new(seed as u32))
}

/// Frees an opaque handle; null is accepted.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoroshiro64ss_free(ptr: *mut Xoroshiro64Ss) {
    unsafe { stream::free::<Xoroshiro64Ss, u32, 1>(ptr) }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoroshiro64ss_next_u32s(
    ptr: *mut Xoroshiro64Ss,
    out: *mut u32,
    count: usize,
) {
    unsafe {
        stream::fill::<Xoroshiro64Ss, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn xoroshiro64ss_next_f32s(
    ptr: *mut Xoroshiro64Ss,
    out: *mut f32,
    count: usize,
) {
    unsafe {
        stream::fill::<Xoroshiro64Ss, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn xoroshiro64ss_rand_i32s(
    ptr: *mut Xoroshiro64Ss,
    out: *mut i32,
    count: usize,
    min: i32,
    max: i32,
) {
    if min > max {
        return;
    }
    unsafe {
        stream::fill::<Xoroshiro64Ss, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn xoroshiro64ss_rand_f32s(
    ptr: *mut Xoroshiro64Ss,
    out: *mut f32,
    count: usize,
    min: f32,
    max: f32,
) {
    if !min.is_finite() || !max.is_finite() || min >= max {
        return;
    }
    unsafe {
        stream::fill::<Xoroshiro64Ss, u32, _, 1, _, _>(
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
pub extern "C" fn xoroshiro64ssx8_new(seed: u32) -> *mut Xoroshiro64Ssx8 {
    if !std::is_x86_feature_detected!("avx2") {
        return std::ptr::null_mut();
    }
    stream::new::<_, u32, 8>(unsafe { Xoroshiro64Ssx8::new(seed as u32) })
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
/// Frees an opaque handle; null is accepted.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoroshiro64ssx8_free(ptr: *mut Xoroshiro64Ssx8) {
    unsafe { stream::free::<Xoroshiro64Ssx8, u32, 8>(ptr) }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoroshiro64ssx8_next_u32s(
    ptr: *mut Xoroshiro64Ssx8,
    out: *mut u32,
    count: usize,
) {
    unsafe {
        stream::fill::<Xoroshiro64Ssx8, u32, _, 8, _, _>(
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
pub unsafe extern "C" fn xoroshiro64ssx8_next_f32s(
    ptr: *mut Xoroshiro64Ssx8,
    out: *mut f32,
    count: usize,
) {
    unsafe {
        stream::fill::<Xoroshiro64Ssx8, u32, _, 8, _, _>(
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
pub unsafe extern "C" fn xoroshiro64ssx8_rand_i32s(
    ptr: *mut Xoroshiro64Ssx8,
    out: *mut i32,
    count: usize,
    min: i32,
    max: i32,
) {
    if min > max {
        return;
    }
    unsafe {
        stream::fill::<Xoroshiro64Ssx8, u32, _, 8, _, _>(
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
pub unsafe extern "C" fn xoroshiro64ssx8_rand_f32s(
    ptr: *mut Xoroshiro64Ssx8,
    out: *mut f32,
    count: usize,
    min: f32,
    max: f32,
) {
    if !min.is_finite() || !max.is_finite() || min >= max {
        return;
    }
    unsafe {
        stream::fill::<Xoroshiro64Ssx8, u32, _, 8, _, _>(
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
pub extern "C" fn xoroshiro64ssx16_new(seed: u32) -> *mut Xoroshiro64Ssx16 {
    if !std::is_x86_feature_detected!("avx512f") {
        return std::ptr::null_mut();
    }
    stream::new::<_, u32, 16>(unsafe { Xoroshiro64Ssx16::new(seed as u32) })
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
/// Frees an opaque handle; null is accepted.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoroshiro64ssx16_free(ptr: *mut Xoroshiro64Ssx16) {
    unsafe { stream::free::<Xoroshiro64Ssx16, u32, 16>(ptr) }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xoroshiro64ssx16_next_u32s(
    ptr: *mut Xoroshiro64Ssx16,
    out: *mut u32,
    count: usize,
) {
    unsafe {
        stream::fill::<Xoroshiro64Ssx16, u32, _, 16, _, _>(
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
pub unsafe extern "C" fn xoroshiro64ssx16_next_f32s(
    ptr: *mut Xoroshiro64Ssx16,
    out: *mut f32,
    count: usize,
) {
    unsafe {
        stream::fill::<Xoroshiro64Ssx16, u32, _, 16, _, _>(
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
pub unsafe extern "C" fn xoroshiro64ssx16_rand_i32s(
    ptr: *mut Xoroshiro64Ssx16,
    out: *mut i32,
    count: usize,
    min: i32,
    max: i32,
) {
    if min > max {
        return;
    }
    unsafe {
        stream::fill::<Xoroshiro64Ssx16, u32, _, 16, _, _>(
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
pub unsafe extern "C" fn xoroshiro64ssx16_rand_f32s(
    ptr: *mut Xoroshiro64Ssx16,
    out: *mut f32,
    count: usize,
    min: f32,
    max: f32,
) {
    if !min.is_finite() || !max.is_finite() || min >= max {
        return;
    }
    unsafe {
        stream::fill::<Xoroshiro64Ssx16, u32, _, 16, _, _>(
            ptr,
            out,
            count,
            |r| std::mem::transmute(r.nextuv()),
            |r| r.randf(min, max),
        )
    }
}
