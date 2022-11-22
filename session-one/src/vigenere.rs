pub fn encode(text: &str, key: &str) -> String {
    let text = text
        .to_uppercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>();

    text.chars()
        .zip(key.chars().cycle())
        .map(|(text_char, key_char)| {
            let key_index = key_char as u8 - b'A';
            let text_index = text_char as u8 - b'A';
            let new_index = (text_index + key_index) % 26;

            (new_index + b'A') as char
        })
        .collect()
}

pub fn decode(cipher: &str, key: &str) -> String {
    cipher
        .chars()
        .zip(key.chars().cycle())
        .map(|(cipher_char, key_char)| {
            let key_index = key_char as u8 - b'A';
            let cipher_index = cipher_char as u8 - b'A';
            let new_index = (cipher_index + 26 - key_index) % 26;

            (new_index + b'A') as char
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::vigenere::{decode, encode};

    #[test]
    fn encode_works() {
        let text = "attackatdawn";
        let key = "LEMON";
        let encrypted = encode(text, key);
        assert_eq!(encrypted, "LXFOPVEFRNHR");
    }

    #[test]
    fn decode_works() {
        let text = "LXFOPVEFRNHR";
        let key = "LEMON";
        let decrypted = decode(text, key);
        assert_eq!(decrypted, "ATTACKATDAWN");
    }
}
