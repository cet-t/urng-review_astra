//! Independent scalar recurrences transcribed from the cited reference sources.
#![allow(dead_code)]
pub fn sm64(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}
// urng's documented 32-bit Weyl state and 64-bit finalizer (not SplitMix64).
pub fn sm32(s: &mut u32) -> u32 {
    *s = s.wrapping_add(0x9e3779b9);
    let a = (*s as u64 ^ (*s as u64 >> 16)).wrapping_mul(0xff51afd7ed558ccd);
    let b = (a ^ (a >> 16)).wrapping_mul(0xc4ceb9fe1a85ec53);
    (b ^ (b >> 16)) as u32
}
// https://www.pcg-random.org/download.html
pub fn pcg(s: &mut u64, inc: u64) -> u32 {
    let old = *s;
    *s = old.wrapping_mul(6364136223846793005).wrapping_add(inc);
    ((((old >> 18) ^ old) >> 27) as u32).rotate_right((old >> 59) as u32)
}
// https://ffmpeg.org/doxygen/8.0/sfc64_8h_source.html
pub fn sfc(s: &mut [u64; 4]) -> u64 {
    let [a, b, c, d] = *s;
    let output = a.wrapping_add(b).wrapping_add(d);
    *s = [
        b ^ (b >> 11),
        c.wrapping_mul(9),
        c.rotate_left(24).wrapping_add(output),
        d.wrapping_add(1),
    ];
    output
}
// https://github.com/danielcota/biski64 (simultaneous assignment).
pub fn biski(s: &mut [u64; 3]) -> u64 {
    let [a, b, c] = *s;
    *s = [
        a.wrapping_add(0x9999999999999999),
        b.rotate_left(16).wrapping_add(c.rotate_left(40)),
        a ^ b,
    ];
    b.wrapping_add(c)
}
// https://prng.di.unimi.it/xoroshiro128plusplus.c and xoroshiro128starstar.c
pub fn xoroshiro(s: &mut [u64; 2], pp: bool) -> u64 {
    let [a, b] = *s;
    let t = a ^ b;
    if pp {
        *s = [a.rotate_left(49) ^ t ^ (t << 21), t.rotate_left(28)];
        a.wrapping_add(b).rotate_left(17).wrapping_add(a)
    } else {
        *s = [a.rotate_left(24) ^ t ^ (t << 16), t.rotate_left(37)];
        a.wrapping_mul(5).rotate_left(7).wrapping_mul(9)
    }
}
pub fn jump(s: &mut [u64; 2], polynomial: [u64; 2], pp: bool) {
    let mut out = [0; 2];
    for coefficient in polynomial {
        for bit in 0..64 {
            if coefficient & (1u64 << bit) != 0 {
                out[0] ^= s[0];
                out[1] ^= s[1];
            }
            xoroshiro(s, pp);
        }
    }
    *s = out;
}
// https://prng.di.unimi.it/xoroshiro64starstar.c
pub fn xoroshiro32(s: &mut [u32; 2]) -> u32 {
    let [a, b] = *s;
    let t = a ^ b;
    *s = [a.rotate_left(26) ^ t ^ (t << 9), t.rotate_left(13)];
    a.wrapping_mul(0x9e3779bb).rotate_left(5).wrapping_mul(5)
}
// Random123 Philox4x32-10; KATs separately ensure reference transcription.
pub fn philox(mut counter: [u32; 4], mut key: [u32; 2]) -> [u32; 4] {
    for round in 0..10 {
        let p = (counter[0] as u64) * 0xd2511f53;
        let q = (counter[2] as u64) * 0xcd9e8d57;
        counter = [
            (q >> 32) as u32 ^ counter[1] ^ key[0],
            q as u32,
            (p >> 32) as u32 ^ counter[3] ^ key[1],
            p as u32,
        ];
        if round < 9 {
            key[0] = key[0].wrapping_add(0x9e3779b9);
            key[1] = key[1].wrapping_add(0xbb67ae85);
        }
    }
    counter
}
