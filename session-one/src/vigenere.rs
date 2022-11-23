pub struct Vigenere {
    key: String,
}

impl Vigenere {
    pub fn new(key: &str) -> Self {
        Self {
            key: key
                .to_uppercase()
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>(),
        }
    }

    fn parse_input(text: &str) -> String {
        text.to_uppercase()
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect()
    }

    pub fn encode(&self, text: &str) -> String {
        Self::parse_input(text)
            .chars()
            .zip(self.key.chars().cycle())
            .map(|(text_char, key_char)| {
                let key_index = key_char as u8 - b'A';
                let text_index = text_char as u8 - b'A';
                let new_index = (text_index + key_index) % 26;

                (new_index + b'A') as char
            })
            .collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        Self::parse_input(cipher)
            .chars()
            .zip(self.key.chars().cycle())
            .map(|(cipher_char, key_char)| {
                let key_index = key_char as u8 - b'A';
                let cipher_index = cipher_char as u8 - b'A';
                let new_index = (cipher_index + 26 - key_index) % 26;

                (new_index + b'A') as char
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::vigenere::Vigenere;

    #[test]
    fn encode_works() {
        let vigenere = Vigenere::new("lemon");
        let text = "attackatdawn";

        let encrypted = vigenere.encode(text);
        assert_eq!(encrypted, "LXFOPVEFRNHR");
    }

    #[test]
    fn decode_works() {
        let vigenere = Vigenere::new("lemon");
        let text = "lxfopvefrnhr";

        let decrypted = vigenere.decode(text);
        assert_eq!(decrypted, "ATTACKATDAWN");
    }
}
