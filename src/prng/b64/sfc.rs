use wrapn::{wrap, wu64};

use crate::prng::b64::SplitMix64;
use crate::rng::Rng;

// --- Sfc64 ---

/// SFC (Small Fast Chaotic) 64-bit RNG implementation.
///
/// # Example
/// ```
/// use urng::{Rng, Sfc64};
///
/// let mut rng = Sfc64::new(1);
/// let _ = rng.nextu();
/// ```
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct Sfc64 {
    a: wu64,
    b: wu64,
    c: wu64,
    counter: wu64,
}

impl Sfc64 {
    /// Creates a new `Sfc64` instance with the given seed.
    pub const fn new(seed: u64) -> Self {
        let mut seedgen = SplitMix64::new(seed);
        Self {
            a: wrap!(seedgen.nextu_const()),
            b: wrap!(seedgen.nextu_const()),
            c: wrap!(seedgen.nextu_const()),
            counter: wrap!(1),
        }
    }
}

impl Rng for Sfc64 {
    type Word = u64;

    #[inline(always)]
    fn nextu(&mut self) -> Self::Word {
        let res = self.a + self.b + self.counter;
        self.a = self.b ^ (self.b >> 11);
        self.b = self.c + (self.c << 3);
        self.c = self.c.rotate_left(24) + res;

        self.counter += 1;
        *res
    }
}

#[cfg(feature = "simd")]
pub use simd::*;

#[cfg(feature = "simd")]
pub mod simd {
    use std::arch::x86_64::*;

    use crate::_internal::{i2f_bits, u2f_01};
    use crate::prng::b64::SplitMix64;

    // --- Sfc64x4 (AVX2) ---

    /// An 8-way SIMD SFC64 generator using AVX-512F intrinsics.
    ///
    /// # Example
    /// ```no_run
    /// use urng::Sfc64x8;
    ///
    /// let mut rng = Sfc64x8::new(0);
    /// let _ = unsafe { rng.nextu() };
    /// ```
    #[cfg(all(feature = "simd", target_arch = "x86_64"))]
    #[repr(C, align(64))]
    pub struct Sfc64x8 {
        a: __m512i,
        b: __m512i,
        c: __m512i,
        counter: __m512i,
    }

    #[cfg(all(feature = "simd", target_arch = "x86_64"))]
    impl Sfc64x8 {
        /// Creates a new `Sfc64x8` from 8 independent seeds.
        ///
        /// # Safety
        /// Panics when AVX-512F is unavailable.
        #[inline(always)]
        pub fn new(seed: u64) -> Self {
            assert!(
                std::is_x86_feature_detected!("avx512f"),
                "AVX-512F required"
            );
            let mut a = [0u64; 8];
            let mut b = [0u64; 8];
            let mut c = [0u64; 8];
            let mut sg = SplitMix64::new(seed);
            for i in 0..8 {
                a[i] = sg.nextu_const();
                b[i] = sg.nextu_const();
                c[i] = sg.nextu_const();
            }
            unsafe {
                Self {
                    a: _mm512_loadu_si512(a.as_ptr() as *const __m512i),
                    b: _mm512_loadu_si512(b.as_ptr() as *const __m512i),
                    c: _mm512_loadu_si512(c.as_ptr() as *const __m512i),
                    counter: _mm512_set1_epi64(1),
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
                let one = _mm512_set1_epi64(1);

                // res = a + b + counter
                let res = _mm512_add_epi64(_mm512_add_epi64(self.a, self.b), self.counter);

                // a = b ^ (b >> 11)
                self.a = _mm512_xor_si512(self.b, _mm512_srli_epi64(self.b, 11));

                // b = c + (c << 3)
                self.b = _mm512_add_epi64(self.c, _mm512_slli_epi64(self.c, 3));

                // c = rotate_left(old_c, 24) + result
                self.c = _mm512_add_epi64(_mm512_rol_epi64(self.c, 24), res);

                // counter += 1
                self.counter = _mm512_add_epi64(self.counter, one);

                // Store 8 results
                let mut out = [0u64; 8];
                _mm512_storeu_si512(out.as_mut_ptr() as *mut __m512i, res);
                out
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

    crate::safe_test! { Sfc64 }

    #[cfg(all(feature = "simd", target_feature = "avx512f"))]
    crate::unsafe_test! { nextu, nextf; Sfc64x8 }
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
            let initial: [[u64; 4]; 8] = std::array::from_fn(|_| {
                [
                    crate::test_reference::sm64(&mut sg),
                    crate::test_reference::sm64(&mut sg),
                    crate::test_reference::sm64(&mut sg),
                    1,
                ]
            });
            let mut refs = initial;
            let mut scalars = std::array::from_fn::<_, 8, _>(|i| Sfc64 {
                a: wrap!(initial[i][0]),
                b: wrap!(initial[i][1]),
                c: wrap!(initial[i][2]),
                counter: wrap!(initial[i][3]),
            });
            let mut vector = Sfc64x8::new(seed);
            for _ in 0..1024 {
                for (i, word) in unsafe { vector.nextu() }.into_iter().enumerate() {
                    let expected = crate::test_reference::sfc(&mut refs[i]);
                    assert_eq!(word, expected);
                    assert_eq!(scalars[i].nextu(), expected);
                }
            }
        }
    }
}
