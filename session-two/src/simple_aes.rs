use aes::cipher::generic_array::typenum::U32;
use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes256;

pub struct Aes {
    cipher: Aes256,
}

impl Aes {
    pub fn new(key: &[u8]) -> Self {
        let cipher = Aes256::new(GenericArray::<u8, U32>::from_slice(key));
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

    #[test]
    fn aes_works() {
        let data = hex::decode("539B333B39706D149028CFE1D9D4A407").unwrap();
        let key = hex::decode("8000000000000000000000000000000000000000000000000000000000000001")
            .unwrap();
        let cipher = Aes::new(&key);

        let encrypted = cipher.encrypt(&data);
        let decrypted = cipher.decrypt(&encrypted);

        assert_eq!(decrypted, data);
    }
}
