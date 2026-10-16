// Exported-family coverage generated from the preserved ABI inventory.
#[test]
fn jsf32_boundaries_alignment_and_partition() {
    let mut r = Jsf32::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        jsf32_new,
        jsf32_free,
        jsf32_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        jsf32_new,
        jsf32_free,
        |p, o, n| unsafe { jsf32_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        jsf32_new,
        jsf32_free,
        |p, o, n| unsafe { jsf32_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        jsf32_new,
        jsf32_free,
        |p, o, n| unsafe { jsf32_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[test]
fn jsf32x16_boundaries_alignment_and_partition() {
    if !std::is_x86_feature_detected!("avx512f") {
        assert!(jsf32x16_new(1).is_null());
        return;
    }
    let mut r = unsafe { Jsf32x16::new(1) };
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.extend(unsafe { std::mem::transmute::<_, [u32; 16]>(r.nextuv()) });
    }
    check(
        1,
        jsf32x16_new,
        jsf32x16_free,
        jsf32x16_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        jsf32x16_new,
        jsf32x16_free,
        |p, o, n| unsafe { jsf32x16_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        jsf32x16_new,
        jsf32x16_free,
        |p, o, n| unsafe { jsf32x16_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        jsf32x16_new,
        jsf32x16_free,
        |p, o, n| unsafe { jsf32x16_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn mt19937_boundaries_alignment_and_partition() {
    let mut r = Mt19937::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        mt19937_new,
        mt19937_free,
        mt19937_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        mt19937_new,
        mt19937_free,
        |p, o, n| unsafe { mt19937_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        mt19937_new,
        mt19937_free,
        |p, o, n| unsafe { mt19937_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        mt19937_new,
        mt19937_free,
        |p, o, n| unsafe { mt19937_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn sfmt19937_boundaries_alignment_and_partition() {
    let mut r = Sfmt19937::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        sfmt19937_new,
        sfmt19937_free,
        sfmt19937_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        sfmt19937_new,
        sfmt19937_free,
        |p, o, n| unsafe { sfmt19937_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        sfmt19937_new,
        sfmt19937_free,
        |p, o, n| unsafe { sfmt19937_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        sfmt19937_new,
        sfmt19937_free,
        |p, o, n| unsafe { sfmt19937_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn sfmt607_boundaries_alignment_and_partition() {
    let mut r = Sfmt607::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        sfmt607_new,
        sfmt607_free,
        sfmt607_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        sfmt607_new,
        sfmt607_free,
        |p, o, n| unsafe { sfmt607_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        sfmt607_new,
        sfmt607_free,
        |p, o, n| unsafe { sfmt607_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        sfmt607_new,
        sfmt607_free,
        |p, o, n| unsafe { sfmt607_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn sfmt1279_boundaries_alignment_and_partition() {
    let mut r = Sfmt1279::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        sfmt1279_new,
        sfmt1279_free,
        sfmt1279_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        sfmt1279_new,
        sfmt1279_free,
        |p, o, n| unsafe { sfmt1279_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        sfmt1279_new,
        sfmt1279_free,
        |p, o, n| unsafe { sfmt1279_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        sfmt1279_new,
        sfmt1279_free,
        |p, o, n| unsafe { sfmt1279_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn sfmt2281_boundaries_alignment_and_partition() {
    let mut r = Sfmt2281::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        sfmt2281_new,
        sfmt2281_free,
        sfmt2281_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        sfmt2281_new,
        sfmt2281_free,
        |p, o, n| unsafe { sfmt2281_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        sfmt2281_new,
        sfmt2281_free,
        |p, o, n| unsafe { sfmt2281_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        sfmt2281_new,
        sfmt2281_free,
        |p, o, n| unsafe { sfmt2281_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn sfmt4253_boundaries_alignment_and_partition() {
    let mut r = Sfmt4253::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        sfmt4253_new,
        sfmt4253_free,
        sfmt4253_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        sfmt4253_new,
        sfmt4253_free,
        |p, o, n| unsafe { sfmt4253_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        sfmt4253_new,
        sfmt4253_free,
        |p, o, n| unsafe { sfmt4253_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        sfmt4253_new,
        sfmt4253_free,
        |p, o, n| unsafe { sfmt4253_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn sfmt11213_boundaries_alignment_and_partition() {
    let mut r = Sfmt11213::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        sfmt11213_new,
        sfmt11213_free,
        sfmt11213_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        sfmt11213_new,
        sfmt11213_free,
        |p, o, n| unsafe { sfmt11213_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        sfmt11213_new,
        sfmt11213_free,
        |p, o, n| unsafe { sfmt11213_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        sfmt11213_new,
        sfmt11213_free,
        |p, o, n| unsafe { sfmt11213_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn sfmt44497_boundaries_alignment_and_partition() {
    let mut r = Sfmt44497::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        sfmt44497_new,
        sfmt44497_free,
        sfmt44497_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        sfmt44497_new,
        sfmt44497_free,
        |p, o, n| unsafe { sfmt44497_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        sfmt44497_new,
        sfmt44497_free,
        |p, o, n| unsafe { sfmt44497_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        sfmt44497_new,
        sfmt44497_free,
        |p, o, n| unsafe { sfmt44497_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn sfmt86243_boundaries_alignment_and_partition() {
    let mut r = Sfmt86243::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        sfmt86243_new,
        sfmt86243_free,
        sfmt86243_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        sfmt86243_new,
        sfmt86243_free,
        |p, o, n| unsafe { sfmt86243_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        sfmt86243_new,
        sfmt86243_free,
        |p, o, n| unsafe { sfmt86243_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        sfmt86243_new,
        sfmt86243_free,
        |p, o, n| unsafe { sfmt86243_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn sfmt132049_boundaries_alignment_and_partition() {
    let mut r = Sfmt132049::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        sfmt132049_new,
        sfmt132049_free,
        sfmt132049_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        sfmt132049_new,
        sfmt132049_free,
        |p, o, n| unsafe { sfmt132049_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        sfmt132049_new,
        sfmt132049_free,
        |p, o, n| unsafe { sfmt132049_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        sfmt132049_new,
        sfmt132049_free,
        |p, o, n| unsafe { sfmt132049_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn sfmt216091_boundaries_alignment_and_partition() {
    let mut r = Sfmt216091::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        sfmt216091_new,
        sfmt216091_free,
        sfmt216091_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        sfmt216091_new,
        sfmt216091_free,
        |p, o, n| unsafe { sfmt216091_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        sfmt216091_new,
        sfmt216091_free,
        |p, o, n| unsafe { sfmt216091_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        sfmt216091_new,
        sfmt216091_free,
        |p, o, n| unsafe { sfmt216091_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn pcg32_boundaries_alignment_and_partition() {
    let mut r = Pcg32::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        pcg32_new,
        pcg32_free,
        pcg32_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        pcg32_new,
        pcg32_free,
        |p, o, n| unsafe { pcg32_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        pcg32_new,
        pcg32_free,
        |p, o, n| unsafe { pcg32_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        pcg32_new,
        pcg32_free,
        |p, o, n| unsafe { pcg32_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[test]
fn pcg32x8_boundaries_alignment_and_partition() {
    if !std::is_x86_feature_detected!("avx512f") {
        assert!(pcg32x8_new(1).is_null());
        return;
    }
    let mut r = unsafe { Pcg32x8::new(1) };
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.extend(unsafe { r.nextu() });
    }
    check(
        1,
        pcg32x8_new,
        pcg32x8_free,
        pcg32x8_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        pcg32x8_new,
        pcg32x8_free,
        |p, o, n| unsafe { pcg32x8_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        pcg32x8_new,
        pcg32x8_free,
        |p, o, n| unsafe { pcg32x8_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        pcg32x8_new,
        pcg32x8_free,
        |p, o, n| unsafe { pcg32x8_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn sfc32_boundaries_alignment_and_partition() {
    let mut r = Sfc32::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        sfc32_new,
        sfc32_free,
        sfc32_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        sfc32_new,
        sfc32_free,
        |p, o, n| unsafe { sfc32_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        sfc32_new,
        sfc32_free,
        |p, o, n| unsafe { sfc32_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        sfc32_new,
        sfc32_free,
        |p, o, n| unsafe { sfc32_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[test]
fn sfc32x4_boundaries_alignment_and_partition() {
    let mut r = Sfc32x4::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.extend(r.nextu());
    }
    check(
        1,
        sfc32x4_new,
        sfc32x4_free,
        sfc32x4_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        sfc32x4_new,
        sfc32x4_free,
        |p, o, n| unsafe { sfc32x4_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        sfc32x4_new,
        sfc32x4_free,
        |p, o, n| unsafe { sfc32x4_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        sfc32x4_new,
        sfc32x4_free,
        |p, o, n| unsafe { sfc32x4_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[test]
fn sfc32x8_boundaries_alignment_and_partition() {
    if !std::is_x86_feature_detected!("avx2") {
        assert!(sfc32x8_new(1).is_null());
        return;
    }
    let mut r = unsafe { Sfc32x8::new(1) };
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.extend(r.nextu());
    }
    check(
        1,
        sfc32x8_new,
        sfc32x8_free,
        sfc32x8_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        sfc32x8_new,
        sfc32x8_free,
        |p, o, n| unsafe { sfc32x8_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        sfc32x8_new,
        sfc32x8_free,
        |p, o, n| unsafe { sfc32x8_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        sfc32x8_new,
        sfc32x8_free,
        |p, o, n| unsafe { sfc32x8_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[test]
fn sfc32x16_boundaries_alignment_and_partition() {
    if !std::is_x86_feature_detected!("avx512f") {
        assert!(sfc32x16_new(1).is_null());
        return;
    }
    let mut r = unsafe { Sfc32x16::new(1) };
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.extend(r.nextu());
    }
    check(
        1,
        sfc32x16_new,
        sfc32x16_free,
        sfc32x16_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        sfc32x16_new,
        sfc32x16_free,
        |p, o, n| unsafe { sfc32x16_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        sfc32x16_new,
        sfc32x16_free,
        |p, o, n| unsafe { sfc32x16_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        sfc32x16_new,
        sfc32x16_free,
        |p, o, n| unsafe { sfc32x16_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn splitmix32_boundaries_alignment_and_partition() {
    let mut r = SplitMix32::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        splitmix32_new,
        splitmix32_free,
        splitmix32_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        splitmix32_new,
        splitmix32_free,
        |p, o, n| unsafe { splitmix32_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        splitmix32_new,
        splitmix32_free,
        |p, o, n| unsafe { splitmix32_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        splitmix32_new,
        splitmix32_free,
        |p, o, n| unsafe { splitmix32_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[test]
fn splitmix32x16_boundaries_alignment_and_partition() {
    if !std::is_x86_feature_detected!("avx512f") {
        assert!(splitmix32x16_new(1).is_null());
        return;
    }
    let mut r = unsafe { SplitMix32x16::new(1) };
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.extend(unsafe { std::mem::transmute::<_, [u32; 16]>(r.nextuv()) });
    }
    check(
        1,
        splitmix32x16_new,
        splitmix32x16_free,
        splitmix32x16_next_u32s,
        &expected,
        u32::MAX,
    );
}
#[test]
fn xoroshiro64ss_boundaries_alignment_and_partition() {
    let mut r = Xoroshiro64Ss::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        xoroshiro64ss_new,
        xoroshiro64ss_free,
        xoroshiro64ss_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        xoroshiro64ss_new,
        xoroshiro64ss_free,
        |p, o, n| unsafe { xoroshiro64ss_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        xoroshiro64ss_new,
        xoroshiro64ss_free,
        |p, o, n| unsafe { xoroshiro64ss_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        xoroshiro64ss_new,
        xoroshiro64ss_free,
        |p, o, n| unsafe { xoroshiro64ss_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[test]
fn xoroshiro64ssx8_boundaries_alignment_and_partition() {
    if !std::is_x86_feature_detected!("avx2") {
        assert!(xoroshiro64ssx8_new(1).is_null());
        return;
    }
    let mut r = unsafe { Xoroshiro64Ssx8::new(1) };
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.extend(unsafe { std::mem::transmute::<_, [u32; 8]>(r.nextuv()) });
    }
    check(
        1,
        xoroshiro64ssx8_new,
        xoroshiro64ssx8_free,
        xoroshiro64ssx8_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        xoroshiro64ssx8_new,
        xoroshiro64ssx8_free,
        |p, o, n| unsafe { xoroshiro64ssx8_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        xoroshiro64ssx8_new,
        xoroshiro64ssx8_free,
        |p, o, n| unsafe { xoroshiro64ssx8_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        xoroshiro64ssx8_new,
        xoroshiro64ssx8_free,
        |p, o, n| unsafe { xoroshiro64ssx8_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[test]
fn xoroshiro64ssx16_boundaries_alignment_and_partition() {
    if !std::is_x86_feature_detected!("avx512f") {
        assert!(xoroshiro64ssx16_new(1).is_null());
        return;
    }
    let mut r = unsafe { Xoroshiro64Ssx16::new(1) };
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.extend(unsafe { std::mem::transmute::<_, [u32; 16]>(r.nextuv()) });
    }
    check(
        1,
        xoroshiro64ssx16_new,
        xoroshiro64ssx16_free,
        xoroshiro64ssx16_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        xoroshiro64ssx16_new,
        xoroshiro64ssx16_free,
        |p, o, n| unsafe { xoroshiro64ssx16_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        xoroshiro64ssx16_new,
        xoroshiro64ssx16_free,
        |p, o, n| unsafe { xoroshiro64ssx16_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        xoroshiro64ssx16_new,
        xoroshiro64ssx16_free,
        |p, o, n| unsafe { xoroshiro64ssx16_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn xorshift32_boundaries_alignment_and_partition() {
    let mut r = Xorshift32::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        xorshift32_new,
        xorshift32_free,
        xorshift32_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        xorshift32_new,
        xorshift32_free,
        |p, o, n| unsafe { xorshift32_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        xorshift32_new,
        xorshift32_free,
        |p, o, n| unsafe { xorshift32_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        xorshift32_new,
        xorshift32_free,
        |p, o, n| unsafe { xorshift32_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn xorshift128_boundaries_alignment_and_partition() {
    let mut r = Xorshift128::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        xorshift128_new,
        xorshift128_free,
        xorshift128_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        xorshift128_new,
        xorshift128_free,
        |p, o, n| unsafe { xorshift128_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        xorshift128_new,
        xorshift128_free,
        |p, o, n| unsafe { xorshift128_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        xorshift128_new,
        xorshift128_free,
        |p, o, n| unsafe { xorshift128_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn xoshiro128pp_boundaries_alignment_and_partition() {
    let mut r = Xoshiro128Pp::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        xoshiro128pp_new,
        xoshiro128pp_free,
        xoshiro128pp_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        xoshiro128pp_new,
        xoshiro128pp_free,
        |p, o, n| unsafe { xoshiro128pp_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        xoshiro128pp_new,
        xoshiro128pp_free,
        |p, o, n| unsafe { xoshiro128pp_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        xoshiro128pp_new,
        xoshiro128pp_free,
        |p, o, n| unsafe { xoshiro128pp_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn xoshiro128ss_boundaries_alignment_and_partition() {
    let mut r = Xoshiro128Ss::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        xoshiro128ss_new,
        xoshiro128ss_free,
        xoshiro128ss_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        xoshiro128ss_new,
        xoshiro128ss_free,
        |p, o, n| unsafe { xoshiro128ss_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        xoshiro128ss_new,
        xoshiro128ss_free,
        |p, o, n| unsafe { xoshiro128ss_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        xoshiro128ss_new,
        xoshiro128ss_free,
        |p, o, n| unsafe { xoshiro128ss_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[test]
fn xoshiro128ppx16_boundaries_alignment_and_partition() {
    if !std::is_x86_feature_detected!("avx512f") {
        assert!(xoshiro128ppx16_new(1).is_null());
        return;
    }
    let mut r = unsafe { Xoshiro128Ppx16::new(1) };
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.extend(unsafe { std::mem::transmute::<_, [u32; 16]>(r.nextuv()) });
    }
    check(
        1,
        xoshiro128ppx16_new,
        xoshiro128ppx16_free,
        xoshiro128ppx16_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        xoshiro128ppx16_new,
        xoshiro128ppx16_free,
        |p, o, n| unsafe { xoshiro128ppx16_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        xoshiro128ppx16_new,
        xoshiro128ppx16_free,
        |p, o, n| unsafe { xoshiro128ppx16_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        xoshiro128ppx16_new,
        xoshiro128ppx16_free,
        |p, o, n| unsafe { xoshiro128ppx16_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[test]
fn xoshiro128ssx16_boundaries_alignment_and_partition() {
    if !std::is_x86_feature_detected!("avx512f") {
        assert!(xoshiro128ssx16_new(1).is_null());
        return;
    }
    let mut r = unsafe { Xoshiro128Ssx16::new(1) };
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.extend(unsafe { std::mem::transmute::<_, [u32; 16]>(r.nextuv()) });
    }
    check(
        1,
        xoshiro128ssx16_new,
        xoshiro128ssx16_free,
        xoshiro128ssx16_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        xoshiro128ssx16_new,
        xoshiro128ssx16_free,
        |p, o, n| unsafe { xoshiro128ssx16_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        xoshiro128ssx16_new,
        xoshiro128ssx16_free,
        |p, o, n| unsafe { xoshiro128ssx16_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        xoshiro128ssx16_new,
        xoshiro128ssx16_free,
        |p, o, n| unsafe { xoshiro128ssx16_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn biski64_boundaries_alignment_and_partition() {
    let mut r = Biski64::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        biski64_new,
        biski64_free,
        biski64_next_u64s,
        &expected,
        u64::MAX,
    );
    let converted: Vec<f64> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        biski64_new,
        biski64_free,
        |p, o, n| unsafe { biski64_next_f64s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i64::MIN, i64::MAX))
        .collect();
    check_with(
        1,
        biski64_new,
        biski64_free,
        |p, o, n| unsafe { biski64_rand_i64s(p, o, n, i64::MIN, i64::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f64::MAX, f64::MAX))
        .collect();
    check_with(
        1,
        biski64_new,
        biski64_free,
        |p, o, n| unsafe { biski64_rand_f64s(p, o, n, -f64::MAX, f64::MAX) },
        &converted,
        -999.0,
    );
}
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[test]
fn biski64x8_boundaries_alignment_and_partition() {
    if !std::is_x86_feature_detected!("avx512f") {
        assert!(biski64x8_new(1).is_null());
        return;
    }
    let mut r = Biski64x8::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.extend(unsafe { r.nextu() });
    }
    check(
        1,
        biski64x8_new,
        biski64x8_free,
        biski64x8_next_u64s,
        &expected,
        u64::MAX,
    );
    let converted: Vec<f64> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        biski64x8_new,
        biski64x8_free,
        |p, o, n| unsafe { biski64x8_next_f64s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i64::MIN, i64::MAX))
        .collect();
    check_with(
        1,
        biski64x8_new,
        biski64x8_free,
        |p, o, n| unsafe { biski64x8_rand_i64s(p, o, n, i64::MIN, i64::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f64::MAX, f64::MAX))
        .collect();
    check_with(
        1,
        biski64x8_new,
        biski64x8_free,
        |p, o, n| unsafe { biski64x8_rand_f64s(p, o, n, -f64::MAX, f64::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn cet64_boundaries_alignment_and_partition() {
    let mut r = Cet64::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        cet64_new,
        cet64_free,
        cet64_next_u64s,
        &expected,
        u64::MAX,
    );
    let converted: Vec<f64> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        cet64_new,
        cet64_free,
        |p, o, n| unsafe { cet64_next_f64s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i64::MIN, i64::MAX))
        .collect();
    check_with(
        1,
        cet64_new,
        cet64_free,
        |p, o, n| unsafe { cet64_rand_i64s(p, o, n, i64::MIN, i64::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f64::MAX, f64::MAX))
        .collect();
    check_with(
        1,
        cet64_new,
        cet64_free,
        |p, o, n| unsafe { cet64_rand_f64s(p, o, n, -f64::MAX, f64::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn cet256_boundaries_alignment_and_partition() {
    let mut r = Cet256::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        cet256_new,
        cet256_free,
        cet256_next_u64s,
        &expected,
        u64::MAX,
    );
    let converted: Vec<f64> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        cet256_new,
        cet256_free,
        |p, o, n| unsafe { cet256_next_f64s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i64::MIN, i64::MAX))
        .collect();
    check_with(
        1,
        cet256_new,
        cet256_free,
        |p, o, n| unsafe { cet256_rand_i64s(p, o, n, i64::MIN, i64::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f64::MAX, f64::MAX))
        .collect();
    check_with(
        1,
        cet256_new,
        cet256_free,
        |p, o, n| unsafe { cet256_rand_f64s(p, o, n, -f64::MAX, f64::MAX) },
        &converted,
        -999.0,
    );
}
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[test]
fn cet64x8_boundaries_alignment_and_partition() {
    if !std::is_x86_feature_detected!("avx512dq") || !std::is_x86_feature_detected!("avx512f") {
        assert!(cet64x8_new(1).is_null());
        return;
    }
    let mut r = Cet64x8::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.extend(unsafe { r.nextu() });
    }
    check(
        1,
        cet64x8_new,
        cet64x8_free,
        cet64x8_next_u64s,
        &expected,
        u64::MAX,
    );
}
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[test]
fn cet256x2_boundaries_alignment_and_partition() {
    if !std::is_x86_feature_detected!("avx512dq") || !std::is_x86_feature_detected!("avx512f") {
        assert!(cet256x2_new(1).is_null());
        return;
    }
    let mut r = Cet256x2::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.extend(unsafe { r.nextu() });
    }
    check(
        1,
        cet256x2_new,
        cet256x2_free,
        cet256x2_next_u64s,
        &expected,
        u64::MAX,
    );
}
#[test]
fn mt1993764_boundaries_alignment_and_partition() {
    let mut r = Mt1993764::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        mt1993764_new,
        mt1993764_free,
        mt1993764_next_u64s,
        &expected,
        u64::MAX,
    );
    let converted: Vec<f64> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        mt1993764_new,
        mt1993764_free,
        |p, o, n| unsafe { mt1993764_next_f64s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i64::MIN, i64::MAX))
        .collect();
    check_with(
        1,
        mt1993764_new,
        mt1993764_free,
        |p, o, n| unsafe { mt1993764_rand_i64s(p, o, n, i64::MIN, i64::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f64::MAX, f64::MAX))
        .collect();
    check_with(
        1,
        mt1993764_new,
        mt1993764_free,
        |p, o, n| unsafe { mt1993764_rand_f64s(p, o, n, -f64::MAX, f64::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn sfmt1993764_boundaries_alignment_and_partition() {
    let mut r = Sfmt1993764::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        sfmt1993764_new,
        sfmt1993764_free,
        sfmt1993764_next_u64s,
        &expected,
        u64::MAX,
    );
    let converted: Vec<f64> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        sfmt1993764_new,
        sfmt1993764_free,
        |p, o, n| unsafe { sfmt1993764_next_f64s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i64::MIN, i64::MAX))
        .collect();
    check_with(
        1,
        sfmt1993764_new,
        sfmt1993764_free,
        |p, o, n| unsafe { sfmt1993764_rand_i64s(p, o, n, i64::MIN, i64::MAX) },
        &converted,
        123,
    );
}
#[test]
fn sfc64_boundaries_alignment_and_partition() {
    let mut r = Sfc64::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        sfc64_new,
        sfc64_free,
        sfc64_next_u64s,
        &expected,
        u64::MAX,
    );
    let converted: Vec<f64> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        sfc64_new,
        sfc64_free,
        |p, o, n| unsafe { sfc64_next_f64s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i64::MIN, i64::MAX))
        .collect();
    check_with(
        1,
        sfc64_new,
        sfc64_free,
        |p, o, n| unsafe { sfc64_rand_i64s(p, o, n, i64::MIN, i64::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f64::MAX, f64::MAX))
        .collect();
    check_with(
        1,
        sfc64_new,
        sfc64_free,
        |p, o, n| unsafe { sfc64_rand_f64s(p, o, n, -f64::MAX, f64::MAX) },
        &converted,
        -999.0,
    );
}
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[test]
fn sfc64x8_boundaries_alignment_and_partition() {
    if !std::is_x86_feature_detected!("avx512f") {
        assert!(sfc64x8_new(1).is_null());
        return;
    }
    let mut r = Sfc64x8::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.extend(unsafe { r.nextu() });
    }
    check(
        1,
        sfc64x8_new,
        sfc64x8_free,
        sfc64x8_next_u64s,
        &expected,
        u64::MAX,
    );
}
#[test]
fn splitmix64_boundaries_alignment_and_partition() {
    let mut r = SplitMix64::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        splitmix64_new,
        splitmix64_free,
        splitmix64_next_u64s,
        &expected,
        u64::MAX,
    );
    let converted: Vec<f64> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        splitmix64_new,
        splitmix64_free,
        |p, o, n| unsafe { splitmix64_next_f64s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i64::MIN, i64::MAX))
        .collect();
    check_with(
        1,
        splitmix64_new,
        splitmix64_free,
        |p, o, n| unsafe { splitmix64_rand_i64s(p, o, n, i64::MIN, i64::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f64::MAX, f64::MAX))
        .collect();
    check_with(
        1,
        splitmix64_new,
        splitmix64_free,
        |p, o, n| unsafe { splitmix64_rand_f64s(p, o, n, -f64::MAX, f64::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn twisted_gfsr_boundaries_alignment_and_partition() {
    let mut r = TwistedGFSR::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        twisted_gfsr_new,
        twisted_gfsr_free,
        twisted_gfsr_next_u64s,
        &expected,
        u64::MAX,
    );
    let converted: Vec<f64> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        twisted_gfsr_new,
        twisted_gfsr_free,
        |p, o, n| unsafe { twisted_gfsr_next_f64s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i64::MIN, i64::MAX))
        .collect();
    check_with(
        1,
        twisted_gfsr_new,
        twisted_gfsr_free,
        |p, o, n| unsafe { twisted_gfsr_rand_i64s(p, o, n, i64::MIN, i64::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f64::MAX, f64::MAX))
        .collect();
    check_with(
        1,
        twisted_gfsr_new,
        twisted_gfsr_free,
        |p, o, n| unsafe { twisted_gfsr_rand_f64s(p, o, n, -f64::MAX, f64::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn xoroshiro128pp_boundaries_alignment_and_partition() {
    let mut r = Xoroshiro128Pp::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        xoroshiro128pp_new,
        xoroshiro128pp_free,
        xoroshiro128pp_next_u64s,
        &expected,
        u64::MAX,
    );
    let converted: Vec<f64> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        xoroshiro128pp_new,
        xoroshiro128pp_free,
        |p, o, n| unsafe { xoroshiro128pp_next_f64s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i64::MIN, i64::MAX))
        .collect();
    check_with(
        1,
        xoroshiro128pp_new,
        xoroshiro128pp_free,
        |p, o, n| unsafe { xoroshiro128pp_rand_i64s(p, o, n, i64::MIN, i64::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f64::MAX, f64::MAX))
        .collect();
    check_with(
        1,
        xoroshiro128pp_new,
        xoroshiro128pp_free,
        |p, o, n| unsafe { xoroshiro128pp_rand_f64s(p, o, n, -f64::MAX, f64::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn xoroshiro128ss_boundaries_alignment_and_partition() {
    let mut r = Xoroshiro128Ss::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        xoroshiro128ss_new,
        xoroshiro128ss_free,
        xoroshiro128ss_next_u64s,
        &expected,
        u64::MAX,
    );
    let converted: Vec<f64> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        xoroshiro128ss_new,
        xoroshiro128ss_free,
        |p, o, n| unsafe { xoroshiro128ss_next_f64s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i64::MIN, i64::MAX))
        .collect();
    check_with(
        1,
        xoroshiro128ss_new,
        xoroshiro128ss_free,
        |p, o, n| unsafe { xoroshiro128ss_rand_i64s(p, o, n, i64::MIN, i64::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f64::MAX, f64::MAX))
        .collect();
    check_with(
        1,
        xoroshiro128ss_new,
        xoroshiro128ss_free,
        |p, o, n| unsafe { xoroshiro128ss_rand_f64s(p, o, n, -f64::MAX, f64::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn xorshift64_boundaries_alignment_and_partition() {
    let mut r = Xorshift64::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        xorshift64_new,
        xorshift64_free,
        xorshift64_next_u64s,
        &expected,
        u64::MAX,
    );
    let converted: Vec<f64> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        xorshift64_new,
        xorshift64_free,
        |p, o, n| unsafe { xorshift64_next_f64s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i64::MIN, i64::MAX))
        .collect();
    check_with(
        1,
        xorshift64_new,
        xorshift64_free,
        |p, o, n| unsafe { xorshift64_rand_i64s(p, o, n, i64::MIN, i64::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f64::MAX, f64::MAX))
        .collect();
    check_with(
        1,
        xorshift64_new,
        xorshift64_free,
        |p, o, n| unsafe { xorshift64_rand_f64s(p, o, n, -f64::MAX, f64::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn xoshiro256pp_boundaries_alignment_and_partition() {
    let mut r = Xoshiro256Pp::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        xoshiro256pp_new,
        xoshiro256pp_free,
        xoshiro256pp_next_u64s,
        &expected,
        u64::MAX,
    );
    let converted: Vec<f64> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        xoshiro256pp_new,
        xoshiro256pp_free,
        |p, o, n| unsafe { xoshiro256pp_next_f64s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i64::MIN, i64::MAX))
        .collect();
    check_with(
        1,
        xoshiro256pp_new,
        xoshiro256pp_free,
        |p, o, n| unsafe { xoshiro256pp_rand_i64s(p, o, n, i64::MIN, i64::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f64::MAX, f64::MAX))
        .collect();
    check_with(
        1,
        xoshiro256pp_new,
        xoshiro256pp_free,
        |p, o, n| unsafe { xoshiro256pp_rand_f64s(p, o, n, -f64::MAX, f64::MAX) },
        &converted,
        -999.0,
    );
}
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[test]
fn xoshiro256ssx2_boundaries_alignment_and_partition() {
    let mut r = Xoshiro256Ssx2::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.extend(r.nextu());
    }
    check(
        1,
        xoshiro256ssx2_new,
        xoshiro256ssx2_free,
        xoshiro256ssx2_next_u64s,
        &expected,
        u64::MAX,
    );
}
#[test]
fn xoshiro256ss_boundaries_alignment_and_partition() {
    let mut r = Xoshiro256Ss::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        xoshiro256ss_new,
        xoshiro256ss_free,
        xoshiro256ss_next_u64s,
        &expected,
        u64::MAX,
    );
    let converted: Vec<f64> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        xoshiro256ss_new,
        xoshiro256ss_free,
        |p, o, n| unsafe { xoshiro256ss_next_f64s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i64::MIN, i64::MAX))
        .collect();
    check_with(
        1,
        xoshiro256ss_new,
        xoshiro256ss_free,
        |p, o, n| unsafe { xoshiro256ss_rand_i64s(p, o, n, i64::MIN, i64::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f64::MAX, f64::MAX))
        .collect();
    check_with(
        1,
        xoshiro256ss_new,
        xoshiro256ss_free,
        |p, o, n| unsafe { xoshiro256ss_rand_f64s(p, o, n, -f64::MAX, f64::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn philox32_boundaries_alignment_and_partition() {
    let mut r = Philox32::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        philox32_new,
        philox32_free,
        philox32_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        philox32_new,
        philox32_free,
        |p, o, n| unsafe { philox32_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        philox32_new,
        philox32_free,
        |p, o, n| unsafe { philox32_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        philox32_new,
        philox32_free,
        |p, o, n| unsafe { philox32_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[test]
fn philox32x4x4_boundaries_alignment_and_partition() {
    if !std::is_x86_feature_detected!("avx512f") {
        assert!(philox32x4x4_new(1).is_null());
        return;
    }
    let mut r = unsafe { Philox32x4x4::new(1) };
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.extend(unsafe { r.nextu() });
    }
    check(
        1,
        philox32x4x4_new,
        philox32x4x4_free,
        philox32x4x4_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        philox32x4x4_new,
        philox32x4x4_free,
        |p, o, n| unsafe { philox32x4x4_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        philox32x4x4_new,
        philox32x4x4_free,
        |p, o, n| unsafe { philox32x4x4_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        philox32x4x4_new,
        philox32x4x4_free,
        |p, o, n| unsafe { philox32x4x4_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn squares32_boundaries_alignment_and_partition() {
    let mut r = Squares32::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        squares32_new,
        squares32_free,
        squares32_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        squares32_new,
        squares32_free,
        |p, o, n| unsafe { squares32_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        squares32_new,
        squares32_free,
        |p, o, n| unsafe { squares32_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        squares32_new,
        squares32_free,
        |p, o, n| unsafe { squares32_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[test]
fn squares32x8_boundaries_alignment_and_partition() {
    if !std::is_x86_feature_detected!("avx512dq") || !std::is_x86_feature_detected!("avx512f") {
        assert!(squares32x8_new(1).is_null());
        return;
    }
    let mut r = unsafe { Squares32x8::new(1) };
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.extend(unsafe { r.nextu() });
    }
    check(
        1,
        squares32x8_new,
        squares32x8_free,
        squares32x8_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        squares32x8_new,
        squares32x8_free,
        |p, o, n| unsafe { squares32x8_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        squares32x8_new,
        squares32x8_free,
        |p, o, n| unsafe { squares32x8_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        squares32x8_new,
        squares32x8_free,
        |p, o, n| unsafe { squares32x8_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn threefry32x4_boundaries_alignment_and_partition() {
    let mut r = Threefry32x4::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        threefry32x4_new,
        threefry32x4_free,
        threefry32x4_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        threefry32x4_new,
        threefry32x4_free,
        |p, o, n| unsafe { threefry32x4_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        threefry32x4_new,
        threefry32x4_free,
        |p, o, n| unsafe { threefry32x4_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f32::MAX, f32::MAX))
        .collect();
    check_with(
        1,
        threefry32x4_new,
        threefry32x4_free,
        |p, o, n| unsafe { threefry32x4_rand_f32s(p, o, n, -f32::MAX, f32::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn threefry32x2_boundaries_alignment_and_partition() {
    let mut r = Threefry32x2::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        threefry32x2_new,
        threefry32x2_free,
        threefry32x2_next_u32s,
        &expected,
        u32::MAX,
    );
    let converted: Vec<f32> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        threefry32x2_new,
        threefry32x2_free,
        |p, o, n| unsafe { threefry32x2_next_f32s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i32> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i32::MIN, i32::MAX))
        .collect();
    check_with(
        1,
        threefry32x2_new,
        threefry32x2_free,
        |p, o, n| unsafe { threefry32x2_rand_i32s(p, o, n, i32::MIN, i32::MAX) },
        &converted,
        123,
    );
}
#[test]
fn philox64_boundaries_alignment_and_partition() {
    let mut r = Philox64::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        philox64_new,
        philox64_free,
        philox64_next_u64s,
        &expected,
        u64::MAX,
    );
    let converted: Vec<f64> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        philox64_new,
        philox64_free,
        |p, o, n| unsafe { philox64_next_f64s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i64::MIN, i64::MAX))
        .collect();
    check_with(
        1,
        philox64_new,
        philox64_free,
        |p, o, n| unsafe { philox64_rand_i64s(p, o, n, i64::MIN, i64::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f64::MAX, f64::MAX))
        .collect();
    check_with(
        1,
        philox64_new,
        philox64_free,
        |p, o, n| unsafe { philox64_rand_f64s(p, o, n, -f64::MAX, f64::MAX) },
        &converted,
        -999.0,
    );
}
#[test]
fn threefish256_boundaries_alignment_and_partition() {
    let mut r = Threefish256::new(1);
    let mut expected = Vec::new();
    while expected.len() < 1024 {
        expected.push(r.nextu());
    }
    check(
        1,
        threefish256_new,
        threefish256_free,
        threefish256_next_u64s,
        &expected,
        u64::MAX,
    );
    let converted: Vec<f64> = expected.iter().copied().map(Word::to_f01).collect();
    check_with(
        1,
        threefish256_new,
        threefish256_free,
        |p, o, n| unsafe { threefish256_next_f64s(p, o, n) },
        &converted,
        -999.0,
    );
    let converted: Vec<i64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randi(i64::MIN, i64::MAX))
        .collect();
    check_with(
        1,
        threefish256_new,
        threefish256_free,
        |p, o, n| unsafe { threefish256_rand_i64s(p, o, n, i64::MIN, i64::MAX) },
        &converted,
        123,
    );
    let converted: Vec<f64> = expected
        .iter()
        .copied()
        .map(|x| x.to_randf(-f64::MAX, f64::MAX))
        .collect();
    check_with(
        1,
        threefish256_new,
        threefish256_free,
        |p, o, n| unsafe { threefish256_rand_f64s(p, o, n, -f64::MAX, f64::MAX) },
        &converted,
        -999.0,
    );
}
