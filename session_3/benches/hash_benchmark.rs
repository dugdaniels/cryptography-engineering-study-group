use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sha2::{Digest, Sha256};
use sha3::Sha3_256;

const INPUT: [u8; 11] = *b"hello world";

fn sha2_256_hash(c: &mut Criterion) {
    c.bench_function("sha2", |b| {
        b.iter(|| {
            Sha256::digest(black_box(&INPUT));
        })
    });
}

fn sha3_256_hash(c: &mut Criterion) {
    c.bench_function("sha3", |b| {
        b.iter(|| {
            Sha3_256::digest(black_box(&INPUT));
        })
    });
}

fn blake3_hash(c: &mut Criterion) {
    c.bench_function("blake3", |b| {
        b.iter(|| {
            blake3::hash(black_box(&INPUT));
        })
    });
}

criterion_group!(benches, sha2_256_hash, sha3_256_hash, blake3_hash);
criterion_main!(benches);
