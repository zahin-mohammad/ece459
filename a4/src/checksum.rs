use sha2::{Digest, Sha256};
use std::fmt;

#[derive(Default)]
pub struct Checksum(Vec<u8>);

impl Checksum {
    // Initialize the checksum with the SHA256 hash of the input string
    pub fn with_sha256(sha: &str) -> Self {
        let digest = Sha256::digest(sha.as_bytes());
        Self(digest.to_vec())
    }

    // XOR the two checksums
    pub fn update(&mut self, rhs: Self) {
        if self.0.is_empty() {
            *self = rhs;
        } else if rhs.0.is_empty() {
        } else {
            let b: Vec<u8> = rhs.0;
            assert_eq!(self.0.len(), b.len());
            for (i, b_char) in b.iter().enumerate() {
                self.0[i] ^= b_char
            }
        };
    }
}

impl fmt::Display for Checksum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0.clone()))
    }
}
