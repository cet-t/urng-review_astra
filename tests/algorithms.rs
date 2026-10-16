use urng::*;
#[path = "support/reference.rs"]
mod reference;

#[test]
fn scalar_recurrences_match_independent_references() {
    for seed in [0, 1, 2, 42, u64::MAX] {
        let mut sg = seed;
        let mut sfc = [
            reference::sm64(&mut sg),
            reference::sm64(&mut sg),
            reference::sm64(&mut sg),
            1,
        ];
        let mut biski = [sfc[0], sfc[1], sfc[2]];
        let mut pp = [sfc[0], sfc[1]];
        let mut ss = pp;
        let (mut a, mut b, mut c, mut d) = (
            Sfc64::new(seed),
            Biski64::new(seed),
            Xoroshiro128Pp::new(seed),
            Xoroshiro128Ss::new(seed),
        );
        for _ in 0..1024 {
            assert_eq!(a.nextu(), reference::sfc(&mut sfc));
            assert_eq!(b.nextu(), reference::biski(&mut biski));
            assert_eq!(c.nextu(), reference::xoroshiro(&mut pp, true));
            assert_eq!(d.nextu(), reference::xoroshiro(&mut ss, false));
        }
    }
}

#[test]
fn jumps_match_reference_polynomials() {
    for seed in [0, 1, 42, u64::MAX] {
        let mut sg = seed;
        let initial = [reference::sm64(&mut sg), reference::sm64(&mut sg)];
        for long in [false, true] {
            let (mut pp, mut ss) = (Xoroshiro128Pp::new(seed), Xoroshiro128Ss::new(seed));
            let (mut p, mut s) = (initial, initial);
            let (p_poly, s_poly) = if long {
                pp.long_jump();
                ss.long_jump();
                (
                    [0x360fd5f2cf8d5d99, 0x9c6e6877736c46e3],
                    [0xd2a98b26625eee7b, 0xdddf9b1090aa7ac1],
                )
            } else {
                pp.jump();
                ss.jump();
                (
                    [0x2bd7a6a6e99c2ddc, 0x0992ccaf6a6fca05],
                    [0xdf900294d8f554a5, 0x170865df4b3201fc],
                )
            };
            reference::jump(&mut p, p_poly, true);
            reference::jump(&mut s, s_poly, false);
            for _ in 0..128 {
                assert_eq!(pp.nextu(), reference::xoroshiro(&mut p, true));
                assert_eq!(ss.nextu(), reference::xoroshiro(&mut s, false));
            }
        }
    }
}

#[test]
fn seeds_preserve_low_bit_and_xorshift_zero_is_repaired() {
    // SplitMix64's published zero-seed first word.
    assert_eq!(SplitMix64::new(0).nextu(), 0xe220a8397b1dcdaf);
    assert_ne!(SplitMix64::new(0).nextu(), SplitMix64::new(1).nextu());
    assert_ne!(SplitMix32::new(0).nextu(), SplitMix32::new(1).nextu());
    let mut rng = Xorshift64::new(0x61c8864680b583eb);
    let mut state = 1u64;
    for _ in 0..100 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        assert_eq!(rng.nextu(), state);
    }
}

#[test]
fn philox_seeded_blocks_match_reference() {
    assert_eq!(
        reference::philox([0; 4], [0; 2]),
        [0x6627e8d5, 0xe169c58d, 0xbc57ac4c, 0x9b00dbd8]
    );
    for seed in [0, 1, 2, 42, u32::MAX] {
        let mut sg = seed;
        let mut counter = std::array::from_fn(|_| reference::sm32(&mut sg));
        let key = [reference::sm32(&mut sg), reference::sm32(&mut sg)];
        let mut r = Philox32::new(seed);
        for _ in 0..100 {
            assert_eq!(
                std::array::from_fn::<_, 4, _>(|_| r.nextu()),
                reference::philox(counter, key)
            );
            for word in &mut counter {
                let (next, carry) = word.overflowing_add(1);
                *word = next;
                if !carry {
                    break;
                }
            }
        }
    }
}

#[test]
fn buffered_generators_start_with_generated_block() {
    for seed in [0, 1, 42, u64::MAX] {
        let mut p = Philox4x64::new(seed);
        let mut raw = Philox4x64::new(seed);
        let block = raw.next_raw().map(|v| *v);
        assert_eq!(std::array::from_fn::<_, 4, _>(|_| p.nextu()), block);
        let mut t = Threefish256::new(seed);
        let mut raw = t;
        let block = raw.next_raw().map(|v| *v);
        assert_eq!(std::array::from_fn::<_, 4, _>(|_| t.nextu()), block);
    }
}

#[test]
fn twisted_gfsr_has_full_width_and_reaches_each_choice() {
    let mut r = TwistedGFSR::new(123);
    let mut seen = [false; 10];
    assert!(r.nextu() > u32::MAX as u64);
    for _ in 0..1000 {
        seen[*r.choice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9])] = true;
    }
    assert!(seen.into_iter().all(|x| x));
}

#[cfg(feature = "wide")]
#[test]
fn wide_splitmix_matches_consecutive_scalar_words_at_all_widths() {
    use urng::wide::RngW;
    macro_rules! check {
        ($ty:ty) => {
            for seed in [0, 1, 42, u32::MAX] {
                let mut wide = <$ty>::new(seed);
                let mut scalar = SplitMix32::new(seed);
                let mut reference = seed;
                for _ in 0..100 {
                    for word in wide.nextu() {
                        let expected = reference::sm32(&mut reference);
                        assert_eq!(word, expected);
                        assert_eq!(scalar.nextu(), expected);
                    }
                }
            }
        };
    }
    check!(urng::wide::SplitMix32x4);
    check!(urng::wide::SplitMix32x8);
    check!(urng::wide::SplitMix32x16);
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[test]
fn simd_splitmix_matches_consecutive_scalar_words() {
    if !std::is_x86_feature_detected!("avx512f") {
        return;
    }
    for seed in [0, 1, 42, u32::MAX] {
        let mut vector = unsafe { SplitMix32x16::new(seed) };
        let mut scalar = SplitMix32::new(seed);
        let mut state = seed;
        for _ in 0..100 {
            let words: [u32; 16] = unsafe { std::mem::transmute(vector.nextuv()) };
            for word in words {
                let expected = reference::sm32(&mut state);
                assert_eq!(word, expected);
                assert_eq!(scalar.nextu(), expected);
            }
        }
    }
}
