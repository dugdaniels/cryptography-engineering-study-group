pub fn pad(data: &[u8], block_size: usize) -> Vec<u8> {
    let mut padded = data.to_vec();
    let padding = match block_size - (data.len() % block_size) {
        0 => block_size,
        n => n,
    };
    padded.extend(vec![padding as u8; padding]);

    padded
}

pub fn has_valid_padding(data: &[u8], block_size: usize) -> bool {
    let padding = data[data.len() - 1] as usize;

    if padding > block_size {
        return false;
    }

    let padding_slice = &data[data.len() - padding..];
    padding_slice.iter().all(|&x| x == padding as u8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn pad_works() {
        let data = hex!(
            "7C3D 26F7 7377 635A 5E43 E9B5 CC5D 0592
             6E26 FFC5 220D C7D4 05F1 7086 70E6 E0"
        );

        let expected = hex!(
            "7C3D 26F7 7377 635A 5E43 E9B5 CC5D 0592
             6E26 FFC5 220D C7D4 05F1 7086 70E6 E001"
        );

        let padded = pad(&data, 16);
        assert_eq!(padded, expected);
    }

    #[test]
    fn has_valid_padding_catches_incorrect_padding() {
        let data = hex!(
            "7C3D 26F7 7377 635A 5E43 E9B5 CC5D 0592
             6E26 FFC5 220D C7D4 05F1 7086 70E6 E017"
        );

        assert!(!has_valid_padding(&data, 16));
    }

    #[test]
    fn has_valid_padding_verifies_correct_padding() {
        let data = hex!(
            "7C3D 26F7 7377 635A 5E43 E9B5 CC5D 0592
             6E26 FFC5 220D C7D4 05F1 7086 70E6 E001"
        );

        assert!(has_valid_padding(&data, 16));
    }
}
