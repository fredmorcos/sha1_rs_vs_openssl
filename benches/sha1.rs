use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::distributions::{DistIter, Uniform};
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use sha_1::digest::consts::U20;
use sha_1::digest::generic_array::GenericArray;

fn rust_sha_1(prefix: &[u8], suffix: &[u8], result: &mut GenericArray<u8, U20>) {
    use sha_1::digest::FixedOutput;
    use sha_1::{Digest, Sha1};

    let mut hasher = Sha1::new();
    hasher.update(prefix);
    hasher.update(suffix);
    hasher.finalize_into(result)
}

fn openssl_sha1(prefix: &[u8], suffix: &[u8]) -> [u8; 20] {
    use openssl::sha::Sha1;

    let mut hasher = Sha1::new();
    hasher.update(prefix);
    hasher.update(suffix);
    hasher.finish()
}

fn rust_sha1(prefix: &[u8], suffix: &[u8]) -> [u8; 20] {
    use sha1::Sha1;

    let mut hasher = Sha1::new();
    hasher.update(prefix);
    hasher.update(suffix);
    hasher.digest().bytes()
}

type BytesRng = DistIter<Uniform<u8>, ThreadRng, u8>;

fn random_string<const LEN: usize>(s: &mut Vec<u8>, rng: &mut BytesRng) {
    fn pred(&c: &u8) -> bool {
        c != b'\r' && c != b'\t' && c != b'\n' && c != b' '
    }

    s.clear();
    s.extend(rng.filter(pred).take(LEN));
}

fn criterion_benchmark(c: &mut Criterion) {
    let prefix = "kHtMDdVrTKHhUaNusVyBaJybfNMWjfxnaIiAYqgfmCTkNKFvYGloeHDHdsksfFla".as_bytes();
    let mut rng = thread_rng().sample_iter(Uniform::from(1..255));

    let mut group = c.benchmark_group("SHA1");
    group.warm_up_time(Duration::from_secs(5));

    let mut suffix = Vec::with_capacity(8);
    random_string::<8>(&mut suffix, &mut rng);

    let mut hash = Default::default();

    rust_sha_1(prefix, &suffix, &mut hash);
    assert_eq!(rust_sha1(prefix, &suffix), hash.as_slice());
    assert_eq!(openssl_sha1(prefix, &suffix), hash.as_slice());

    group.bench_with_input(BenchmarkId::new("sha-1 crate", "short"), &(), |b, _| {
        b.iter(|| rust_sha_1(black_box(prefix), black_box(&suffix), black_box(&mut hash)))
    });

    group.bench_with_input(BenchmarkId::new("sha1 crate", "short"), &(), |b, _| {
        b.iter(|| black_box(rust_sha1(black_box(prefix), black_box(&suffix))))
    });

    group.bench_with_input(BenchmarkId::new("openssl sha1", "short"), &(), |b, _| {
        b.iter(|| black_box(openssl_sha1(black_box(prefix), black_box(&suffix))))
    });

    let mut suffix = Vec::with_capacity(64);
    random_string::<64>(&mut suffix, &mut rng);

    rust_sha_1(prefix, &suffix, &mut hash);
    assert_eq!(rust_sha1(prefix, &suffix), hash.as_slice());
    assert_eq!(openssl_sha1(prefix, &suffix), hash.as_slice());

    group.bench_with_input(BenchmarkId::new("sha-1 crate", "long"), &(), |b, _| {
        b.iter(|| rust_sha_1(black_box(prefix), black_box(&suffix), black_box(&mut hash)))
    });

    group.bench_with_input(BenchmarkId::new("sha1 crate", "long"), &(), |b, _| {
        b.iter(|| black_box(rust_sha1(black_box(prefix), black_box(&suffix))))
    });

    group.bench_with_input(BenchmarkId::new("openssl sha1", "long"), &(), |b, _| {
        b.iter(|| black_box(openssl_sha1(black_box(prefix), black_box(&suffix))))
    });

    let mut suffix = Vec::with_capacity(512);
    random_string::<512>(&mut suffix, &mut rng);

    rust_sha_1(prefix, &suffix, &mut hash);
    assert_eq!(rust_sha1(prefix, &suffix), hash.as_slice());
    assert_eq!(openssl_sha1(prefix, &suffix), hash.as_slice());

    group.bench_with_input(BenchmarkId::new("sha-1 crate", "huge"), &(), |b, _| {
        b.iter(|| rust_sha_1(black_box(prefix), black_box(&suffix), black_box(&mut hash)))
    });

    group.bench_with_input(BenchmarkId::new("sha1 crate", "huge"), &(), |b, _| {
        b.iter(|| black_box(rust_sha1(black_box(prefix), black_box(&suffix))))
    });

    group.bench_with_input(BenchmarkId::new("openssl sha1", "huge"), &(), |b, _| {
        b.iter(|| black_box(openssl_sha1(black_box(prefix), black_box(&suffix))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
