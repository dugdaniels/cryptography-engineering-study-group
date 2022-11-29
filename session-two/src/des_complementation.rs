use des::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use des::Des;

pub fn complement(data: &[u8]) -> Vec<u8> {
    data.iter().map(|x| !x).collect()
}

pub struct SimpleDes {
    pub cipher: Des,
}

impl SimpleDes {
    pub fn new(key: &[u8]) -> Self {
        let cipher = Des::new(GenericArray::from_slice(key));
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
    fn complement_works() {
        let data = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let complement = complement(&data);

        assert_eq!(
            complement,
            vec![0xFF, 0xFE, 0xFD, 0xFC, 0xFB, 0xFA, 0xF9, 0xF8]
        );
    }

    #[test]
    fn des_works() {
        let data = hex::decode("1fc8e5e7a322e4ad").unwrap();
        let key = hex::decode("8000000000000001").unwrap();
        let cipher = SimpleDes::new(&key);

        let encrypted = cipher.encrypt(&data);
        let decrypted = cipher.decrypt(&encrypted);

        assert_eq!(decrypted, data);
    }

    #[test]
    fn des_complementation_property() {
        let data = hex::decode("1fc8e5e7a322e4ad").unwrap();
        let data_complement = complement(&data);

        let key = hex::decode("8000000000000001").unwrap();
        let key_complement = complement(&key);

        let lhs_cipher = SimpleDes::new(&key_complement);
        let lhs_encrypted = lhs_cipher.encrypt(&data_complement);

        let rhs_cipher = SimpleDes::new(&key);
        let rhs_encrypted = complement(&rhs_cipher.encrypt(&data));

        assert_eq!(lhs_encrypted, rhs_encrypted);
    }
}
