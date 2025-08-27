use hex;
use base64::{engine::general_purpose, Engine};

/// Converts a hexadecimal string to Base64 string using the `base64` crate.
pub fn hex_to_base64_lib_func(hex_string: &str) -> String {
    let bytes_data = hex::decode(hex_string).expect("Invalid hex string");
    let base64_string: String = general_purpose::STANDARD.encode(&bytes_data);
    base64_string
}

