use std::arch::x86_64::*;

mod sealed {
    use std::arch::x86_64::{__m128i, __m256i, __m512i};

    pub trait Sealed {}
    impl Sealed for __m128i {}
    impl Sealed for __m256i {}
    impl Sealed for __m512i {}
}

pub trait WordV: sealed::Sealed + Copy + Sized {
    type Int;
    type Float;

    #[must_use]
    fn to_f01(self) -> Self::Float;

    /// Single-word multiply-high mapping; may be biased. Zero scale denotes 2^32.
    #[must_use]
    fn to_randi(self, scale: Self::Int, min: Self::Int) -> Self::Int;

    /// Low-level affine conversion: `to_f01() * scale + min`, without range validation.
    #[must_use]
    fn to_randf(self, scale: Self::Float, min: Self::Float) -> Self::Float;

    /// Rejection-sampled integer lanes. Zero scale denotes the full i32 range.
    fn sample_randi(draw: impl FnMut() -> Self, scale: Self::Int, min: Self::Int) -> Self::Int;
}

// These public safe conversions also accept vectors constructed without an
// AVX-capable RNG. Array conversion keeps them safe on every x86_64 CPU.
macro_rules! impl_wordv {
    ($int:ty, $float:ty, $n:expr) => {
        impl WordV for $int {
            type Int = Self;
            type Float = $float;
            fn to_f01(self) -> Self::Float {
                let words: [u32; $n] = unsafe { std::mem::transmute(self) };
                unsafe { std::mem::transmute(words.map(crate::Word::to_f01)) }
            }
            fn to_randi(self, scale: Self::Int, min: Self::Int) -> Self::Int {
                let words: [u32; $n] = unsafe { std::mem::transmute(self) };
                let scales: [u32; $n] = unsafe { std::mem::transmute(scale) };
                let mins: [i32; $n] = unsafe { std::mem::transmute(min) };
                let result: [i32; $n] = std::array::from_fn(|i| {
                    let range = if scales[i] == 0 {
                        1u64 << 32
                    } else {
                        scales[i] as u64
                    };
                    let offset = ((words[i] as u64 * range) >> 32) as u32;
                    (mins[i] as u32).wrapping_add(offset) as i32
                });
                unsafe { std::mem::transmute(result) }
            }
            fn sample_randi(
                mut draw: impl FnMut() -> Self,
                scale: Self::Int,
                min: Self::Int,
            ) -> Self::Int {
                use crate::Word;
                let scales: [u32; $n] = unsafe { std::mem::transmute(scale) };
                let mins: [i32; $n] = unsafe { std::mem::transmute(min) };
                let maxs: [i32; $n] = std::array::from_fn(|i| {
                    let range = if scales[i] == 0 {
                        1i64 << 32
                    } else {
                        scales[i] as i64
                    };
                    let max = mins[i] as i64 + range - 1;
                    i32::try_from(max).expect("integer lane range exceeds i32")
                });
                let mut values = [None; $n];
                while values.iter().any(Option::is_none) {
                    let words: [u32; $n] = unsafe { std::mem::transmute(draw()) };
                    for i in 0..$n {
                        if values[i].is_none() {
                            values[i] = words[i].try_randi(mins[i], maxs[i]);
                        }
                    }
                }
                let result = values.map(Option::unwrap);
                unsafe { std::mem::transmute(result) }
            }
            fn to_randf(self, scale: Self::Float, min: Self::Float) -> Self::Float {
                let values: [f32; $n] = unsafe { std::mem::transmute(self.to_f01()) };
                let scales: [f32; $n] = unsafe { std::mem::transmute(scale) };
                let mins: [f32; $n] = unsafe { std::mem::transmute(min) };
                let result: [f32; $n] = std::array::from_fn(|i| values[i] * scales[i] + mins[i]);
                unsafe { std::mem::transmute(result) }
            }
        }
    };
}
impl_wordv!(__m128i, __m128, 4);
impl_wordv!(__m256i, __m256, 8);
impl_wordv!(__m512i, __m512, 16);

pub trait RngV {
    type Word: WordV;

    fn nextuv(&mut self) -> Self::Word;

    fn nextfv(&mut self) -> <Self::Word as WordV>::Float {
        self.nextuv().to_f01()
    }

    fn randiv(
        &mut self,
        scale: <Self::Word as WordV>::Int,
        min: <Self::Word as WordV>::Int,
    ) -> <Self::Word as WordV>::Int {
        Self::Word::sample_randi(|| self.nextuv(), scale, min)
    }

    fn randfv(
        &mut self,
        scale: <Self::Word as WordV>::Float,
        min: <Self::Word as WordV>::Float,
    ) -> <Self::Word as WordV>::Float {
        self.nextuv().to_randf(scale, min)
    }
}
