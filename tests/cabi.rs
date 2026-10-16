#![cfg(feature = "cabi")]
use urng::*;

fn check<R, W: Copy + Default + PartialEq + std::fmt::Debug, S: Copy>(
    seed: S,
    new: extern "C" fn(S) -> *mut R,
    free: unsafe extern "C" fn(*mut R),
    fill: unsafe extern "C" fn(*mut R, *mut W, usize),
    expected: &[W],
    sentinel: W,
) {
    check_with(
        seed,
        new,
        free,
        |p, o, n| unsafe { fill(p, o, n) },
        expected,
        sentinel,
    );
}

fn check_with<R, W: Copy + Default + PartialEq + std::fmt::Debug, S: Copy>(
    seed: S,
    new: extern "C" fn(S) -> *mut R,
    free: unsafe extern "C" fn(*mut R),
    fill: impl Fn(*mut R, *mut W, usize),
    expected: &[W],
    sentinel: W,
) {
    unsafe {
        fill(std::ptr::null_mut(), std::ptr::null_mut(), 0);
        free(std::ptr::null_mut());
        // All natural alignments within a cache line; guards detect overwrites.
        for offset in 0..64 / std::mem::size_of::<W>() {
            for count in [0, 1, 2, 3, 15, 16, 17, 63, 64, 65, 257] {
                let p = new(seed);
                assert!(!p.is_null());
                let mut buf = vec![sentinel; count + 64];
                let start = 16 + offset;
                fill(p, std::ptr::null_mut(), 0);
                fill(p, buf.as_mut_ptr().add(start), count);
                assert_eq!(&buf[start..start + count], &expected[..count]);
                assert!(buf[..start].iter().all(|&x| x == sentinel));
                assert!(buf[start + count..].iter().all(|&x| x == sentinel));
                let mut tail = [W::default(); 3];
                fill(p, tail.as_mut_ptr(), 3);
                assert_eq!(&tail, &expected[count..count + 3]);
                free(p);
            }
        }
        // Irregular calls must consume exactly the same stream, including tails.
        let p = new(seed);
        let mut split = vec![W::default(); expected.len()];
        let mut pos = 0;
        for count in [1, 3, 7, 16, 63, 64, 65] {
            fill(p, split.as_mut_ptr().add(pos), count);
            pos += count;
        }
        fill(p, split.as_mut_ptr().add(pos), split.len() - pos);
        assert_eq!(split, expected);
        free(p);
    }
}

include!("support/cabi_cases.rs");

#[test]
fn rejected_pointers_counts_and_ranges_do_not_touch_memory_or_state() {
    unsafe {
        let p = pcg32_new(1);
        let mut output = [0x12345678u32; 8];
        pcg32_next_u32s(std::ptr::null_mut(), output.as_mut_ptr(), 1);
        pcg32_next_u32s(p, std::ptr::null_mut(), 1);
        pcg32_next_u32s(p, output.as_mut_ptr().cast::<u8>().add(1).cast(), 1);
        pcg32_next_u32s(p.cast::<u8>().add(1).cast(), output.as_mut_ptr(), 1);
        pcg32_next_u32s(p, output.as_mut_ptr(), usize::MAX);
        pcg32_rand_i32s(p, output.as_mut_ptr().cast(), 1, 10, 9);
        pcg32_rand_f32s(p, output.as_mut_ptr().cast(), 1, 0.0, f32::INFINITY);
        assert_eq!(output, [0x12345678; 8]);
        pcg32_next_u32s(p, output.as_mut_ptr(), 8);
        let mut reference = Pcg32::new(1);
        assert_eq!(output, std::array::from_fn(|_| reference.nextu()));
        pcg32_free(p);
    }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[test]
fn mixed_numeric_calls_share_pending_simd_words() {
    if !std::is_x86_feature_detected!("avx512f") {
        return;
    }
    struct Flattened(std::vec::IntoIter<u64>);
    impl Rng for Flattened {
        type Word = u64;
        fn nextu(&mut self) -> u64 {
            self.0.next().unwrap()
        }
    }
    let mut vector = Biski64x8::new(1);
    let mut words = Vec::new();
    for _ in 0..32 {
        words.extend(unsafe { vector.nextu() });
    }
    let mut reference = Flattened(words.into_iter());
    unsafe {
        let p = biski64x8_new(1);
        let mut u = [0; 1];
        biski64x8_next_u64s(p, u.as_mut_ptr(), 1);
        assert_eq!(u, [reference.nextu()]);
        let mut f = [0.0; 3];
        biski64x8_next_f64s(p, f.as_mut_ptr(), 3);
        assert_eq!(f, std::array::from_fn(|_| reference.nextf()));
        let mut i = [0; 5];
        biski64x8_rand_i64s(p, i.as_mut_ptr(), 5, -5, 7);
        assert_eq!(i, std::array::from_fn(|_| reference.randi(-5, 7)));
        let mut f = [0.0; 7];
        biski64x8_rand_f64s(p, f.as_mut_ptr(), 7, -1.0, 1.0);
        assert_eq!(f, std::array::from_fn(|_| reference.randf(-1.0, 1.0)));
        let mut u = [0; 33];
        biski64x8_next_u64s(p, u.as_mut_ptr(), 33);
        assert_eq!(u, std::array::from_fn(|_| reference.nextu()));
        biski64x8_free(p);
    }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[test]
fn former_parallel_chunks_preserve_the_reference_stream() {
    if !std::is_x86_feature_detected!("avx512f") {
        return;
    }
    macro_rules! check_long {
        ($ty:ty,$new:ident,$fill:ident,$free:ident,$n:expr) => {{
            let mut r = <$ty>::new(1);
            let mut expected = Vec::new();
            while expected.len() < $n {
                expected.extend(unsafe { r.nextu() });
            }
            expected.truncate($n);
            let mut got = vec![0; $n + 1];
            unsafe {
                let p = $new(1);
                $fill(p, got[1..].as_mut_ptr(), $n);
                $free(p);
            }
            assert_eq!(&got[1..], expected);
        }};
    }
    check_long!(Sfc64x8, sfc64x8_new, sfc64x8_next_u64s, sfc64x8_free, 8193);
    check_long!(
        Biski64x8,
        biski64x8_new,
        biski64x8_next_u64s,
        biski64x8_free,
        32769
    );
    if !std::is_x86_feature_detected!("avx512dq") {
        return;
    }
    check_long!(
        Cet64x8,
        cet64x8_new,
        cet64x8_next_u64s,
        cet64x8_free,
        131073
    );
}
