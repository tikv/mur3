#![feature(test)]

extern crate test;

use std::hash::Hasher;
use test::Bencher;

use mur3::*;

#[bench]
fn bench_32(b: &mut Bencher) {
    let string: &[u8] =
        test::black_box(b"Lorem ipsum dolor sit amet, consectetur adipisicing elit");
    b.bytes = string.len() as u64;
    b.iter(|| murmurhash3_x86_32(string, 0));
}

#[bench]
fn bench_x64_128(b: &mut Bencher) {
    let string: &[u8] =
        test::black_box(b"Lorem ipsum dolor sit amet, consectetur adipisicing elit");
    b.bytes = string.len() as u64;
    b.iter(|| murmurhash3_x64_128(string, 0));
}

#[bench]
fn bench_32_hasher(b: &mut Bencher) {
    let string: &[u8] =
        test::black_box(b"Lorem ipsum dolor sit amet, consectetur adipisicing elit");
    b.bytes = string.len() as u64;
    b.iter(|| {
        let mut hasher = Hasher32::with_seed(0);
        hasher.write(string);
        let h = hasher.finish32();
        test::black_box(h);
    });
}

#[bench]
fn bench_x64_128_hasher(b: &mut Bencher) {
    let string: &[u8] =
        test::black_box(b"Lorem ipsum dolor sit amet, consectetur adipisicing elit");
    b.bytes = string.len() as u64;
    b.iter(|| {
        let mut hasher = Hasher128::with_seed(0);
        hasher.write(string);
        let h = hasher.finish128();
        test::black_box(h);
    });
}
