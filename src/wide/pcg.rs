use ::wide::{u64x4, u64x8};

use crate::wide::RngW;
use crate::{Rng, SplitMix64};

macro_rules! impl_pcg32_variants {
    ($size:expr, $lanes:expr) => {
        ::pastey::paste! {
            #[doc = concat!("PCG32 (Permuted Congruential Generator) producing ", stringify!($size), " values per call via `wide` SIMD vectors.")]
            #[doc = ""]
            #[doc = "Portable-SIMD counterpart of [`crate::prng::b32::Pcg32`]. Uses the PCG-XSH-RR output"]
            #[doc = "function over wide `u64` state; each `nextu` call returns an array of `u32`."]
            #[doc = ""]
            #[doc = "# Example"]
            #[doc = "```"]
            #[doc = "use urng::wide::RngW;"]
            #[doc = concat!("use urng::wide::Pcg32x", stringify!($size), ";")]
            #[doc = ""]
            #[doc = concat!("let mut rng = Pcg32x", stringify!($size), "::new(0);")]
            #[doc = concat!("let _ = rng.nextu();")]
            #[doc = "```"]
            #[allow(dead_code)]
            #[repr(C, align(64))]
            pub struct [<Pcg32x $size>] {
                state: [<u64x $lanes>],
                inc: [<u64x $lanes>],
            }

            #[allow(dead_code)]
            impl [<Pcg32x $size>] {
                #[doc = "Creates a new generator, seeding every lane's state and increment from `seed`."]
                pub fn new(seed: u64) -> Self {
                    let mut seedgen = SplitMix64::new(seed);
                    Self {
                        state: [<u64x $lanes>]::from([0u64; $lanes].map(|_| seedgen.nextu())),
                        inc: [<u64x $lanes>]::from([0u64; $lanes].map(|_| seedgen.nextu() | 1)),
                    }
                }

                #[doc = "Advances one PCG32 stream (`state = state * MULT + inc`) and applies the XSH-RR output function."]
                #[inline(always)]
                fn step(state: &mut [<u64x $lanes>], inc: [<u64x $lanes>]) -> [u32; $lanes] {
                    let oldstate = *state;
                    *state = oldstate * 6364136223846793005u64 + inc;
                    let xorshifted: [u64; $lanes] =
                        (((oldstate >> 18u64) ^ oldstate) >> 27u64).to_array();
                    let rot: [u64; $lanes] = (oldstate >> 59u64).to_array();
                    std::array::from_fn(|i| (xorshifted[i] as u32).rotate_right(rot[i] as u32))
                }

            }

            impl RngW<$size> for [<Pcg32x $size>] {
                type Word = u32;

                #[doc = "Generates the next block of `u32` values, one per SIMD lane."]
                #[inline(always)]
                fn nextu(&mut self) -> [Self::Word; $size] {
                    bytemuck::cast(Self::step(&mut self.state, self.inc))
                }
            }
        }
    };
}

impl_pcg32_variants!(4, 4);
impl_pcg32_variants!(8, 8);

/// PCG32 producing 16 values per call by combining two [`Pcg32x8`] streams.
///
/// Portable-SIMD counterpart of [`crate::prng::b32::Pcg32`]. Each `nextu` call returns
/// a `[u32; 16]` by drawing 8 values from each underlying `Pcg32x8` lane-group.
///
/// # Example
/// ```
/// use urng::wide::{Pcg32x16, RngW};
///
/// let mut rng = Pcg32x16::new(0);
/// let v = rng.nextu();
/// assert_eq!(v.len(), 16);
/// ```
#[allow(dead_code)]
#[repr(C, align(64))]
pub struct Pcg32x16 {
    lo: Pcg32x8,
    hi: Pcg32x8,
}

#[allow(dead_code)]
impl Pcg32x16 {
    /// Creates a new generator, seeding the lower and upper `Pcg32x8` lane-groups from `seed`.
    pub fn new(seed: u64) -> Self {
        Self {
            lo: Pcg32x8::new(seed),
            hi: Pcg32x8::new(SplitMix64::compute(seed ^ 0x9E3779B97F4A7C15)),
        }
    }
}

impl RngW<16> for Pcg32x16 {
    type Word = u32;

    #[doc = "Generates the next 16 `u32` values by combining both `Pcg32x8` lane-groups."]
    #[inline(always)]
    fn nextu(&mut self) -> [u32; 16] {
        let lo = self.lo.nextu();
        let hi = self.hi.nextu();
        std::array::from_fn(|i| if i < 8 { lo[i] } else { hi[i - 8] })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_widths_match_scalar_and_reference_with_odd_increments() {
        macro_rules! check {
            ($rng:expr,$states:expr,$incs:expr,$n:expr) => {{
                let mut rng = $rng;
                let mut states = $states;
                let incs = $incs;
                assert!(incs.iter().all(|x| x & 1 == 1));
                let mut scalars = std::array::from_fn::<_, $n, _>(|i| {
                    crate::Pcg32::from_raw_for_test(states[i], incs[i])
                });
                for _ in 0..1024 {
                    for (i, word) in rng.nextu().into_iter().enumerate() {
                        let expected = crate::test_reference::pcg(&mut states[i], incs[i]);
                        assert_eq!(word, expected);
                        assert_eq!(scalars[i].nextu(), expected);
                    }
                }
            }};
        }
        let r = Pcg32x4::new(0);
        let (s, i) = (r.state.to_array(), r.inc.to_array());
        check!(r, s, i, 4);
        let r = Pcg32x8::new(0);
        let (s, i) = (r.state.to_array(), r.inc.to_array());
        check!(r, s, i, 8);
        let r = Pcg32x16::new(0);
        let s: [u64; 16] = std::array::from_fn(|i| {
            if i < 8 {
                r.lo.state.to_array()[i]
            } else {
                r.hi.state.to_array()[i - 8]
            }
        });
        let inc: [u64; 16] = std::array::from_fn(|i| {
            if i < 8 {
                r.lo.inc.to_array()[i]
            } else {
                r.hi.inc.to_array()[i - 8]
            }
        });
        check!(r, s, inc, 16);
    }

    crate::safe_test! {
        Pcg32x4,
        Pcg32x8,
        Pcg32x16
    }
}
