pub fn encode(text: &str, key: &str) -> String {
    let text = text
        .to_uppercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>();

    let mut key_iter = key.chars().cycle();
    let mut result = String::new();

    for c in text.chars() {
        let key_char = key_iter.next().unwrap();

        let key_index = key_char as u8 - b'A';
        let c_index = c as u8 - b'A';
        let new_index = (c_index + key_index) % 26;

        result.push((new_index + b'A') as char);
    }
    result
}

pub fn decode(cipher: &str, key: &str) -> String {
    let mut key_iter = key.chars().cycle();
    let mut result = String::new();

    for c in cipher.chars() {
        let key_char = key_iter.next().unwrap();

        let key_index = key_char as u8 - b'A';
        let c_index = c as u8 - b'A';
        let new_index = (c_index + 26 - key_index) % 26;

        result.push((new_index + b'A') as char);
    }
    result
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
