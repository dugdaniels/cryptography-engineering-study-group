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
    use hex_literal::hex;

    #[test]
    fn complement_works() {
        let data = hex!("0001 0203 0405 0607");
        let complement = complement(&data);

        assert_eq!(complement, hex!("fffe fdfc fbfa f9f8"));
    }

    #[test]
    fn des_works() {
        let data = hex!("1fc8e5e7a322e4ad");
        let key = hex!("8000000000000001");
        let cipher = SimpleDes::new(&key);

        let encrypted = cipher.encrypt(&data);
        let decrypted = cipher.decrypt(&encrypted);

        assert_eq!(decrypted, data);
    }

    #[test]
    fn des_complementation_works() {
        let data = hex!("1fc8e5e7a322e4ad");
        let data_complement = complement(&data);

        let key = hex!("8000000000000001");
        let key_complement = complement(&key);

        let lhs_cipher = SimpleDes::new(&key_complement);
        let lhs_encrypted = lhs_cipher.encrypt(&data_complement);

        let rhs_cipher = SimpleDes::new(&key);
        let rhs_encrypted = complement(&rhs_cipher.encrypt(&data));

        assert_eq!(lhs_encrypted, rhs_encrypted);
    }
}
