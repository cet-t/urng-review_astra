//! Fixed before/after benchmark. Build against either release urng rlib.
use std::hint::black_box;
use std::time::Instant;
use urng::{Rng, wide::RngW};

const N: usize = 1_048_576;
const RUNS: usize = 9;
fn measure(name: &str, words: usize, mut run: impl FnMut()) {
    run();
    let mut samples = Vec::new();
    for _ in 0..RUNS {
        let start = Instant::now();
        run();
        samples.push(start.elapsed().as_secs_f64() * 1e9 / words as f64);
    }
    samples.sort_by(f64::total_cmp);
    println!(
        "{name},{:.4},{:.4},{:.4}",
        samples[RUNS / 2],
        samples[0],
        samples[RUNS - 1]
    );
}
fn scalar32<T: Rng<Word = u32>>(name: &str, mut new: impl FnMut() -> T) {
    measure(name, N, || {
        let mut r = new();
        for _ in 0..N {
            black_box(r.nextu());
        }
    });
}
fn scalar64<T: Rng<Word = u64>>(name: &str, mut new: impl FnMut() -> T) {
    measure(name, N, || {
        let mut r = new();
        for _ in 0..N {
            black_box(r.nextu());
        }
    });
}
fn main() {
    println!("name,median_ns_per_word,min_ns_per_word,max_ns_per_word");
    scalar32("pcg32", || urng::Pcg32::new(1));
    scalar32("philox32", || urng::Philox32::new(1));
    scalar32("chacha20", || urng::ChaCha20::new(1));
    scalar64("sfc64", || urng::Sfc64::new(1));
    scalar64("biski64", || urng::Biski64::new(1));
    scalar64("xoshiro256pp", || urng::Xoshiro256Pp::new(1));
    scalar64("xoroshiro128pp", || urng::Xoroshiro128Pp::new(1));
    measure("pcg32_randi", N, || {
        let mut r = urng::Pcg32::new(1);
        for _ in 0..N {
            black_box(r.randi(black_box(-5), black_box(7)));
        }
    });
    measure("sfc64_randi", N, || {
        let mut r = urng::Sfc64::new(1);
        for _ in 0..N {
            black_box(r.randi(black_box(-5), black_box(7)));
        }
    });
    measure("sfc64_randf", N, || {
        let mut r = urng::Sfc64::new(1);
        for _ in 0..N {
            black_box(r.randf(black_box(-1.0), black_box(1.0)));
        }
    });
    let mut buf = vec![0u32; N];
    measure("wide_splitmix_x4_fill", N, || {
        let mut r = urng::wide::SplitMix32x4::new(1);
        for dst in buf.chunks_exact_mut(4) {
            dst.copy_from_slice(&r.nextu());
        }
        black_box(&buf);
    });
    measure("cabi_pcg32_fill", N, || unsafe {
        let p = urng::cabi::pcg32_new(1);
        urng::cabi::pcg32_next_u32s(p, buf.as_mut_ptr(), N);
        urng::cabi::pcg32_free(p);
        black_box(&buf);
    });
    if std::is_x86_feature_detected!("avx512f") {
        let mut buf = vec![0u64; N];
        measure("cabi_sfc64x8_fill", N, || unsafe {
            let p = urng::cabi::sfc64x8_new(1);
            urng::cabi::sfc64x8_next_u64s(p, buf.as_mut_ptr(), N);
            urng::cabi::sfc64x8_free(p);
            black_box(&buf);
        });
    }
}
