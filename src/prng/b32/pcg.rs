use wrapn::{wrap, wu64};

use crate::{_internal::sm64_from_seed32, Rng};

// --- Pcg32 ---

/// A PCG (Permuted Congruential Generator) random number generator.
///
/// This implementation uses the PCG-XSH-RR algorithm with 64-bit state and 32-bit output.
///
/// # Example
/// ```
/// use urng::{Rng, Pcg32};
///
/// let mut rng = Pcg32::new(1);
/// let _ = rng.nextu();
/// ```
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct Pcg32 {
    state: wu64,
    inc: wu64,
}

impl Pcg32 {
    #[cfg(test)]
    pub(crate) fn from_raw_for_test(state: u64, inc: u64) -> Self {
        Self {
            state: wrap!(state),
            inc: wrap!(inc),
        }
    }
    /// Creates a new `Pcg32` instance with the given seed.
    pub const fn new(seed: u32) -> Self {
        let mut seedgen = sm64_from_seed32!(seed);
        Pcg32 {
            state: wrap!(seedgen.nextu_const()),
            inc: wrap!(seedgen.nextu_const() | 1),
        }
    }
}

impl Rng for Pcg32 {
    type Word = u32;

    #[inline]
    fn nextu(&mut self) -> Self::Word {
        let oldstate = self.state;
        self.state = oldstate * 6364136223846793005 + self.inc;
        let xorshifted = (((oldstate >> 18) ^ oldstate) >> 27).cast::<u32>();
        let rot = (oldstate >> 59).cast::<u32>();
        *xorshifted.rotate_right(*rot)
    }
}

// --- Pcg32x8 (AVX-512) ---

#[cfg(feature = "simd")]
pub use simd::*;

#[cfg(feature = "simd")]
pub mod simd {
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    use crate::RngV;

    pub const PCG32X8_LANE: usize = 8;

    /// 8-way SIMD implementation of PCG (Permuted Congruential Generator) 32-bit RNG.
    /// This implementation uses AVX-512F instructions to generate 8 random numbers in parallel.
    ///
    /// # Example
    /// ```no_run
    /// use urng::Pcg32x8;
    ///
    /// unsafe {
    ///     let mut rng = Pcg32x8::new(1);
    ///     let _ = rng.nextu();
    /// }
    /// ```
    #[cfg(target_arch = "x86_64")]
    #[repr(C, align(64))]
    pub struct Pcg32x8 {
        pub(crate) state: __m512i,
        pub(crate) inc: __m512i,
    }

    #[cfg(target_arch = "x86_64")]
    impl Pcg32x8 {
        /// Creates a new `Pcg32x8` instance with 8 independent PCG32 streams.
        /// Requires AVX-512F support.
        ///
        /// # Safety
        ///
        /// Must only be called on a CPU that supports AVX-512F.
        #[target_feature(enable = "avx512f")]
        pub unsafe fn new(seed: u32) -> Self {
            let mut seedgen = crate::sm64_from_seed32!(seed);

            let mut state = [0u64; PCG32X8_LANE];
            state.iter_mut().for_each(|v| *v = seedgen.nextu_const());

            let mut inc = [0u64; PCG32X8_LANE];
            inc.iter_mut().for_each(|v| *v = seedgen.nextu_const() | 1);

            unsafe {
                Pcg32x8 {
                    state: _mm512_loadu_si512(state.as_ptr() as _),
                    inc: _mm512_loadu_si512(inc.as_ptr() as _),
                }
            }
        }

        /// Advances all 8 PCG32 streams and returns their outputs.
        ///
        /// # Safety
        ///
        /// Must only be called on a CPU that supports AVX-512F.
        #[allow(unsafe_op_in_unsafe_fn)]
        #[target_feature(enable = "avx512f")]
        pub unsafe fn nextu(&mut self) -> [u32; PCG32X8_LANE] {
            let mult_lo = _mm512_set1_epi64(0x4C957F2D_i64);
            let mult_hi = _mm512_set1_epi64(0x5851F42D_i64);
            let mask32 = _mm512_set1_epi64(0xFFFFFFFF_i64);
            let out256 = Self::step_u32(&mut self.state, self.inc, mult_lo, mult_hi, mask32);
            std::mem::transmute(out256)
        }

        #[cfg(target_arch = "x86_64")]
        #[inline]
        #[allow(unsafe_op_in_unsafe_fn)]
        #[target_feature(enable = "avx512f")]
        pub(crate) unsafe fn step_u32(
            state: &mut __m512i,
            inc: __m512i,
            mult_lo: __m512i,
            mult_hi: __m512i,
            mask32: __m512i,
        ) -> __m256i {
            let oldstate = *state;
            let state_hi = _mm512_srli_epi64(oldstate, 32);
            let prod_lo = _mm512_mul_epu32(oldstate, mult_lo);
            let cross = _mm512_add_epi64(
                _mm512_mul_epu32(state_hi, mult_lo),
                _mm512_mul_epu32(oldstate, mult_hi),
            );
            *state = _mm512_add_epi64(_mm512_add_epi64(prod_lo, _mm512_slli_epi64(cross, 32)), inc);

            let xs = _mm512_srli_epi64(
                _mm512_xor_si512(_mm512_srli_epi64(oldstate, 18), oldstate),
                27,
            );
            let rot = _mm512_srli_epi64(oldstate, 59);
            let rotated = _mm512_rorv_epi32(_mm512_and_si512(xs, mask32), rot);
            _mm512_cvtepi64_epi32(rotated)
        }
    }

    impl RngV for Pcg32x8 {
        type Word = __m256i;

        #[inline]
        fn nextuv(&mut self) -> Self::Word {
            unsafe {
                let mult_lo = _mm512_set1_epi64(0x4C957F2D_i64);
                let mult_hi = _mm512_set1_epi64(0x5851F42D_i64);
                let mask32 = _mm512_set1_epi64(0xFFFFFFFF_i64);
                Self::step_u32(&mut self.state, self.inc, mult_lo, mult_hi, mask32)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    crate::safe_test! { Pcg32 }
}

#[cfg(test)]
mod regression {
    use super::*;
    #[test]
    fn official_pcg_demo_vector_and_odd_increments() {
        // pcg32_srandom_r(initstate=42, initseq=54), pcg-c-basic demo.
        let mut r = Pcg32 {
            state: wrap!(0),
            inc: wrap!(109),
        };
        let _ = r.nextu();
        r.state += 42;
        let _ = r.nextu();
        assert_eq!(
            std::array::from_fn::<_, 6, _>(|_| r.nextu()),
            [
                0xa15c02b7, 0x7b47f409, 0xba1d3330, 0x83d2f293, 0xbfa4784b, 0xcbed606e
            ]
        );
        for seed in 0..1024 {
            assert_eq!(*Pcg32::new(seed).inc & 1, 1);
        }
    }
    #[cfg(all(feature = "simd", target_arch = "x86_64"))]
    #[test]
    fn simd_all_lanes_match_scalar_and_reference() {
        if !std::is_x86_feature_detected!("avx512f") {
            return;
        }
        unsafe {
            let mut r = Pcg32x8::new(42);
            let states: [u64; 8] = std::mem::transmute(r.state);
            let inc: [u64; 8] = std::mem::transmute(r.inc);
            let mut refs = states;
            let mut scalars = std::array::from_fn::<_, 8, _>(|i| Pcg32 {
                state: wrap!(states[i]),
                inc: wrap!(inc[i]),
            });
            assert!(inc.into_iter().all(|x| x & 1 == 1));
            for _ in 0..1024 {
                for (i, word) in r.nextu().into_iter().enumerate() {
                    let expected = crate::test_reference::pcg(&mut refs[i], inc[i]);
                    assert_eq!(word, expected);
                    assert_eq!(scalars[i].nextu(), expected);
                }
            }
        }
    }
}
