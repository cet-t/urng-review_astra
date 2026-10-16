use wrapn::{wrap, wu64};

use crate::{prng::b64::SplitMix64, rng::Rng};

/// Biski64 64-bit RNG implementation.
///
/// # Example
/// ```
/// use urng::{Rng, Biski64};
///
/// let mut rng = Biski64::new(1);
/// let _ = rng.nextu();
/// ```
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct Biski64 {
    fast_loop: wu64,
    mix: wu64,
    loop_mix: wu64,
}

impl Biski64 {
    /// Creates a new `Biski64` instance with the given seed.
    pub const fn new(seed: u64) -> Self {
        let mut seedgen = SplitMix64::new(seed);
        Self {
            fast_loop: wrap!(seedgen.nextu_const()),
            mix: wrap!(seedgen.nextu_const()),
            loop_mix: wrap!(seedgen.nextu_const()),
        }
    }
}

impl Rng for Biski64 {
    type Word = u64;

    #[inline(always)]
    fn nextu(&mut self) -> Self::Word {
        let output = self.mix + self.loop_mix;

        (self.fast_loop, self.mix, self.loop_mix) = (
            self.fast_loop + 0x9999999999999999,
            self.mix.rotate_left(16) + self.loop_mix.rotate_left(40),
            self.fast_loop ^ self.mix,
        );

        *output
    }
}

#[cfg(feature = "simd")]
pub use simd::*;

#[cfg(feature = "simd")]
pub mod simd {
    use std::arch::x86_64::*;

    use crate::_internal::{i2f_bits, u2f_01};
    use crate::SplitMix64;

    /// An 8-way SIMD Biski64 generator using AVX512 512-bit intrinsics.
    ///
    /// # Example
    /// ```no_run
    /// use urng::Biski64x8;
    ///
    /// let mut rng = Biski64x8::new(0);
    /// let _ = unsafe { rng.nextu() };
    /// ```
    #[cfg(all(feature = "simd", target_arch = "x86_64"))]
    #[repr(C, align(64))]
    pub struct Biski64x8 {
        fast_loop: __m512i,
        mix: __m512i,
        loop_mix: __m512i,
    }

    #[cfg(feature = "simd")]
    pub(crate) const INC: u64 = 0x9999999999999999;

    #[cfg(all(feature = "simd", target_arch = "x86_64"))]
    impl Biski64x8 {
        /// Creates a new `Biski64x8` from 8 independent seeds.
        ///
        /// # Safety
        /// Panics when AVX-512F is unavailable.
        #[inline(always)]
        pub fn new(seed: u64) -> Self {
            assert!(
                std::is_x86_feature_detected!("avx512f"),
                "AVX-512F required"
            );
            let mut sg = SplitMix64::new(seed);
            let mut fast_loop = [0u64; 8];
            let mut mix = [0u64; 8];
            let mut loop_mix = [0u64; 8];
            for i in 0..8 {
                fast_loop[i] = sg.nextu_const();
                mix[i] = sg.nextu_const();
                loop_mix[i] = sg.nextu_const();
            }
            unsafe {
                Self {
                    fast_loop: _mm512_loadu_si512(fast_loop.as_ptr() as *const __m512i),
                    mix: _mm512_loadu_si512(mix.as_ptr() as *const __m512i),
                    loop_mix: _mm512_loadu_si512(loop_mix.as_ptr() as *const __m512i),
                }
            }
        }

        /// Generates 8 random `u64` values simultaneously.
        ///
        /// # Safety
        /// The caller must ensure AVX-512F is available.
        #[inline]
        #[target_feature(enable = "avx512f")]
        pub unsafe fn nextu(&mut self) -> [u64; 8] {
            unsafe {
                // let output = self.mix.wrapping_add(self.loop_mix);
                let output = _mm512_add_epi64(self.mix, self.loop_mix);

                let inc = _mm512_set1_epi64(INC as i64);
                let fast_loop = _mm512_add_epi64(self.fast_loop, inc);
                let mix = _mm512_add_epi64(
                    _mm512_rol_epi64(self.mix, 16),
                    _mm512_rol_epi64(self.loop_mix, 40),
                );
                self.loop_mix = _mm512_xor_si512(self.fast_loop, self.mix);
                self.fast_loop = fast_loop;
                self.mix = mix;

                let mut res = [0u64; 8];
                _mm512_storeu_si512(res.as_mut_ptr() as *mut __m512i, output);
                res
            }
        }

        /// Generates 8 random `f64` values in [0, 1) and writes them to `out`.
        ///
        /// # Safety
        ///
        /// The caller must ensure the CPU supports the `avx512f` target feature.
        #[inline]
        #[target_feature(enable = "avx512f")]
        pub unsafe fn nextf(&mut self) -> [f64; 8] {
            unsafe {
                let u = self.nextu();
                let mut out = [0f64; 8];
                for i in 0..8 {
                    out[i] = u2f_01!(f64, 64, u[i]);
                }
                out
            }
        }

        /// Generates 8 random `i64` values in [min, max].
        ///
        /// # Safety
        ///
        /// The caller must ensure the CPU supports the `avx512f` target feature.
        #[inline]
        #[target_feature(enable = "avx512f")]
        pub unsafe fn randi(&mut self, min: i64, max: i64) -> [i64; 8] {
            unsafe { crate::rng::array_randi(|| self.nextu(), min, max) }
        }

        /// Generates 8 random `f64` values in [min, max) and writes them to `out`.
        ///
        /// # Safety
        ///
        /// The caller must ensure the CPU supports the `avx512f` target feature.
        #[inline]
        #[target_feature(enable = "avx512f")]
        pub unsafe fn randf(&mut self, min: f64, max: f64) -> [f64; 8] {
            unsafe { self.nextu().map(|x| crate::Word::to_randf(x, min, max)) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    crate::safe_test! { Biski64 }

    #[cfg(all(feature = "simd", target_feature = "avx512f"))]
    crate::unsafe_test! { nextu, nextf; Biski64x8 }
}

#[cfg(all(test, feature = "simd", target_arch = "x86_64"))]
mod regression {
    use super::*;
    #[test]
    fn all_lanes_match_scalar_and_independent_reference() {
        if !std::is_x86_feature_detected!("avx512f") {
            return;
        }
        for seed in [0, 1, 42, u64::MAX] {
            let mut sg = seed;
            let initial: [[u64; 3]; 8] = std::array::from_fn(|_| {
                [
                    crate::test_reference::sm64(&mut sg),
                    crate::test_reference::sm64(&mut sg),
                    crate::test_reference::sm64(&mut sg),
                ]
            });
            let mut refs = initial;
            let mut scalars = std::array::from_fn::<_, 8, _>(|i| Biski64 {
                fast_loop: wrap!(initial[i][0]),
                mix: wrap!(initial[i][1]),
                loop_mix: wrap!(initial[i][2]),
            });
            let mut vector = Biski64x8::new(seed);
            for _ in 0..1024 {
                for (i, word) in unsafe { vector.nextu() }.into_iter().enumerate() {
                    let expected = crate::test_reference::biski(&mut refs[i]);
                    assert_eq!(word, expected);
                    assert_eq!(scalars[i].nextu(), expected);
                }
            }
        }
    }
}
