use hex;
use base64::{engine::general_purpose, Engine};

/// Converts a hexadecimal string to Base64 string using the `base64` crate.
pub fn hex_to_base64(hex_string: &str) -> String {
    let bytes_data = hex::decode(hex_string).expect("Invalid hex string");
    let base64_string = general_purpose::STANDARD.encode(&bytes_data);
    base64_string
}

/// Converts a hexadecimal string to bytes.
pub fn hex_to_bytes(hex_string: &str) -> Vec<u8> {
    hex_string.as_bytes()
        .chunks(2)
        .map(|pair| {
            let hi = (pair[0] as char).to_digit(16).unwrap();
            let lo = (pair[1] as char).to_digit(16).unwrap();
            ((hi << 4) | lo) as u8
        })
        .collect()
}

/// Converts bytes to Base64 string manually without external crates.
pub fn bytes_to_base64(bytes: &[u8]) -> String {
    const TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    let mut i = 0;

    while i < bytes.len() {
        let b0 = bytes[i];
        let b1 = if i + 1 < bytes.len() { bytes[i + 1] } else { 0 };
        let b2 = if i + 2 < bytes.len() { bytes[i + 2] } else { 0 };

        out.push(TABLE[(b0 >> 2) as usize] as char);
        out.push(TABLE[(((b0 & 0b11) << 4) | (b1 >> 4)) as usize] as char);
        out.push(if i + 1 < bytes.len() {
            TABLE[(((b1 & 0b1111) << 2) | (b2 >> 6)) as usize] as char
        } else {
            '='
        });
        out.push(if i + 2 < bytes.len() {
            TABLE[(b2 & 0b111111) as usize] as char
        } else {
            '='
        });

        i += 3;
    }

    out
}
