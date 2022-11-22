use criterion::{black_box, criterion_group, criterion_main, Criterion};
use session_one::vigenere::encode;

pub fn encode_benchmark(c: &mut Criterion) {
    let text = black_box("attackatdawn");
    let key = black_box("LEMON");

    c.bench_function("vigenere encode", |b| {
        b.iter(|| {
            encode(text, key);
        })
    });
}

criterion_group!(benches, encode_benchmark);
criterion_main!(benches);
