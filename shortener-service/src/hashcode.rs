use sha2::{Digest, Sha256};
use std::collections::HashSet;

/// Character set for Base62 encoding
const BASE62_CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// Converts a decimal number to a Base62 string
fn base62_encode(mut num: u64) -> String {
    let mut encoded = String::new();
    while num > 0 {
        encoded.push(BASE62_CHARSET[(num % 62) as usize] as char);
        num /= 62;
    }
    encoded.chars().rev().collect() // Reverse to maintain correct order
}

/// Generates a unique short code from a long URL using SHA-256 and Base62 encoding
pub fn generate_short_code(long_url: &str, existing_short_codes: &HashSet<String>) -> String {
    let hash = Sha256::digest(long_url.as_bytes());

    let hash_bytes = &hash[0..6];
    let num = u64::from_be_bytes([
        0, 0, 0, 0, hash_bytes[0], hash_bytes[1], hash_bytes[2], hash_bytes[3]
    ]) + u64::from_be_bytes([0, 0, 0, 0, hash_bytes[4], hash_bytes[5], 0, 0]);

    let mut short_code = base62_encode(num);

    let mut counter = 1;
    while existing_short_codes.contains(&short_code) {
        let suffix = base62_encode(counter as u64);
        short_code = format!("{}{}", short_code, suffix);
        counter += 1;
    }

    short_code
}
