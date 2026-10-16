//! C ABI exports backed by the shared buffered stream. See [`crate::cabi`].
use crate::cabi::stream;
use crate::*;

/// Creates an opaque handle. SIMD variants return null on unsupported CPUs.
#[unsafe(no_mangle)]
pub extern "C" fn mt19937_new(seed: u32) -> *mut Mt19937 {
    stream::new::<_, u32, 1>(Mt19937::new(seed as u32))
}

/// Frees an opaque handle; null is accepted.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt19937_free(ptr: *mut Mt19937) {
    unsafe { stream::free::<Mt19937, u32, 1>(ptr) }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt19937_next_u32s(ptr: *mut Mt19937, out: *mut u32, count: usize) {
    unsafe {
        stream::fill::<Mt19937, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn mt19937_next_f32s(ptr: *mut Mt19937, out: *mut f32, count: usize) {
    unsafe {
        stream::fill::<Mt19937, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn mt19937_rand_i32s(
    ptr: *mut Mt19937,
    out: *mut i32,
    count: usize,
    min: i32,
    max: i32,
) {
    if min > max {
        return;
    }
    unsafe {
        stream::fill::<Mt19937, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn mt19937_rand_f32s(
    ptr: *mut Mt19937,
    out: *mut f32,
    count: usize,
    min: f32,
    max: f32,
) {
    if !min.is_finite() || !max.is_finite() || min >= max {
        return;
    }
    unsafe {
        stream::fill::<Mt19937, u32, _, 1, _, _>(
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
pub extern "C" fn sfmt19937_new(seed: u64) -> *mut Sfmt19937 {
    stream::new::<_, u32, 1>(Sfmt19937::new(seed as u32))
}

/// Frees an opaque handle; null is accepted.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfmt19937_free(ptr: *mut Sfmt19937) {
    unsafe { stream::free::<Sfmt19937, u32, 1>(ptr) }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfmt19937_next_u32s(ptr: *mut Sfmt19937, out: *mut u32, count: usize) {
    unsafe {
        stream::fill::<Sfmt19937, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt19937_next_f32s(ptr: *mut Sfmt19937, out: *mut f32, count: usize) {
    unsafe {
        stream::fill::<Sfmt19937, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt19937_rand_i32s(
    ptr: *mut Sfmt19937,
    out: *mut i32,
    count: usize,
    min: i32,
    max: i32,
) {
    if min > max {
        return;
    }
    unsafe {
        stream::fill::<Sfmt19937, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt19937_rand_f32s(
    ptr: *mut Sfmt19937,
    out: *mut f32,
    count: usize,
    min: f32,
    max: f32,
) {
    if !min.is_finite() || !max.is_finite() || min >= max {
        return;
    }
    unsafe {
        stream::fill::<Sfmt19937, u32, _, 1, _, _>(
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
pub extern "C" fn sfmt607_new(seed: u64) -> *mut Sfmt607 {
    stream::new::<_, u32, 1>(Sfmt607::new(seed as u32))
}

/// Frees an opaque handle; null is accepted.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfmt607_free(ptr: *mut Sfmt607) {
    unsafe { stream::free::<Sfmt607, u32, 1>(ptr) }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfmt607_next_u32s(ptr: *mut Sfmt607, out: *mut u32, count: usize) {
    unsafe {
        stream::fill::<Sfmt607, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt607_next_f32s(ptr: *mut Sfmt607, out: *mut f32, count: usize) {
    unsafe {
        stream::fill::<Sfmt607, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt607_rand_i32s(
    ptr: *mut Sfmt607,
    out: *mut i32,
    count: usize,
    min: i32,
    max: i32,
) {
    if min > max {
        return;
    }
    unsafe {
        stream::fill::<Sfmt607, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt607_rand_f32s(
    ptr: *mut Sfmt607,
    out: *mut f32,
    count: usize,
    min: f32,
    max: f32,
) {
    if !min.is_finite() || !max.is_finite() || min >= max {
        return;
    }
    unsafe {
        stream::fill::<Sfmt607, u32, _, 1, _, _>(
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
pub extern "C" fn sfmt1279_new(seed: u64) -> *mut Sfmt1279 {
    stream::new::<_, u32, 1>(Sfmt1279::new(seed as u32))
}

/// Frees an opaque handle; null is accepted.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfmt1279_free(ptr: *mut Sfmt1279) {
    unsafe { stream::free::<Sfmt1279, u32, 1>(ptr) }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfmt1279_next_u32s(ptr: *mut Sfmt1279, out: *mut u32, count: usize) {
    unsafe {
        stream::fill::<Sfmt1279, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt1279_next_f32s(ptr: *mut Sfmt1279, out: *mut f32, count: usize) {
    unsafe {
        stream::fill::<Sfmt1279, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt1279_rand_i32s(
    ptr: *mut Sfmt1279,
    out: *mut i32,
    count: usize,
    min: i32,
    max: i32,
) {
    if min > max {
        return;
    }
    unsafe {
        stream::fill::<Sfmt1279, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt1279_rand_f32s(
    ptr: *mut Sfmt1279,
    out: *mut f32,
    count: usize,
    min: f32,
    max: f32,
) {
    if !min.is_finite() || !max.is_finite() || min >= max {
        return;
    }
    unsafe {
        stream::fill::<Sfmt1279, u32, _, 1, _, _>(
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
pub extern "C" fn sfmt2281_new(seed: u64) -> *mut Sfmt2281 {
    stream::new::<_, u32, 1>(Sfmt2281::new(seed as u32))
}

/// Frees an opaque handle; null is accepted.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfmt2281_free(ptr: *mut Sfmt2281) {
    unsafe { stream::free::<Sfmt2281, u32, 1>(ptr) }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfmt2281_next_u32s(ptr: *mut Sfmt2281, out: *mut u32, count: usize) {
    unsafe {
        stream::fill::<Sfmt2281, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt2281_next_f32s(ptr: *mut Sfmt2281, out: *mut f32, count: usize) {
    unsafe {
        stream::fill::<Sfmt2281, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt2281_rand_i32s(
    ptr: *mut Sfmt2281,
    out: *mut i32,
    count: usize,
    min: i32,
    max: i32,
) {
    if min > max {
        return;
    }
    unsafe {
        stream::fill::<Sfmt2281, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt2281_rand_f32s(
    ptr: *mut Sfmt2281,
    out: *mut f32,
    count: usize,
    min: f32,
    max: f32,
) {
    if !min.is_finite() || !max.is_finite() || min >= max {
        return;
    }
    unsafe {
        stream::fill::<Sfmt2281, u32, _, 1, _, _>(
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
pub extern "C" fn sfmt4253_new(seed: u64) -> *mut Sfmt4253 {
    stream::new::<_, u32, 1>(Sfmt4253::new(seed as u32))
}

/// Frees an opaque handle; null is accepted.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfmt4253_free(ptr: *mut Sfmt4253) {
    unsafe { stream::free::<Sfmt4253, u32, 1>(ptr) }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfmt4253_next_u32s(ptr: *mut Sfmt4253, out: *mut u32, count: usize) {
    unsafe {
        stream::fill::<Sfmt4253, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt4253_next_f32s(ptr: *mut Sfmt4253, out: *mut f32, count: usize) {
    unsafe {
        stream::fill::<Sfmt4253, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt4253_rand_i32s(
    ptr: *mut Sfmt4253,
    out: *mut i32,
    count: usize,
    min: i32,
    max: i32,
) {
    if min > max {
        return;
    }
    unsafe {
        stream::fill::<Sfmt4253, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt4253_rand_f32s(
    ptr: *mut Sfmt4253,
    out: *mut f32,
    count: usize,
    min: f32,
    max: f32,
) {
    if !min.is_finite() || !max.is_finite() || min >= max {
        return;
    }
    unsafe {
        stream::fill::<Sfmt4253, u32, _, 1, _, _>(
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
pub extern "C" fn sfmt11213_new(seed: u64) -> *mut Sfmt11213 {
    stream::new::<_, u32, 1>(Sfmt11213::new(seed as u32))
}

/// Frees an opaque handle; null is accepted.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfmt11213_free(ptr: *mut Sfmt11213) {
    unsafe { stream::free::<Sfmt11213, u32, 1>(ptr) }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfmt11213_next_u32s(ptr: *mut Sfmt11213, out: *mut u32, count: usize) {
    unsafe {
        stream::fill::<Sfmt11213, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt11213_next_f32s(ptr: *mut Sfmt11213, out: *mut f32, count: usize) {
    unsafe {
        stream::fill::<Sfmt11213, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt11213_rand_i32s(
    ptr: *mut Sfmt11213,
    out: *mut i32,
    count: usize,
    min: i32,
    max: i32,
) {
    if min > max {
        return;
    }
    unsafe {
        stream::fill::<Sfmt11213, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt11213_rand_f32s(
    ptr: *mut Sfmt11213,
    out: *mut f32,
    count: usize,
    min: f32,
    max: f32,
) {
    if !min.is_finite() || !max.is_finite() || min >= max {
        return;
    }
    unsafe {
        stream::fill::<Sfmt11213, u32, _, 1, _, _>(
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
pub extern "C" fn sfmt44497_new(seed: u64) -> *mut Sfmt44497 {
    stream::new::<_, u32, 1>(Sfmt44497::new(seed as u32))
}

/// Frees an opaque handle; null is accepted.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfmt44497_free(ptr: *mut Sfmt44497) {
    unsafe { stream::free::<Sfmt44497, u32, 1>(ptr) }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfmt44497_next_u32s(ptr: *mut Sfmt44497, out: *mut u32, count: usize) {
    unsafe {
        stream::fill::<Sfmt44497, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt44497_next_f32s(ptr: *mut Sfmt44497, out: *mut f32, count: usize) {
    unsafe {
        stream::fill::<Sfmt44497, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt44497_rand_i32s(
    ptr: *mut Sfmt44497,
    out: *mut i32,
    count: usize,
    min: i32,
    max: i32,
) {
    if min > max {
        return;
    }
    unsafe {
        stream::fill::<Sfmt44497, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt44497_rand_f32s(
    ptr: *mut Sfmt44497,
    out: *mut f32,
    count: usize,
    min: f32,
    max: f32,
) {
    if !min.is_finite() || !max.is_finite() || min >= max {
        return;
    }
    unsafe {
        stream::fill::<Sfmt44497, u32, _, 1, _, _>(
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
pub extern "C" fn sfmt86243_new(seed: u64) -> *mut Sfmt86243 {
    stream::new::<_, u32, 1>(Sfmt86243::new(seed as u32))
}

/// Frees an opaque handle; null is accepted.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfmt86243_free(ptr: *mut Sfmt86243) {
    unsafe { stream::free::<Sfmt86243, u32, 1>(ptr) }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfmt86243_next_u32s(ptr: *mut Sfmt86243, out: *mut u32, count: usize) {
    unsafe {
        stream::fill::<Sfmt86243, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt86243_next_f32s(ptr: *mut Sfmt86243, out: *mut f32, count: usize) {
    unsafe {
        stream::fill::<Sfmt86243, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt86243_rand_i32s(
    ptr: *mut Sfmt86243,
    out: *mut i32,
    count: usize,
    min: i32,
    max: i32,
) {
    if min > max {
        return;
    }
    unsafe {
        stream::fill::<Sfmt86243, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt86243_rand_f32s(
    ptr: *mut Sfmt86243,
    out: *mut f32,
    count: usize,
    min: f32,
    max: f32,
) {
    if !min.is_finite() || !max.is_finite() || min >= max {
        return;
    }
    unsafe {
        stream::fill::<Sfmt86243, u32, _, 1, _, _>(
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
pub extern "C" fn sfmt132049_new(seed: u64) -> *mut Sfmt132049 {
    stream::new::<_, u32, 1>(Sfmt132049::new(seed as u32))
}

/// Frees an opaque handle; null is accepted.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfmt132049_free(ptr: *mut Sfmt132049) {
    unsafe { stream::free::<Sfmt132049, u32, 1>(ptr) }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfmt132049_next_u32s(ptr: *mut Sfmt132049, out: *mut u32, count: usize) {
    unsafe {
        stream::fill::<Sfmt132049, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt132049_next_f32s(ptr: *mut Sfmt132049, out: *mut f32, count: usize) {
    unsafe {
        stream::fill::<Sfmt132049, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt132049_rand_i32s(
    ptr: *mut Sfmt132049,
    out: *mut i32,
    count: usize,
    min: i32,
    max: i32,
) {
    if min > max {
        return;
    }
    unsafe {
        stream::fill::<Sfmt132049, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt132049_rand_f32s(
    ptr: *mut Sfmt132049,
    out: *mut f32,
    count: usize,
    min: f32,
    max: f32,
) {
    if !min.is_finite() || !max.is_finite() || min >= max {
        return;
    }
    unsafe {
        stream::fill::<Sfmt132049, u32, _, 1, _, _>(
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
pub extern "C" fn sfmt216091_new(seed: u64) -> *mut Sfmt216091 {
    stream::new::<_, u32, 1>(Sfmt216091::new(seed as u32))
}

/// Frees an opaque handle; null is accepted.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfmt216091_free(ptr: *mut Sfmt216091) {
    unsafe { stream::free::<Sfmt216091, u32, 1>(ptr) }
}

/// Fills the next values, preserving the stream across calls and alignments.
/// # Safety
/// Rejected fill inputs listed in [`crate::cabi`] are accepted without access.
/// Otherwise the handle must come from the matching constructor and be live and exclusive.
/// Nonempty output buffers must be valid, naturally aligned, and disjoint from the
/// handle; count must fit the allocation. Free consumes the handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfmt216091_next_u32s(ptr: *mut Sfmt216091, out: *mut u32, count: usize) {
    unsafe {
        stream::fill::<Sfmt216091, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt216091_next_f32s(ptr: *mut Sfmt216091, out: *mut f32, count: usize) {
    unsafe {
        stream::fill::<Sfmt216091, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt216091_rand_i32s(
    ptr: *mut Sfmt216091,
    out: *mut i32,
    count: usize,
    min: i32,
    max: i32,
) {
    if min > max {
        return;
    }
    unsafe {
        stream::fill::<Sfmt216091, u32, _, 1, _, _>(
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
pub unsafe extern "C" fn sfmt216091_rand_f32s(
    ptr: *mut Sfmt216091,
    out: *mut f32,
    count: usize,
    min: f32,
    max: f32,
) {
    if !min.is_finite() || !max.is_finite() || min >= max {
        return;
    }
    unsafe {
        stream::fill::<Sfmt216091, u32, _, 1, _, _>(
            ptr,
            out,
            count,
            |r| [Rng::nextu(r)],
            |r| r.randf(min, max),
        )
    }
}
