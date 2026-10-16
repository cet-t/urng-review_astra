use urng::{Choice, Rng, Shuffle, Word};

struct Script<W> {
    words: std::vec::IntoIter<W>,
    draws: usize,
}
impl<W: Word> Rng for Script<W> {
    type Word = W;
    fn nextu(&mut self) -> W {
        self.draws += 1;
        self.words.next().expect("unexpected extra draw")
    }
}
fn script<W>(words: impl Into<Vec<W>>) -> Script<W> {
    Script {
        words: words.into().into_iter(),
        draws: 0,
    }
}

#[test]
fn signed_ranges_include_full_width_without_overflow() {
    macro_rules! check {
        ($u:ty,$i:ty) => {{
            for (min, max) in [
                (<$i>::MIN, <$i>::MAX),
                (<$i>::MIN, 0),
                (-1, <$i>::MAX),
                (-7, 7),
                (13, 13),
            ] {
                for word in [0, 1, <$u>::MAX / 2, <$u>::MAX] {
                    let expected = (((word as u128 * (max as i128 - min as i128 + 1) as u128)
                        >> <$u>::BITS) as i128
                        + min as i128) as $i;
                    assert_eq!(word.to_randi(min, max), expected);
                }
            }
            let mut r = script(vec![0 as $u, <$u>::MAX]);
            assert_eq!(r.randi(<$i>::MIN, <$i>::MAX), <$i>::MIN);
            assert_eq!(r.randi(<$i>::MIN, <$i>::MAX), <$i>::MAX);
            assert!(std::panic::catch_unwind(|| (1 as $u).to_randi(1, 0)).is_err());
        }};
    }
    check!(u32, i32);
    check!(u64, i64);
}

#[test]
fn integer_and_index_sampling_reject_biased_low_products() {
    let mut r = script(vec![0u32, u32::MAX]);
    assert_eq!(r.randi(-1, 1), 1);
    assert_eq!(r.draws, 2);
    let mut r = script(vec![0u64, u64::MAX]);
    assert_eq!(r.randi(-1, 1), 1);
    assert_eq!(r.draws, 2);
    let mut r = script(vec![0u32, u32::MAX]);
    assert_eq!(*r.choice(&[10, 20, 30]), 30);
    assert_eq!(r.draws, 2);
    #[cfg(target_pointer_width = "64")]
    {
        let mut r = script(vec![u32::MAX, u32::MAX]);
        assert_eq!(r.index(usize::MAX), usize::MAX - 1);
        assert_eq!(r.draws, 2);
    }
}

#[test]
fn float_ranges_exclude_upper_endpoint_and_remain_finite() {
    macro_rules! check {
        ($u:ty,$f:ty) => {{
            for (min, max) in [
                (1.0 as $f, (1.0 as $f).next_up()),
                (-<$f>::MAX, <$f>::MAX),
                (-<$f>::MIN_POSITIVE, <$f>::MIN_POSITIVE),
                (0.0, <$f>::from_bits(1)),
                (-7.0, 13.0),
            ] {
                for word in [0 as $u, 1, <$u>::MAX / 2, <$u>::MAX] {
                    let value = word.to_randf(min, max);
                    assert!(
                        value.is_finite() && value >= min && value < max,
                        "{word}: {min} <= {value} < {max}"
                    );
                }
            }
            assert!(std::panic::catch_unwind(|| (0 as $u).to_randf(0.0, <$f>::INFINITY)).is_err());
        }};
    }
    check!(u32, f32);
    check!(u64, f64);
}

#[test]
fn fisher_yates_has_exactly_six_three_element_permutations() {
    let mut permutations = std::collections::BTreeSet::new();
    for first in 0..3u64 {
        for second in 0..2u64 {
            let words = vec![
                ((first * (1u64 << 32)).div_ceil(3) + 1) as u32,
                (second << 31) as u32,
            ];
            let mut r = script(words.clone());
            let mut values = [0, 1, 2];
            r.shuffle(&mut values).unwrap();
            assert_eq!(r.draws, 2);
            let mut r = script(words);
            let iter: Vec<_> = r.shuffled(&[0, 1, 2]).unwrap().copied().collect();
            assert_eq!(iter, values);
            permutations.insert(values);
        }
    }
    assert_eq!(permutations.len(), 6);
    let mut r = script(Vec::<u32>::new());
    assert!(r.shuffle::<u8>(&mut []).is_err());
    r.shuffle(&mut [1]).unwrap();
    assert_eq!(r.draws, 0);
}

#[cfg(feature = "sampler")]
#[test]
fn cumulative_sampler_skips_zero_weights_and_exact_boundaries() {
    use urng::{Bst32, Bst64, Sampler};
    let mut r = script(vec![0u32, 1u32 << 31, u32::MAX]);
    let mut s = Bst32::new(&mut r, &[0.0, 1.0, 0.0, 1.0, 0.0]);
    assert_eq!([s.sample(), s.sample(), s.sample()], [1, 3, 3]);
    let mut r = script(vec![0u64, 1u64 << 62, u64::MAX]);
    let mut s = Bst64::new(&mut r, &[0.0, 1.0, 0.0, 1.0, 0.0]);
    assert_eq!([s.sample(), s.sample(), s.sample()], [1, 3, 3]);
    for weights in [
        vec![],
        vec![0.0],
        vec![-1.0, 2.0],
        vec![f64::NAN],
        vec![f64::INFINITY],
        vec![f64::MAX, f64::MAX],
    ] {
        assert!(
            std::panic::catch_unwind(|| {
                let mut r = script(Vec::<u64>::new());
                Bst64::new(&mut r, &weights);
            })
            .is_err()
        );
    }
}

#[cfg(feature = "wide")]
#[test]
fn wide_rejection_keeps_already_accepted_lanes() {
    use urng::wide::RngW;
    struct Blocks {
        words: std::vec::IntoIter<[u32; 4]>,
    }
    impl RngW<4> for Blocks {
        type Word = u32;
        fn nextu(&mut self) -> [u32; 4] {
            self.words.next().unwrap()
        }
    }
    let mut r = Blocks {
        words: vec![[0, u32::MAX, 1 << 31, 1], [u32::MAX, 0, 0, 0]].into_iter(),
    };
    assert_eq!(r.randi(0, 2), [2, 2, 1, 0]);
    let mut r = Blocks {
        words: vec![[0, 1, 1 << 31, u32::MAX]].into_iter(),
    };
    assert_eq!(
        r.randi(i32::MIN, i32::MAX),
        [i32::MIN, i32::MIN + 1, 0, i32::MAX]
    );
}
