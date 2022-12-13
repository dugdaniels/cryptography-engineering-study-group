use sha2::{Digest, Sha512};

#[derive(Copy, Clone)]
pub enum NSize {
    N8 = 8,
    N16 = 16,
    N24 = 24,
    N32 = 32,
    N40 = 40,
    N48 = 48,
}

pub struct Sha512n {
    n: usize,
}

impl Sha512n {
    pub fn new(n: NSize) -> Self {
        Self { n: n as usize }
    }

    pub fn digest(&self, data: &[u8]) -> Vec<u8> {
        let mut hash = Sha512::digest(data).to_vec();
        hash.truncate(self.n / 8);

        hash
    }
}

pub fn find_collision(n: NSize) -> Option<usize> {
    let mut cache: Vec<Vec<u8>> = Vec::with_capacity(2_usize.pow(n as u32));
    let sha512n = Sha512n::new(n);

    for i in 0..cache.capacity() {
        let hash = sha512n.digest(&i.to_be_bytes());
        if cache.contains(&hash) {
            return Some(i);
        } else {
            cache.push(hash);
        }
    }

    None
}

pub fn find_preimage(n: NSize, target_hash: &[u8]) -> Option<usize> {
    let sha512n = Sha512n::new(n);

    for i in 0..2_usize.pow(n as u32) {
        let hash = sha512n.digest(&i.to_be_bytes());
        if hash == target_hash {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn sha512n_works() {
        let n = NSize::N8;

        let data = b"hello world";
        let expected = hex!(
            "309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f
            989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f"
        );

        let hasher = Sha512n::new(n);
        let encrypted = hasher.digest(data);

        assert_eq!(encrypted, expected[..n as usize / 8]);
    }

    #[test]
    fn find_collision_works() {
        let n = NSize::N16;
        let attempts = find_collision(n);

        assert_eq!(attempts, Some(344))
    }

    #[test]
    fn find_preimage_works() {
        let n = NSize::N16;
        let target_hash = hex!("3D 4B");
        let attempts = find_preimage(n, &target_hash);

        assert_eq!(attempts, Some(38804))
    }
}
