pub mod aes_in_cbc;
pub mod des_complementation;
pub mod pkcs;
pub mod simple_aes;
pub mod simple_rsa;

#[cfg(test)]
mod tests {
    use crate::simple_aes::Aes;
    use crate::simple_rsa::Rsa;

    #[test]
    fn double_encrypt_works() {
        let data = hex::decode("296C93FDF499AAEB4194BABC2E63561D").unwrap();

        let key = hex::decode("8000000000000000000000000000000000000000000000000000000000000001")
            .unwrap();
        let aes = Aes::new(&key);

        let rng = rand::thread_rng();
        let rsa = Rsa::new();

        let encrypted = rsa.encrypt(&aes.encrypt(&data), rng);
        let decrypted = aes.decrypt(&rsa.decrypt(&encrypted));

        assert_eq!(decrypted, data);
    }
}
