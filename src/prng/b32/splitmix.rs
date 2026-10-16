use wrapn::{wrap, wu32};

use crate::rng::Rng;

/// SplitMix32 32-bit RNG implementation.
///
/// Fast 32-bit finalizer-based PRNG commonly used to seed other generators.
/// Uses a single 32-bit state word advanced by the golden-ratio constant.
///
/// # Example
/// ```
/// use urng::{Rng, SplitMix32};
///
/// let mut rng = SplitMix32::new(1);
/// let _ = rng.nextu();
/// ```
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct SplitMix32 {
    state: wu32,
}

const A: u64 = 0xFF51_AFD7_ED55_8CCD;
const B: u64 = 0xC4CE_B9FE_1A85_EC53;

impl SplitMix32 {
    /// Creates a new `SplitMix32` instance with the given seed.
    pub const fn new(seed: u32) -> Self {
        Self { state: wrap!(seed) }
    }

    #[inline(always)]
    pub(crate) const fn nextu_const(&mut self) -> u32 {
        self.state.0.0 = self.state.0.0.wrapping_add(0x9E3779B9);

        Self::compute(self.state.0.0)
    }

    pub(crate) const fn compute(state: u32) -> u32 {
        let mut z = state as u64;
        z = (z ^ (z >> 16)).wrapping_mul(A);
        z = (z ^ (z >> 16)).wrapping_mul(B);
        (z ^ (z >> 16)) as u32
    }
}

impl Rng for SplitMix32 {
    type Word = u32;

    #[inline]
    fn nextu(&mut self) -> Self::Word {
        self.nextu_const()
    }
}

#[cfg(feature = "simd")]
pub use simd::*;

#[cfg(feature = "simd")]
pub mod simd {
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    #[cfg(target_arch = "x86_64")]
    use crate::{RngV, SplitMix32};

    pub const SPLITMIX32_GAMMA: u32 = 0x9E37_79B9;

    /// 16-way SIMD implementation of SplitMix32 32-bit RNG.
    /// This implementation uses AVX-512F instructions to generate 16 random numbers in parallel.
    ///
    /// # Example
    /// ```no_run
    /// use urng::{RngV, SplitMix32x16};
    /// unsafe {
    ///     let mut rng = SplitMix32x16::new(1);
    ///     let _ = rng.nextuv();
    /// }
    /// ```
    #[cfg(target_arch = "x86_64")]
    #[repr(C, align(64))]
    pub struct SplitMix32x16 {
        pub(crate) state: __m512i,
    }

    #[cfg(target_arch = "x86_64")]
    impl SplitMix32x16 {
        /// Creates a new `SplitMix32x16` instance with the given seed.
        ///
        /// # Safety
        ///
        /// The caller must ensure the CPU supports the `avx512f` target feature.
        #[target_feature(enable = "avx512f")]
        pub unsafe fn new(seed: u32) -> Self {
            let lanes: [u32; 16] = std::array::from_fn(|i| {
                seed.wrapping_add((i as u32).wrapping_mul(SPLITMIX32_GAMMA))
            });
            Self {
                state: unsafe { std::mem::transmute(lanes) },
            }
        }

        /// Computes the SplitMix32 output for 16 lanes at once.
        ///
        /// # Safety
        ///
        /// The caller must ensure the CPU supports the `avx512f` target feature.
        #[target_feature(enable = "avx512f")]
        pub unsafe fn compute(state: __m512i) -> __m512i {
            let lanes: [u32; 16] = unsafe { std::mem::transmute(state) };
            unsafe { std::mem::transmute(lanes.map(SplitMix32::compute)) }
        }
    }

    #[cfg(target_arch = "x86_64")]
    impl RngV for SplitMix32x16 {
        type Word = __m512i;

        fn nextuv(&mut self) -> Self::Word {
            let lanes: [u32; 16] = unsafe { std::mem::transmute(self.state) };
            let output = lanes.map(|v| SplitMix32::compute(v.wrapping_add(SPLITMIX32_GAMMA)));
            self.state = unsafe {
                std::mem::transmute(
                    lanes.map(|v| v.wrapping_add(SPLITMIX32_GAMMA.wrapping_mul(16))),
                )
            };
            unsafe { std::mem::transmute(output) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    crate::safe_test! { SplitMix32 }
}
