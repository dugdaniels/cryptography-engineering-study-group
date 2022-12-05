use criterion::{black_box, criterion_group, criterion_main, Criterion};
use session_three::sha512n;
use session_three::sha512n::NSize;

fn n_8_collision(c: &mut Criterion) {
    c.bench_function("n_8", |b| {
        b.iter(|| sha512n::find_collision(black_box(NSize::N8)))
    });
}

fn n_16_collision(c: &mut Criterion) {
    c.bench_function("n_16", |b| {
        b.iter(|| sha512n::find_collision(black_box(NSize::N16)))
    });
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().sample_size(10);
    targets = n_8_collision, n_16_collision
}
criterion_main!(benches);
