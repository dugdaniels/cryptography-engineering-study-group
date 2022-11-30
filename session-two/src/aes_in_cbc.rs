use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockDecryptMut, KeyIvInit};

type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;
pub struct AesCbc {
    cipher: Aes256CbcDec,
}

impl AesCbc {
    pub fn new(key: &[u8], iv: &[u8]) -> Self {
        let cipher = Aes256CbcDec::new(key.into(), iv.into());
        Self { cipher }
    }

    pub fn decrypt(&mut self, data: &[u8]) -> Vec<u8> {
        data.chunks(16)
            .flat_map(|chunk| {
                let mut block = GenericArray::clone_from_slice(chunk);
                self.cipher.decrypt_block_mut(&mut block);
                block
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn aes_in_cbc_works() {
        let data = hex!(
            "7C3D 26F7 7377 635A 5E43 E9B5 CC5D 0592
             6E26 FFC5 220D C7D4 05F1 7086 70E6 E017"
        );

        let key = hex!(
            "8000 0000 0000 0000 0000 0000 0000 0000
             0000 0000 0000 0000 0000 0000 0000 0001"
        );

        let iv = hex!("87F3 48FF 79B8 11AF 3857 D671 8E5F 0F91");

        let mut cipher = AesCbc::new(&key, &iv);
        let decrypted = cipher.decrypt(&data);
        let plaintext = String::from_utf8_lossy(decrypted.as_slice());

        assert_eq!(plaintext, "Another secret!  And another.   ");
    }
}
