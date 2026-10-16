use wrapn::{wrap, wu64};

use crate::prng::b64::SplitMix64;
use crate::rng::Rng;

/// Xoshiro256++ 64-bit RNG implementation.
///
/// # Example
/// ```
/// use urng::{Rng, Xoshiro256Pp};
///
/// let mut rng = Xoshiro256Pp::new(1);
/// let _ = rng.nextu();
/// ```
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct Xoshiro256Pp {
    s: [wu64; 4],
}

impl Xoshiro256Pp {
    /// Creates a new `Xoshiro256Pp` instance with the given seed.
    pub const fn new(seed: u64) -> Self {
        let mut seedgen = SplitMix64::new(seed);
        Self {
            s: wrap![
                seedgen.nextu_const(),
                seedgen.nextu_const(),
                seedgen.nextu_const(),
                seedgen.nextu_const(),
            ],
        }
    }
}

impl Rng for Xoshiro256Pp {
    type Word = u64;

    #[inline]
    fn nextu(&mut self) -> Self::Word {
        let s = &mut self.s;
        let res = s[0] + (s[0] + s[3]).rotate_left(23);
        let t = s[1] << 17;

        s[2] ^= s[0];
        s[3] ^= s[1];
        s[1] ^= s[2];
        s[0] ^= s[3];
        s[2] ^= t;
        s[3] = s[3].rotate_left(45);

        *res
    }
}

/// Xoshiro256** 64-bit RNG implementation.
///
/// # Example
/// ```
/// use urng::{Rng, Xoshiro256Ss};
///
/// let mut rng = Xoshiro256Ss::new(1);
/// let _ = rng.nextu();
/// ```
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct Xoshiro256Ss {
    s: [wu64; 4],
}

impl Xoshiro256Ss {
    /// Creates a new `Xoshiro256Ss` instance with the given seed.
    pub const fn new(seed: u64) -> Self {
        let mut seedgen = SplitMix64::new(seed);
        Self {
            s: wrap![
                seedgen.nextu_const(),
                seedgen.nextu_const(),
                seedgen.nextu_const(),
                seedgen.nextu_const(),
            ],
        }
    }
}

impl Rng for Xoshiro256Ss {
    type Word = u64;

    #[inline]
    fn nextu(&mut self) -> Self::Word {
        let s = &mut self.s;
        let res = (s[1] * 5).rotate_left(7) * 9;
        let t = s[1] << 17;

        s[2] ^= s[0];
        s[3] ^= s[1];
        s[1] ^= s[2];
        s[0] ^= s[3];
        s[2] ^= t;
        s[3] = s[3].rotate_left(45);

        *res
    }
}

#[cfg(feature = "simd")]
pub use simd::*;

#[cfg(feature = "simd")]
pub mod simd {
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    use crate::prng::b64::SplitMix64;

    /// Two xoshiro256** streams, producing one word from each stream per call.
    /// The packed state is retained for layout compatibility. The transition
    /// uses scalar words so construction and generation require no AVX-512.
    #[cfg(target_arch = "x86_64")]
    #[repr(C, align(64))]
    pub struct Xoshiro256Ssx2 {
        pub(crate) s: __m512i,
    }

    impl Xoshiro256Ssx2 {
        /// Creates a new `Xoshiro256Ssx2` instance.
        #[cfg(target_arch = "x86_64")]
        pub fn new(seed: u64) -> Self {
            let mut seedgen = SplitMix64::new(seed);
            let s = [0u64; 8].map(|_| seedgen.nextu_const());

            Self {
                s: unsafe { std::mem::transmute(s) },
            }
        }

        /// Generates the next 2 random `u64` values.
        #[cfg(target_arch = "x86_64")]
        #[inline(always)]
        pub fn nextu(&mut self) -> [u64; 2] {
            let mut state: [u64; 8] = unsafe { std::mem::transmute(self.s) };
            let mut out = [0; 2];
            for (s, dst) in state.chunks_exact_mut(4).zip(out.iter_mut()) {
                *dst = s[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
                let t = s[1] << 17;
                s[2] ^= s[0];
                s[3] ^= s[1];
                s[1] ^= s[2];
                s[0] ^= s[3];
                s[2] ^= t;
                s[3] = s[3].rotate_left(45);
            }
            self.s = unsafe { std::mem::transmute(state) };
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    crate::safe_test! {
        Xoshiro256Pp,
        Xoshiro256Ss
    }

    #[cfg(all(feature = "simd", target_arch = "x86_64"))]
    #[test]
    fn packed_xoshiro_two_streams_match_reference_and_scalar() {
        // Blackman/Vigna xoshiro256** reference, state [1,2,3,4].
        let mut states = [[1u64, 2, 3, 4], [5, 6, 7, 8]];
        let mut scalar = states.map(|s| Xoshiro256Ss {
            s: s.map(Into::into),
        });
        let mut packed = Xoshiro256Ssx2 {
            s: unsafe { std::mem::transmute([1u64, 2, 3, 4, 5, 6, 7, 8]) },
        };
        for step in 0..1024 {
            let got = packed.nextu();
            if step == 0 {
                assert_eq!(got, [11520, 34560]);
            }
            for lane in 0..2 {
                let [a, b, c, d] = states[lane];
                let expected = b.wrapping_mul(5).rotate_left(7).wrapping_mul(9);
                states[lane] = [
                    a ^ d ^ b,
                    b ^ c ^ a,
                    c ^ a ^ (b << 17),
                    (d ^ b).rotate_left(45),
                ];
                assert_eq!(got[lane], expected);
                assert_eq!(scalar[lane].nextu(), expected);
            }
        }
    }
}
