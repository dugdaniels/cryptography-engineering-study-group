use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
use sha2::Digest;

pub fn rsa_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    let data = black_box(b"hello world");

    c.bench_function("rsa encode", |b| {
        b.iter(|| {
            pub_key
                .encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), &data[..])
                .expect("failed to encrypt")
        })
    });
}

pub fn rsa_oaep_pss_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let (public_key, _private_key) =
        rsa_oaep_pss::generate_rsa_keys(&mut rng, 2048).expect("keys generation error");

    let mut oaep = rsa_oaep_pss::RsaOaep::new(rand::rngs::OsRng, &sha2::Sha256::new());

    let data = black_box(b"hello world");

    c.bench_function("rsa_oaep_pss encode", |b| {
        b.iter(|| oaep.encrypt(&public_key, data).expect("encryption error"))
    });
}

criterion_group!(benches, rsa_benchmark, rsa_oaep_pss_benchmark);
criterion_main!(benches);
