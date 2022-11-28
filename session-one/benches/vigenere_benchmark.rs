use criterion::{black_box, criterion_group, criterion_main, Criterion};
use session_one::vigenere;

pub fn encode_benchmark(c: &mut Criterion) {
    let text = black_box("attackatdawn");
    let key = black_box("LEMON");

    let vigenere = vigenere::Vigenere::new(key);

    c.bench_function("vigenere encode", |b| {
        b.iter(|| {
            vigenere.encode(text);
        })
    });
}

criterion_group!(benches, encode_benchmark);
criterion_main!(benches);
