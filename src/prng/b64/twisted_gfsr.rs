use wrapn::{wrap, wusize};

use crate::{SplitMix32, rng::Rng};

// --- TwistedGFSR ---

/// Twisted GFSR with a 32-bit core, concatenating two outputs into each u64.
///
/// # Example
/// ```
/// use urng::{Rng, TwistedGFSR};
///
/// let mut rng = TwistedGFSR::new(0);
/// let _ = rng.nextu();
/// ```
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct TwistedGFSR {
    seed: [u32; N_GFSR],
    index: wusize,
}

const N_GFSR: usize = 25;
const M_GFSR: usize = 7;
const MAG01: [u32; 2] = [0x0, 0x8ebf_d028];

impl TwistedGFSR {
    /// Creates a new `TwistedGFSR` instance with the given seed.
    /// The existing seeding scheme uses the low 32 bits of `seed`.
    pub fn new(seed: u64) -> Self {
        let mut seedgen = SplitMix32::new(seed as u32);
        let seed = [0u32; N_GFSR].map(|_| seedgen.nextu_const());
        Self {
            seed,
            index: wrap!(N_GFSR),
        }
    }

    fn twist(&mut self) {
        for k in 0..(N_GFSR - M_GFSR) {
            self.seed[k] =
                self.seed[k + M_GFSR] ^ (self.seed[k] >> 1) ^ MAG01[(self.seed[k] & 1) as usize];
        }
        for k in (N_GFSR - M_GFSR)..N_GFSR {
            self.seed[k] = self.seed[k + M_GFSR - N_GFSR]
                ^ (self.seed[k] >> 1)
                ^ MAG01[(self.seed[k] & 1) as usize];
        }
        self.index = 0.into();
    }
}

impl Rng for TwistedGFSR {
    type Word = u64;

    fn nextu(&mut self) -> u64 {
        ((self.next_native() as u64) << 32) | self.next_native() as u64
    }
}
impl TwistedGFSR {
    fn next_native(&mut self) -> u32 {
        if self.index >= N_GFSR {
            self.twist();
        }
        let mut y = self.seed[*self.index];
        y ^= (y << 7) & 0x2b5b_2500;
        y ^= (y << 15) & 0xdb8b_0000;
        y ^= y >> 16;
        self.index += 1;
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    crate::safe_test! { TwistedGFSR }
}
