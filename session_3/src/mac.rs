#[cfg(test)]
mod tests {
    use aes::Aes256;
    use cbc_mac::{CbcMac, Mac};
    use hex_literal::hex;

    type Aes256Cbc = CbcMac<Aes256>;

    #[test]
    fn cbc_mac_works() {
        let key = hex!(
            "80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
             00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01"
        );
        let data = hex!(
            "4D 41 43 73 20 61 72 65 20 76 65 72 79 20 75 73 
             65 66 75 6C 20 69 6E 20 63 72 79 70 74 6F 67 72 
             61 70 68 79 21 20 20 20 20 20 20 20 20 20 20 20"
        );

        let mut mac = Aes256Cbc::new_from_slice(&key).unwrap();

        mac.update(&data);

        let expected = hex!("0D 82 0E 3A 1E 10 5D 30 72 16 FC 00 C7 A5 B4 49");
        let result = mac.finalize().into_bytes().to_vec();

        assert_eq!(result, expected);
    }
}
