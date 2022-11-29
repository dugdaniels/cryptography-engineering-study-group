use rand::{CryptoRng, RngCore};
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};

pub struct Rsa {
    pub private_key: RsaPrivateKey,
    pub public_key: RsaPublicKey,
}

impl Rsa {
    pub fn new() -> Self {
        let private_key = RsaPrivateKey::new(&mut rand::thread_rng(), 3072).unwrap();
        let public_key = RsaPublicKey::from(&private_key);
        Self {
            private_key,
            public_key,
        }
    }

    pub fn encrypt<T: CryptoRng + RngCore>(&self, data: &[u8], mut rng: T) -> Vec<u8> {
        self.public_key
            .encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), data)
            .expect("Failed to encrypt data")
    }

    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        self.private_key
            .decrypt(PaddingScheme::new_pkcs1v15_encrypt(), data)
            .expect("Failed to decrypt data")
    }
}

impl Default for Rsa {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rsa_works() {
        let cipher = Rsa::new();
        let data = b"Hello, world!";
        let rng = rand::thread_rng();

        let encrypted = cipher.encrypt(data, rng);
        let decrypted = cipher.decrypt(&encrypted);

        assert_eq!(decrypted, data);
    }
}
