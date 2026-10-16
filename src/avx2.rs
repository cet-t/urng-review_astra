//! AVX2-only replacements for intrinsics whose 256-bit spelling needs AVX-512VL.
use std::arch::x86_64::*;

#[target_feature(enable = "avx2")]
pub(crate) unsafe fn rol_epi32(v: __m256i, n: i32) -> __m256i {
    let left = _mm256_sllv_epi32(v, _mm256_set1_epi32(n));
    let right = _mm256_srlv_epi32(v, _mm256_set1_epi32(32 - n));
    _mm256_or_si256(left, right)
}
#[target_feature(enable = "avx2")]
pub(crate) unsafe fn loadu_epi32(ptr: *const i32) -> __m256i {
    unsafe { _mm256_loadu_si256(ptr.cast()) }
}
