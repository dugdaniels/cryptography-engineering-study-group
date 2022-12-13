pub mod aes_in_cbc;
pub mod des_complementation;
pub mod pkcs;
pub mod simple_aes;
pub mod simple_rsa;

#[cfg(test)]
mod tests {
    use crate::simple_aes::Aes;
    use crate::simple_rsa::Rsa;
    use hex_literal::hex;

    #[test]
    fn double_encrypt_works() {
        let data = hex!("296C 93FD F499 AAEB 4194 BABC 2E63 561D");

        let key = hex!(
            "8000 0000 0000 0000 0000 0000 0000 0000
             0000 0000 0000 0000 0000 0000 0000 0001"
        );
        let aes = Aes::new(&key);

        let rng = rand::thread_rng();
        let rsa = Rsa::new();

        let encrypted = rsa.encrypt(&aes.encrypt(&data), rng);
        let decrypted = aes.decrypt(&rsa.decrypt(&encrypted));

        assert_eq!(decrypted, data);
    }
}
