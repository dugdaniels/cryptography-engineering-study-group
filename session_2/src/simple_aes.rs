use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes256;

pub struct Aes {
    cipher: Aes256,
}

impl Aes {
    pub fn new(key: &[u8]) -> Self {
        let cipher = Aes256::new(GenericArray::from_slice(key));
        Self { cipher }
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut block = *GenericArray::from_slice(data);
        self.cipher.encrypt_block(&mut block);

        block.to_vec()
    }

    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut block = GenericArray::clone_from_slice(data);
        self.cipher.decrypt_block(&mut block);

        block.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn aes_works() {
        let data = hex!("539B 333B 3970 6D14 9028 CFE1 D9D4 A407");
        let key = hex!(
            "8000 0000 0000 0000 0000 0000 0000 0000
             0000 0000 0000 0000 0000 0000 0000 0001"
        );
        let cipher = Aes::new(&key);

        let encrypted = cipher.encrypt(&data);
        let decrypted = cipher.decrypt(&encrypted);

        assert_eq!(decrypted, data);
    }
}
