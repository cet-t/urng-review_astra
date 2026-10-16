#[cfg(feature = "simd")]
use std::arch::x86_64::*;

use wrapn::{wrap, wu32, wusize};

#[cfg(feature = "simd")]
use crate::_internal::{i2f_bits, u2f_01};
use crate::prng::b32::SplitMix32;
use crate::rng::Rng;

// --- Philox32 ---

/// A Philox 4x32 random number generator.
///
/// This is a counter-based RNG suitable for parallel applications. Implements
/// [`Rng`] directly: each call to [`Rng::nextu`] hands out one `u32` from
/// an internal 4-word buffer, recomputing a fresh block every 4th call.
///
/// # Examples
///
/// ```
/// use urng::{Rng, Philox32};
///
/// let mut rng = Philox32::new(1);
/// let _: u32 = rng.nextu();
/// ```
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct Philox32 {
    pub(crate) c: [wu32; 4],
    pub(crate) k: [wu32; 2],
    pub(crate) buf: [wu32; 4],
    pub(crate) pos: wusize,
}

impl Philox32 {
    /// Creates a new `Philox32` instance seeded with the given value.
    pub fn new(seed: u32) -> Self {
        let mut seedgen = SplitMix32::new(seed);
        Self {
            c: wrap![
                seedgen.nextu(),
                seedgen.nextu(),
                seedgen.nextu(),
                seedgen.nextu(),
            ],
            k: wrap![seedgen.nextu(), seedgen.nextu()],
            buf: wrap![0; 4],
            pos: 4.into(),
        }
    }

    /// Computes Philox output from counter and key values (pure function).
    #[inline(always)]
    pub(crate) fn compute(c: [wu32; 4], k: [wu32; 2]) -> [wu32; 4] {
        // Random123 Philox4x32-10: nine key bumps between ten rounds.
        let mut x = c.map(|v| *v);
        let mut key = k.map(|v| *v);
        for round in 0..10 {
            let p0 = x[0] as u64 * 0xD2511F53;
            let p1 = x[2] as u64 * 0xCD9E8D57;
            x = [
                (p1 >> 32) as u32 ^ x[1] ^ key[0],
                p1 as u32,
                (p0 >> 32) as u32 ^ x[3] ^ key[1],
                p0 as u32,
            ];
            if round != 9 {
                key[0] = key[0].wrapping_add(0x9E3779B9);
                key[1] = key[1].wrapping_add(0xBB67AE85);
            }
        }
        x.map(Into::into)
    }

    /// Generates the next block of 4 random `u32` values in one call.
    ///
    /// This is the raw bulk-generation path (used internally to refill the
    /// scalar [`Rng::nextu`] buffer, and available directly for
    /// throughput-sensitive callers that want the whole block at once).
    #[inline(always)]
    pub fn next_raw(&mut self) -> [wu32; 4] {
        let out = Self::compute(self.c, self.k);
        self.c[0] += 1;
        if self.c[0] == 0 {
            self.c[1] += 1;
            if self.c[1] == 0 {
                self.c[2] += 1;
                if self.c[2] == 0 {
                    self.c[3] += 1;
                }
            }
        }
        out
    }
}

crate::_internal::impl_ring_rng32!(Philox32, 4, next_raw);

// --- Philox32x4-10 x4 ---

#[cfg(feature = "simd")]
#[allow(non_upper_case_globals)]
pub const PHILOX32x16: usize = 16;

/// A Philox 4x32x4 random number generator.
///
/// This is a counter-based RNG suitable for parallel applications.
///
/// # Example
/// ```no_run
/// use urng::Philox32x4x4;
///
/// unsafe {
///     let mut rng = Philox32x4x4::new(12345);
///     let _ = rng.nextu();
/// }
/// ```
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[repr(C, align(64))]
pub struct Philox32x4x4 {
    pub(crate) c: __m512i,
    pub(crate) k: __m512i,
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
impl Philox32x4x4 {
    /// Creates a new `Philox32x4x4` instance seeded with the given value.
    /// Requires AVX-512F support.
    ///
    /// # Safety
    ///
    /// Must only be called on a CPU that supports AVX-512F.
    #[target_feature(enable = "avx512f")]
    pub fn new(seed: u32) -> Self {
        let mut c = [0; PHILOX32x16];
        let mut k = [0; PHILOX32x16];

        let mut seedgen = SplitMix32::new(seed);
        c.iter_mut().for_each(|c| *c = seedgen.nextu());

        // [k0, 0, k1, 0]
        (0..PHILOX32x16).step_by(2).for_each(|i| {
            k[i] = seedgen.nextu();
        });

        unsafe {
            Self {
                c: _mm512_loadu_si512(c.as_ptr() as *const _),
                k: _mm512_loadu_si512(k.as_ptr() as *const _),
            }
        }
    }

    #[target_feature(enable = "avx512f")]
    pub(crate) fn compute(&mut self) -> [u32; PHILOX32x16] {
        let counters: [u32; 16] = unsafe { std::mem::transmute(self.c) };
        let keys: [u32; 16] = unsafe { std::mem::transmute(self.k) };
        let mut out = [0u32; 16];
        for lane in 0..4 {
            let base = lane * 4;
            let c = std::array::from_fn(|i| wrap!(counters[base + i]));
            let k = [wrap!(keys[base]), wrap!(keys[base + 2])];
            out[base..base + 4].copy_from_slice(&Philox32::compute(c, k).map(|v| *v));
        }
        out
    }

    /// Generates the next block of random numbers.
    ///
    /// # Safety
    ///
    /// The caller must ensure the CPU supports the `avx512f` target feature.
    #[target_feature(enable = "avx512f")]
    pub fn nextu(&mut self) -> [u32; PHILOX32x16] {
        let out = self.compute();

        // increment counter
        // [1, 1, 1, 1, 1, 1, 1, 1]
        let one = _mm512_set1_epi64(1);

        // lower 64 bits (indices 0, 2, 4, 6) +1
        let next_c = _mm512_mask_add_epi64(self.c, 0x55, self.c, one);

        // check overflow of lower 64 bits
        let eq_zero_mask = _mm512_cmpeq_epi64_mask(next_c, _mm512_setzero_si512());
        let carry_mask = (eq_zero_mask & 0x55) << 1;

        if carry_mask != 0 {
            // if overflow, add to upper 64 bits
            self.c = _mm512_mask_add_epi64(next_c, carry_mask, next_c, one);
        } else {
            self.c = next_c;
        }

        out
    }

    /// Generates 16 random `f32` values in the range [0, 1) using AVX-512.
    ///
    /// # Safety
    ///
    /// The caller must ensure the CPU supports the `avx512f` target feature.
    #[target_feature(enable = "avx512f")]
    pub fn nextf(&mut self) -> [f32; PHILOX32x16] {
        self.nextu().map(|x| u2f_01!(f32, 32, x))
    }

    /// Generates a random `i32` value in the range [min, max].
    ///
    /// # Safety
    ///
    /// The caller must ensure the CPU supports the `avx512f` target feature.
    #[target_feature(enable = "avx512f")]
    pub fn randi(&mut self, min: i32, max: i32) -> [i32; PHILOX32x16] {
        crate::rng::array_randi(|| self.nextu(), min, max)
    }

    /// Generates a random `f32` value in the range [min, max).
    ///
    /// # Safety
    ///
    /// The caller must ensure the CPU supports the `avx512f` target feature.
    #[target_feature(enable = "avx512f")]
    pub fn randf(&mut self, min: f32, max: f32) -> [f32; PHILOX32x16] {
        self.nextu().map(|x| crate::Word::to_randf(x, min, max))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    crate::safe_test! { Philox32 }
    #[cfg(all(feature = "simd", target_feature = "avx512f"))]
    crate::unsafe_test! { nextu, nextf; Philox32x4x4 }
}

#[cfg(test)]
mod regression {
    use super::*;
    #[test]
    fn random123_known_answer_and_counter_carry() {
        assert_eq!(
            Philox32::compute(wrap![0;4], wrap![0;2]).map(|v| *v),
            [0x6627e8d5, 0xe169c58d, 0xbc57ac4c, 0x9b00dbd8]
        );
        let mut r = Philox32::new(0);
        r.c = wrap![u32::MAX, u32::MAX, u32::MAX, 7];
        r.next_raw();
        assert_eq!(r.c, wrap![0, 0, 0, 8]);
    }
    #[cfg(all(feature = "simd", target_arch = "x86_64"))]
    #[test]
    fn all_four_streams_match_scalar_and_reference_at_carry() {
        if !std::is_x86_feature_detected!("avx512f") {
            return;
        }
        unsafe {
            let mut r = Philox32x4x4::new(42);
            let mut counters: [u32; 16] = std::mem::transmute(r.c);
            let keys: [u32; 16] = std::mem::transmute(r.k);
            for lane in 0..4 {
                counters[lane * 4..lane * 4 + 3].fill(u32::MAX);
            }
            r.c = std::mem::transmute(counters);
            let mut scalars = std::array::from_fn::<_, 4, _>(|i| Philox32 {
                c: std::array::from_fn(|j| wrap!(counters[i * 4 + j])),
                k: wrap![keys[i * 4], keys[i * 4 + 2]],
                buf: wrap![0;4],
                pos: wrap!(4),
            });
            for _ in 0..100 {
                let output = r.nextu();
                for lane in 0..4 {
                    let scalar = &mut scalars[lane];
                    let expected =
                        crate::test_reference::philox(scalar.c.map(|x| *x), scalar.k.map(|x| *x));
                    assert_eq!(&output[lane * 4..lane * 4 + 4], &expected);
                    assert_eq!(scalar.next_raw().map(|x| *x), expected);
                }
            }
        }
    }
}
