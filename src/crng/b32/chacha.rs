use wrapn::{wrap, wu32, wusize};

use crate::{Rng, SplitMix32, impl_ring_rng32};

/// ChaCha stream cipher based RNG implementation, generic over the round count.
///
/// # Example
/// ```
/// use urng::{Rng, ChaCha20};
///
/// let mut rng = ChaCha20::new(12345);
/// let _ = rng.nextu();
/// ```
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct ChaCha<const ROUNDS: usize> {
    x: [wu32; 16],
    exhausted: bool,

    pub(crate) buf: [wu32; 16],
    pub(crate) pos: wusize,
}

impl<const ROUNDS: usize> ChaCha<ROUNDS> {
    /// Deterministic convenience constructor with only 32 bits of seed entropy.
    /// This constructor and Default are not suitable for secret keys.
    pub fn new(seed: u32) -> Self {
        let mut sg = SplitMix32::new(seed);
        let mut key = [0u8; 32];
        for chunk in key.chunks_exact_mut(4) {
            chunk.copy_from_slice(&sg.nextu().to_le_bytes());
        }
        Self::from_key_nonce(key, [0; 12], 0)
    }

    /// Constructs the standard state from a 256-bit key, nonce and block counter.
    /// Panics after the final counter block has been consumed, before repetition.
    pub fn from_key_nonce(key: [u8; 32], nonce: [u8; 12], counter: u32) -> Self {
        assert!(
            ROUNDS > 0 && ROUNDS % 2 == 0,
            "round count must be positive and even"
        );
        let mut x = wrap![0u32; 16];
        x[..4].copy_from_slice(&[0x61707865, 0x3320646e, 0x79622d32, 0x6b206574].map(Into::into));
        for (i, bytes) in key.chunks_exact(4).enumerate() {
            x[4 + i] = wrap!(u32::from_le_bytes(bytes.try_into().unwrap()));
        }
        x[12] = wrap!(counter);
        for (i, bytes) in nonce.chunks_exact(4).enumerate() {
            x[13 + i] = wrap!(u32::from_le_bytes(bytes.try_into().unwrap()));
        }
        Self {
            x,
            exhausted: false,
            buf: wrap![0u32;16],
            pos: wrap!(16),
        }
    }

    fn qr(mut a: wu32, mut b: wu32, mut c: wu32, mut d: wu32) -> [wu32; 4] {
        a += b;
        d ^= a;
        d = d.rotate_left(16);

        c += d;
        b ^= c;
        b = b.rotate_left(12);

        a += b;
        d ^= a;
        d = d.rotate_left(8);

        c += d;
        b ^= c;
        b = b.rotate_left(7);

        [a, b, c, d]
    }

    fn block(input: [wu32; 16]) -> [wu32; 16] {
        let mut x = input;
        for _ in 0..ROUNDS / 2 {
            [x[0], x[4], x[8], x[12]] = Self::qr(x[0], x[4], x[8], x[12]);
            [x[1], x[5], x[9], x[13]] = Self::qr(x[1], x[5], x[9], x[13]);
            [x[2], x[6], x[10], x[14]] = Self::qr(x[2], x[6], x[10], x[14]);
            [x[3], x[7], x[11], x[15]] = Self::qr(x[3], x[7], x[11], x[15]);

            [x[0], x[5], x[10], x[15]] = Self::qr(x[0], x[5], x[10], x[15]);
            [x[1], x[6], x[11], x[12]] = Self::qr(x[1], x[6], x[11], x[12]);
            [x[2], x[7], x[8], x[13]] = Self::qr(x[2], x[7], x[8], x[13]);
            [x[3], x[4], x[9], x[14]] = Self::qr(x[3], x[4], x[9], x[14]);
        }

        std::array::from_fn(|i| x[i] + input[i])
    }

    fn next_raw(&mut self) -> [wu32; 16] {
        assert!(!self.exhausted, "block counter exhausted");
        let output = Self::block(self.x);
        if *self.x[12] == u32::MAX {
            self.exhausted = true;
        } else {
            self.x[12] += 1;
        }
        output
    }
}

impl<const ROUNDS: usize> Default for ChaCha<ROUNDS> {
    fn default() -> Self {
        Self::new(crate::_internal::default_seed32())
    }
}

impl<const ROUNDS: usize> crate::Seed for ChaCha<ROUNDS> {
    type Seed = u32;

    #[inline]
    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(seed)
    }
}

pub type ChaCha8 = ChaCha<8>;
impl_ring_rng32!(ChaCha8, 16, next_raw);

pub type ChaCha20 = ChaCha<20>;
impl_ring_rng32!(ChaCha20, 16, next_raw);

#[cfg(test)]
mod tests {
    use super::*;

    crate::safe_test! {
        ChaCha8,
        ChaCha20
    }
}

#[cfg(test)]
mod regression {
    use super::*;
    #[test]
    fn rfc8439_block_and_counter() {
        let key = std::array::from_fn(|i| i as u8);
        let nonce = [0, 0, 0, 9, 0, 0, 0, 0x4a, 0, 0, 0, 0];
        let expected = [
            0xe4e7f110, 0x15593bd1, 0x1fdd0f50, 0xc47120a3, 0xc7f4d1c7, 0x0368c033, 0x9aaa2204,
            0x4e6cd4c3, 0x466482d2, 0x09aa9f07, 0x05d7c214, 0xa2028bd9, 0xd19c12b5, 0xb94e16de,
            0xe883d0cb, 0x4e3c50a2,
        ];
        let mut r = ChaCha20::from_key_nonce(key, nonce, 1);
        assert_eq!(std::array::from_fn::<_, 16, _>(|_| r.nextu()), expected);
        let mut next = ChaCha20::from_key_nonce(key, nonce, 2);
        for _ in 0..32 {
            assert_eq!(r.nextu(), next.nextu());
        }
        let mut last = ChaCha20::from_key_nonce(key, nonce, u32::MAX);
        for _ in 0..16 {
            let _ = last.nextu();
        }
        assert!(std::panic::catch_unwind(move || last.nextu()).is_err());
    }
}
