use wrapn::{wrap, wu32, wusize};

use crate::{Rng, SplitMix32, impl_ring_rng32};

/// Salsa stream cipher based RNG implementation, generic over the round count.
///
/// # Example
/// ```
/// use urng::{Rng, Salsa20};
///
/// let mut rng = Salsa20::new(12345);
/// let _ = rng.nextu();
/// ```
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct Salsa<const ROUNDS: usize> {
    x: [wu32; 16],
    exhausted: bool,

    pub(crate) buf: [wu32; 16],
    pub(crate) pos: wusize,
}

impl<const ROUNDS: usize> Salsa<ROUNDS> {
    /// Deterministic convenience constructor with only 32 bits of seed entropy.
    /// This constructor and Default are not suitable for secret keys.
    pub fn new(seed: u32) -> Self {
        let mut sg = SplitMix32::new(seed);
        let mut key = [0u8; 32];
        for chunk in key.chunks_exact_mut(4) {
            chunk.copy_from_slice(&sg.nextu().to_le_bytes());
        }
        Self::from_key_nonce(key, [0; 8], 0)
    }

    /// Constructs the standard state from a 256-bit key, nonce and block counter.
    /// Panics after the final counter block has been consumed, before repetition.
    pub fn from_key_nonce(key: [u8; 32], nonce: [u8; 8], counter: u64) -> Self {
        assert!(
            ROUNDS > 0 && ROUNDS % 2 == 0,
            "round count must be positive and even"
        );
        let mut x = wrap![0u32; 16];
        for (i, constant) in [0x61707865, 0x3320646e, 0x79622d32, 0x6b206574]
            .into_iter()
            .enumerate()
        {
            x[i * 5] = wrap!(constant);
        }
        for (index, bytes) in [1, 2, 3, 4, 11, 12, 13, 14]
            .into_iter()
            .zip(key.chunks_exact(4))
        {
            x[index] = wrap!(u32::from_le_bytes(bytes.try_into().unwrap()));
        }
        x[6] = wrap!(u32::from_le_bytes(nonce[..4].try_into().unwrap()));
        x[7] = wrap!(u32::from_le_bytes(nonce[4..].try_into().unwrap()));
        x[8] = wrap!(counter as u32);
        x[9] = wrap!((counter >> 32) as u32);
        Self {
            x,
            exhausted: false,
            buf: wrap![0u32;16],
            pos: wrap!(16),
        }
    }

    fn qr(mut a: wu32, mut b: wu32, mut c: wu32, mut d: wu32) -> [wu32; 4] {
        b ^= (a + d).rotate_left(7);
        c ^= (b + a).rotate_left(9);
        d ^= (c + b).rotate_left(13);
        a ^= (d + c).rotate_left(18);
        [a, b, c, d]
    }

    fn block(input: [wu32; 16]) -> [wu32; 16] {
        let mut x = input;
        for _ in 0..ROUNDS / 2 {
            [x[0], x[4], x[8], x[12]] = Self::qr(x[0], x[4], x[8], x[12]);
            [x[5], x[9], x[13], x[1]] = Self::qr(x[5], x[9], x[13], x[1]);
            [x[10], x[14], x[2], x[6]] = Self::qr(x[10], x[14], x[2], x[6]);
            [x[15], x[3], x[7], x[11]] = Self::qr(x[15], x[3], x[7], x[11]);

            [x[0], x[1], x[2], x[3]] = Self::qr(x[0], x[1], x[2], x[3]);
            [x[5], x[6], x[7], x[4]] = Self::qr(x[5], x[6], x[7], x[4]);
            [x[10], x[11], x[8], x[9]] = Self::qr(x[10], x[11], x[8], x[9]);
            [x[15], x[12], x[13], x[14]] = Self::qr(x[15], x[12], x[13], x[14]);
        }
        std::array::from_fn(|i| x[i] + input[i])
    }

    fn next_raw(&mut self) -> [wu32; 16] {
        assert!(!self.exhausted, "block counter exhausted");
        let output = Self::block(self.x);
        if *self.x[8] == u32::MAX && *self.x[9] == u32::MAX {
            self.exhausted = true;
        } else {
            self.x[8] += 1;
            if *self.x[8] == 0 {
                self.x[9] += 1;
            }
        }
        output
    }
}

impl<const ROUNDS: usize> Default for Salsa<ROUNDS> {
    fn default() -> Self {
        Self::new(crate::_internal::default_seed32())
    }
}

impl<const ROUNDS: usize> crate::Seed for Salsa<ROUNDS> {
    type Seed = u32;

    #[inline]
    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(seed)
    }
}

pub type Salsa8 = Salsa<8>;
impl_ring_rng32!(Salsa8, 16, next_raw);

pub type Salsa20 = Salsa<20>;
impl_ring_rng32!(Salsa20, 16, next_raw);

#[cfg(test)]
mod tests {
    use super::*;

    crate::safe_test! {
        Salsa8,
        Salsa20
    }
}

#[cfg(test)]
mod regression {
    use super::*;
    fn decode(hex: &str) -> [wu32; 16] {
        let bytes: Vec<_> = (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
            .collect();
        std::array::from_fn(|i| {
            wrap!(u32::from_le_bytes(
                bytes[i * 4..i * 4 + 4].try_into().unwrap()
            ))
        })
    }
    #[test]
    fn rfc7914_salsa8_core() {
        let input = decode(concat!(
            "7e879a214f3ec9867ca940e641718f26",
            "baee555b8c61c1b50df846116dcd3b1d",
            "ee24f319df9b3d8514121e4b5ac5aa32",
            "76021d2909c74829edebc68db8b8c25e"
        ));
        let expected = decode(concat!(
            "a41f859c6608cc993b81cacb020cef05",
            "044b2181a2fd337dfd7b1c6396682f29",
            "b4393168e3c9e6bcfe6bc5b7a06d96ba",
            "e424cc102c91745c24ad673dc7618f81"
        ));
        assert_eq!(Salsa8::block(input), expected);
    }
    #[test]
    fn counter_carry_and_exhaustion() {
        let mut r = Salsa20::from_key_nonce([0; 32], [0; 8], u32::MAX as u64);
        for _ in 0..16 {
            let _ = r.nextu();
        }
        let mut next = Salsa20::from_key_nonce([0; 32], [0; 8], 1u64 << 32);
        for _ in 0..32 {
            assert_eq!(r.nextu(), next.nextu());
        }
        let mut last = Salsa20::from_key_nonce([0; 32], [0; 8], u64::MAX);
        for _ in 0..16 {
            let _ = last.nextu();
        }
        assert!(std::panic::catch_unwind(move || last.nextu()).is_err());
    }
}
