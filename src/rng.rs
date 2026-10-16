//! A 32/64-bit random number generator trait.

use crate::_internal::{i2f_bits, u2f_01};

mod sealed {
    /// Prevents `Word` from being implemented outside this crate, so the
    /// `u32 -> f32/i32` / `u64 -> f64/i64` width relationships stay closed.
    pub trait Sealed {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
}

/// The unsigned output word of a generator (`u32` or `u64`).
///
/// A `Word` fixes the whole numeric family of an RNG — its float type, its
/// signed-range integer type, and the bit width — and centralizes the
/// uniform-conversion formulas in one place so every [`Rng`] implementor
/// inherits them for free.
pub trait Word: sealed::Sealed + Copy {
    /// The float type produced from this word (`f32` for `u32`, `f64` for `u64`).
    type Float: Copy;
    /// The signed integer type used for ranged draws (`i32` / `i64`).
    type Int: Copy;

    /// Maps the word uniformly onto `[0, 1)`.
    #[must_use]
    fn to_f01(self) -> Self::Float;

    /// Maps one word onto `[min, max]`. This single-word mapping can be biased.
    /// Use [`Rng::randi`] for rejection sampling without range-reduction bias.
    #[must_use]
    fn to_randi(self, min: Self::Int, max: Self::Int) -> Self::Int;

    /// Maps the word onto `[min, max)`, with finite endpoints and min < max.
    /// Panics for invalid bounds; floating-point rounding is clamped at the upper bound.
    #[must_use]
    fn to_randf(self, min: Self::Float, max: Self::Float) -> Self::Float;

    /// Maps one word onto `[0, len)` using multiply-shift; this can be biased.
    /// Use [`Rng::index`] for an unbiased draw.
    #[must_use]
    fn to_index(self, len: usize) -> usize;

    /// Returns an unbiased candidate, or None when another word is required.
    fn try_randi(self, min: Self::Int, max: Self::Int) -> Option<Self::Int>;

    /// Draws an unbiased index, using two words when a u32 word is too narrow.
    fn sample_index<F: FnMut() -> Self>(draw: F, len: usize) -> usize;
}

macro_rules! impl_word {
    ($bits:tt) => {
        pastey::paste! {
            impl Word for [<u $bits>] {
                type Float = [<f $bits>];
                type Int = [<i $bits>];

                #[inline(always)]
                fn to_f01(self) -> Self::Float {
                    u2f_01!([<f $bits>], $bits, self)
                }

                #[inline(always)]
                fn to_randi(self, min: Self::Int, max: Self::Int) -> Self::Int {
                    assert!(min <= max, "invalid integer range");
                    let range = (max as i128 - min as i128 + 1) as u128;
                    (((self as u128 * range) >> $bits) as i128 + min as i128) as Self::Int
                }

                #[inline(always)]
                fn to_randf(self, min: Self::Float, max: Self::Float) -> Self::Float {
                    assert!(min.is_finite() && max.is_finite() && min < max, "invalid float range");
                    let t = self.to_f01();
                    let width = max - min;
                    let value = if width.is_finite() { t * width + min }
                        else { (1.0 - t) * min + t * max };
                    // Rounding can otherwise reach the excluded upper bound.
                    value.max(min).min(max.next_down())
                }

                #[inline(always)]
                fn to_index(self, len: usize) -> usize {
                    assert!(len > 0, "empty index range");
                    ((self as u128 * len as u128) >> $bits) as usize
                }

                fn try_randi(self, min: Self::Int, max: Self::Int) -> Option<Self::Int> {
                    assert!(min <= max, "invalid integer range");
                    let range = (max as i128 - min as i128 + 1) as u128;
                    let product = self as u128 * range;
                    let mask = (1u128 << $bits) - 1;
                    let threshold = (1u128 << $bits) % range;
                    if product & mask < threshold { None }
                    else { Some(((product >> $bits) as i128 + min as i128) as Self::Int) }
                }

                fn sample_index<F: FnMut() -> Self>(mut draw: F, len: usize) -> usize {
                    assert!(len > 0, "empty index range");
                    let bits = if $bits == 32 && len as u128 > (1u128 << 32) { 64 } else { $bits };
                    let modulus = 1u128 << bits;
                    let threshold = modulus % len as u128;
                    loop {
                        let mut word = draw() as u128;
                        if bits > $bits { word = (word << 32) | draw() as u128; }
                        let product = word * len as u128;
                        if product & (modulus - 1) >= threshold { return (product >> bits) as usize; }
                    }
                }
            }
        }
    };
}

impl_word!(32);
impl_word!(64);

/// A random number generator characterized by a single output [`Word`].
///
/// Implementors provide only [`nextu`](Rng::nextu); the ranged and float
/// draws are supplied as defaults derived from the associated [`Word`], so the
/// numeric formulas live in exactly one place.
pub trait Rng {
    /// The unsigned output word this generator produces.
    type Word: Word;

    /// Generates the next raw word in `[0, 2^BITS)`.
    #[must_use]
    fn nextu(&mut self) -> Self::Word;

    /// Generates the next float in `[0, 1)`.
    #[must_use]
    #[inline(always)]
    fn nextf(&mut self) -> <Self::Word as Word>::Float {
        self.nextu().to_f01()
    }

    /// Generates an integer uniformly in the inclusive range `[min, max]`.
    /// Panics for min > max. Rejection sampling may consume additional words.
    #[must_use]
    #[inline(always)]
    fn randi(
        &mut self,
        min: <Self::Word as Word>::Int,
        max: <Self::Word as Word>::Int,
    ) -> <Self::Word as Word>::Int {
        loop {
            if let Some(value) = self.nextu().try_randi(min, max) {
                return value;
            }
        }
    }

    /// Generates an unbiased index in `[0, len)`. Panics for an empty range.
    fn index(&mut self, len: usize) -> usize {
        Self::Word::sample_index(|| self.nextu(), len)
    }

    /// Generates a float in the half-open range `[min, max)`.
    /// Panics unless both endpoints are finite and min < max.
    #[must_use]
    #[inline(always)]
    fn randf(
        &mut self,
        min: <Self::Word as Word>::Float,
        max: <Self::Word as Word>::Float,
    ) -> <Self::Word as Word>::Float {
        self.nextu().to_randf(min, max)
    }
}

/// Constructs a generator from a raw seed of its native word width.
pub trait Seed {
    /// The seed type (`u32` or `u64`, matching the generator's word width).
    type Seed;

    /// Builds `Self` from a seed.
    #[must_use]
    fn from_seed(seed: Self::Seed) -> Self;
}

// Refill only rejected lanes. Accepted lanes keep their original output.
pub(crate) fn array_randi<W: Word, const N: usize>(
    mut draw: impl FnMut() -> [W; N],
    min: W::Int,
    max: W::Int,
) -> [W::Int; N] {
    let mut values = [None; N];
    while values.iter().any(Option::is_none) {
        for (value, word) in values.iter_mut().zip(draw()) {
            if value.is_none() {
                *value = word.try_randi(min, max);
            }
        }
    }
    values.map(Option::unwrap)
}
