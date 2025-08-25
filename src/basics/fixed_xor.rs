///Function for XOR comparisont of two sets of bytes, `a ^ b`
pub fn xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| x ^ y)
        .collect()
}

/// Converts a byte slice to a lowercase hexadecimal string.
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";

    let mut hex_string = String::with_capacity(bytes.len() * 2);

    for &byte in bytes {
        let hi = (byte >> 4) as usize;
        let lo = (byte & 0x0F) as usize;
        hex_string.push(HEX_DIGITS[hi] as char);
        hex_string.push(HEX_DIGITS[lo] as char);
    }

    hex_string
}
