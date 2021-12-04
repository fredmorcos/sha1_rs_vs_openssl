use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::distributions::{DistIter, Uniform};
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};

fn rust_sha_1(prefix: &[u8], suffix: &[u8]) {
    use sha_1::{Digest, Sha1};

    let mut hasher = Sha1::new();
    hasher.update(prefix);
    hasher.update(suffix);
    let _result = hasher.finalize();
}

fn openssl_sha1(prefix: &[u8], suffix: &[u8]) {
    use openssl::sha::Sha1;

    let mut hasher = Sha1::new();
    hasher.update(prefix);
    hasher.update(suffix);
    let _result = hasher.finish();
}

fn rust_sha1(prefix: &[u8], suffix: &[u8]) {
    use sha1::Sha1;

    let mut hasher = Sha1::new();
    hasher.update(prefix);
    hasher.update(suffix);
    let _result = hasher.digest().bytes();
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
    const LEN: usize = 8;

    let prefix = "kHtMDdVrTKHhUaNusVyBaJybfNMWjfxnaIiAYqgfmCTkNKFvYGloeHDHdsksfFla".as_bytes();
    let mut rng = thread_rng().sample_iter(Uniform::from(1..255));

    let mut group = c.benchmark_group("SHA1");
    group.warm_up_time(Duration::from_secs(10));

    for i in 0..2 {
        let mut suffix = Vec::with_capacity(LEN);
        random_string::<LEN>(&mut suffix, &mut rng);

        group.bench_with_input(BenchmarkId::new("sha-1 crate", i), &(), |b, _| {
            b.iter(|| rust_sha_1(black_box(prefix), black_box(&suffix)))
        });

        group.bench_with_input(BenchmarkId::new("sha1 crate", i), &(), |b, _| {
            b.iter(|| rust_sha1(black_box(prefix), black_box(&suffix)))
        });

        group.bench_with_input(BenchmarkId::new("openssl sha1", i), &(), |b, _| {
            b.iter(|| openssl_sha1(black_box(prefix), black_box(&suffix)))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
