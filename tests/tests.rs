use core::hash::Hasher;
use mur3::*;
use mur3_c::*;
use quickcheck_macros::quickcheck;

const DATA: &[(u32, u32, u64, u64, &str)] = &[
    (0x00, 0x00000000, 0x0000000000000000, 0x0000000000000000, ""),
    (
        0x00,
        0x248bfa47,
        0xcbd8a7b341bd9b02,
        0x5b1e906a48ae1d19,
        "hello",
    ),
    (
        0x00,
        0x149bbb7f,
        0x342fac623a5ebc8e,
        0x4cdcbc079642414d,
        "hello, world",
    ),
    (
        0x00,
        0xe31e8a70,
        0xb89e5988b737affc,
        0x664fc2950231b2cb,
        "19 Jan 2038 at 3:14:07 AM",
    ),
    (
        0x00,
        0xd5c48bfc,
        0xcd99481f9ee902c9,
        0x695da1a38987b6e7,
        "The quick brown fox jumps over the lazy dog.",
    ),
    (0x01, 0x514e28b7, 0x4610abe56eff5cb5, 0x51622daa78f83583, ""),
    (
        0x01,
        0xbb4abcad,
        0xa78ddff5adae8d10,
        0x128900ef20900135,
        "hello",
    ),
    (
        0x01,
        0x6f5cb2e9,
        0x8b95f808840725c6,
        0x1597ed5422bd493b,
        "hello, world",
    ),
    (
        0x01,
        0xf50e1f30,
        0x2a929de9c8f97b2f,
        0x56a41d99af43a2db,
        "19 Jan 2038 at 3:14:07 AM",
    ),
    (
        0x01,
        0x846f6a36,
        0xfb3325171f9744da,
        0xaaf8b92a5f722952,
        "The quick brown fox jumps over the lazy dog.",
    ),
    (0x2a, 0x087fcd5c, 0xf02aa77dfa1b8523, 0xd1016610da11cbb9, ""),
    (
        0x2a,
        0xe2dbd2e1,
        0xc4b8b3c960af6f08,
        0x2334b875b0efbc7a,
        "hello",
    ),
    (
        0x2a,
        0x7ec7c6c2,
        0xb91864d797caa956,
        0xd5d139a55afe6150,
        "hello, world",
    ),
    (
        0x2a,
        0x58f745f6,
        0xfd8f19ebdc8c6b6a,
        0xd30fdc310fa08ff9,
        "19 Jan 2038 at 3:14:07 AM",
    ),
    (
        0x2a,
        0xc02d1434,
        0x74f33c659cda5af7,
        0x4ec7a891caf316f0,
        "The quick brown fox jumps over the lazy dog.",
    ),
];

#[test]
fn test_strings() {
    for (seed, h32, h64_1, h64_2, s) in DATA {
        let (h1, h2) = murmurhash3_x64_128(s.as_bytes(), *seed);
        assert_eq!((h1, h2), (*h64_1, *h64_2), "key: {}, seed: {:0x}", s, seed);

        let mut hasher = Hasher128::with_seed(*seed);
        hasher.write(s.as_bytes());
        assert_eq!(
            hasher.finish128(),
            (*h64_1, *h64_2),
            "key: {}, seed: {:0x}",
            s,
            seed
        );
        assert_eq!(hasher.finish(), *h64_1, "key: {}, seed: {:0x}", s, seed);

        let h = murmurhash3_x86_32(s.as_bytes(), *seed);
        assert_eq!(h, *h32, "key: {}, seed: {:0x}", s, seed);

        let mut hasher = Hasher32::with_seed(*seed);
        hasher.write(s.as_bytes());
        assert_eq!(
            hasher.finish(),
            *h32 as u64,
            "key: {}, seed: {:0x}",
            s,
            seed
        );
    }
}

#[quickcheck]
fn random_check_32(xs: Vec<u8>) -> bool {
    let func_res = murmurhash3_x86_32(&xs, 0);
    let mut hasher32 = Hasher32::with_seed(0);
    hasher32.write(&xs);
    let hash_res = hasher32.finish32();
    let c_res = hash32(&xs, 0);
    func_res == hash_res && hash_res == c_res
}

#[quickcheck]
fn random_check_128(xs: Vec<u8>) -> bool {
    let func_res = murmurhash3_x64_128(&xs, 0);
    let mut hasher = Hasher128::with_seed(0);
    hasher.write(&xs);
    let hash_res = hasher.finish128();
    let c_res = hash128_64(&xs, 0);
    func_res == hash_res && hash_res == c_res
}

#[quickcheck]
fn random_check_32_seed(xs: Vec<u8>, seed: u32) -> bool {
    let func_res = murmurhash3_x86_32(&xs, seed);
    let mut hasher32 = Hasher32::with_seed(seed);
    hasher32.write(&xs);
    let hash_res = hasher32.finish32();
    let c_res = hash32(&xs, seed);
    func_res == hash_res && hash_res == c_res
}

#[quickcheck]
fn random_check_128_seed(xs: Vec<u8>, seed: u32) -> bool {
    let func_res = murmurhash3_x64_128(&xs, seed);
    let mut hasher = Hasher128::with_seed(seed);
    hasher.write(&xs);
    let hash_res = hasher.finish128();
    let c_res = hash128_64(&xs, seed);
    func_res == hash_res && hash_res == c_res
}

#[quickcheck]
fn random_check_32_chunks(xs: Vec<Vec<u8>>, seed: u32) -> bool {
    let mut all_bytes = vec![];
    for c in &xs {
        all_bytes.extend_from_slice(c);
    }
    let func_res = murmurhash3_x86_32(&all_bytes, seed);
    let mut hasher32 = Hasher32::with_seed(seed);
    for x in xs {
        hasher32.write(&x);
    }
    let hash_res = hasher32.finish32();
    let c_res = hash32(&all_bytes, seed);
    func_res == hash_res && hash_res == c_res
}

#[quickcheck]
fn random_check_128_chunks(xs: Vec<Vec<u8>>, seed: u32) -> bool {
    let mut all_bytes = vec![];
    for c in &xs {
        all_bytes.extend_from_slice(c);
    }
    let func_res = murmurhash3_x64_128(&all_bytes, seed);
    let mut hasher = Hasher128::with_seed(seed);
    for x in xs {
        hasher.write(&x);
    }
    let hash_res = hasher.finish128();
    let c_res = hash128_64(&all_bytes, seed);
    func_res == hash_res && hash_res == c_res
}
