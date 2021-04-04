use criterion::*;

use rand::*;
use std::hash::Hasher;

use mur3::*;
use mur3_c::*;

fn bench_murmur3_32(b: &mut Criterion, size: usize) {
    let mut group = b.benchmark_group("Murmur3_x86_32");
    let mut buf = vec![0; size];
    rand::thread_rng().fill_bytes(buf.as_mut_slice());

    group.throughput(Throughput::Bytes(size as u64));
    group.bench_with_input(BenchmarkId::new("rust-func", size), &buf, |b, i| {
        b.iter(|| {
            let res = murmurhash3_x86_32(i, 0);
            black_box(res);
        })
    });
    group.bench_with_input(BenchmarkId::new("c-func", size), &buf, |b, i| {
        b.iter(|| {
            let res = hash32(i, 0);
            black_box(res);
        })
    });
    group.bench_with_input(BenchmarkId::new("hasher", size), &buf, |b, i| {
        b.iter(|| {
            let mut hasher = Hasher32::with_seed(0);
            hasher.write(i);
            let res = hasher.finish32();
            black_box(res);
        })
    });

    group.finish();
}

fn bench_murmur3_128(b: &mut Criterion, size: usize) {
    let mut group = b.benchmark_group("Murmur3_x64_128");
    let mut buf = vec![0; size];
    rand::thread_rng().fill_bytes(buf.as_mut_slice());

    group.throughput(Throughput::Bytes(size as u64));
    group.bench_with_input(BenchmarkId::new("rust-func", size), &buf, |b, i| {
        b.iter(|| {
            let res = murmurhash3_x64_128(i, 0);
            black_box(res);
        })
    });
    group.bench_with_input(BenchmarkId::new("c-func", size), &buf, |b, i| {
        b.iter(|| {
            let res = hash128_64(i, 0);
            black_box(res);
        })
    });
    group.bench_with_input(BenchmarkId::new("hasher", size), &buf, |b, i| {
        b.iter(|| {
            let mut hasher = Hasher128::with_seed(0);
            hasher.write(i);
            let res = hasher.finish128();
            black_box(res);
        })
    });

    group.finish();
}

fn bench_murmur3(b: &mut Criterion) {
    for size in 0..=4 {
        bench_murmur3_32(b, size);
    }

    for size in 0..=16 {
        bench_murmur3_128(b, size);
    }

    for p in 5..=13 {
        let size = 2usize.pow(p);
        bench_murmur3_32(b, size);
        bench_murmur3_128(b, size);
    }
}

criterion_group!(benches, bench_murmur3);
criterion_main!(benches);
