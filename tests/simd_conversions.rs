#![cfg(all(feature = "simd", target_arch = "x86_64"))]
use std::arch::x86_64::*;
use urng::WordV;

#[test]
fn conversions_match_each_scalar_lane_without_cpu_preconditions() {
    macro_rules! check {
        ($vector:ty,$n:expr) => {{
            let words: [u32; $n] =
                std::array::from_fn(|i| u32::MAX.wrapping_sub((i as u32) * 12345));
            let scales: [u32; $n] = std::array::from_fn(|i| 10 + i as u32);
            let mins: [i32; $n] = std::array::from_fn(|i| -(i as i32));
            let v: $vector = unsafe { std::mem::transmute(words) };
            let s: $vector = unsafe { std::mem::transmute(scales) };
            let m: $vector = unsafe { std::mem::transmute(mins) };
            let got: [i32; $n] = unsafe { std::mem::transmute(v.to_randi(s, m)) };
            for i in 0..$n {
                let expected = ((words[i] as u64 * scales[i] as u64) >> 32) as i64 + mins[i] as i64;
                assert_eq!(got[i] as i64, expected);
            }
            let got: [f32; $n] = unsafe { std::mem::transmute(v.to_f01()) };
            for i in 0..$n {
                assert_eq!(got[i], (words[i] >> 9) as f32 / (1u32 << 23) as f32);
            }
        }};
    }
    check!(__m128i, 4);
    check!(__m256i, 8);
    check!(__m512i, 16);
}

#[test]
fn vector_range_sampling_rejects_each_lane_independently() {
    macro_rules! check {
        ($vector:ty,$n:expr) => {{
            let scale: $vector = unsafe { std::mem::transmute([3u32; $n]) };
            let min: $vector = unsafe { std::mem::transmute([-1i32; $n]) };
            let mut draws = 0;
            let output = <$vector>::sample_randi(
                || {
                    draws += 1;
                    unsafe {
                        std::mem::transmute(std::array::from_fn::<u32, $n, _>(|i| {
                            if draws == 1 && i % 2 == 0 {
                                0
                            } else {
                                u32::MAX
                            }
                        }))
                    }
                },
                scale,
                min,
            );
            let output: [i32; $n] = unsafe { std::mem::transmute(output) };
            assert_eq!(output, [1; $n]);
            assert_eq!(draws, 2);
            let scale: $vector = unsafe { std::mem::transmute([0u32; $n]) };
            let min: $vector = unsafe { std::mem::transmute([i32::MIN; $n]) };
            let words: $vector = unsafe { std::mem::transmute([u32::MAX; $n]) };
            let output: [i32; $n] =
                unsafe { std::mem::transmute(<$vector>::sample_randi(|| words, scale, min)) };
            assert_eq!(output, [i32::MAX; $n]);
        }};
    }
    check!(__m128i, 4);
    check!(__m256i, 8);
    check!(__m512i, 16);
}
