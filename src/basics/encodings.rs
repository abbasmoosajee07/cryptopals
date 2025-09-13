/// Converts a hexadecimal string to bytes.
pub fn hex_to_bytes(hex_string: &str) -> Result<Vec<u8>, String> {
    if hex_string.len() % 2 != 0 {
        return Err("Hex string must have even length".to_string());
    }

    let mut result: Vec<u8> = Vec::new();
    for pair in hex_string.as_bytes().chunks(2) {
        let hi = (pair[0] as char)
            .to_digit(16)
            .ok_or_else(|| format!("Invalid hex character '{}'", pair[0] as char))?;
        let lo = (pair[1] as char)
            .to_digit(16)
            .ok_or_else(|| format!("Invalid hex character '{}'", pair[1] as char))?;
        result.push(((hi << 4) | lo) as u8);
    }
    Ok(result)
}

/// Converts a byte slice to a lowercase hexadecimal string.
pub fn bytes_to_hex(bytes: &[u8]) -> Result<String, String> {
    const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";

    if bytes.is_empty() {
        return Err("Empty byte slice".to_string());
    }

    let mut hex_string = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        let hi = (byte >> 4) as usize;
        let lo = (byte & 0x0F) as usize;
        hex_string.push(HEX_DIGITS[hi] as char);
        hex_string.push(HEX_DIGITS[lo] as char);
    }

    Ok(hex_string)
}

/// Decode a Base64 string into a Vec<u8>
pub fn base64_to_bytes(line: &str) -> Result<Vec<u8>, String> {
    const BASE64_TABLE: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = Vec::new();
    let mut buffer = 0u32;
    let mut bits_collected = 0;

    for c in line.chars() {
        if c.is_whitespace() {
            continue; // skip whitespace  ← ADD THIS LINE
        }
        if c == '=' {
            break; // padding reached
        }

        let value = match BASE64_TABLE.iter().position(|&x| x == c as u8) {
            Some(v) => v as u32,
            None => return Err(format!("Invalid base64 character '{}'", c)),
        };

        buffer = (buffer << 6) | value;
        bits_collected += 6;

        if bits_collected >= 8 {
            bits_collected -= 8;
            let byte = (buffer >> bits_collected) as u8 & 0xFF;
            result.push(byte);
        }
    }

    Ok(result)
}

/// Converts bytes to Base64 string manually without external crates.
pub fn bytes_to_base64(bytes: &[u8]) -> Result<String, String> {
    if bytes.is_empty() {
        return Err("Empty byte slice".to_string());
    }

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

    Ok(out)
}

/// Convert bytes to bits
pub fn bytes_to_bits(input: &[u8]) -> Result<Vec<String>, String> {
    if input.is_empty() {
        return Err("Empty byte slice".to_string());
    }

    Ok(input
        .iter()
        .map(|byte| format!("{:08b}", byte))
        .collect())
}

/// Convert a vector of 8-bit binary strings (e.g. "01101001") back into bytes
pub fn bits_to_bytes(bits: &[String]) -> Result<Vec<u8>, String> {
    if bits.is_empty() {
        return Err("Empty bits vector".to_string());
    }

    let mut result = Vec::new();
    for b in bits {
        let parsed = u8::from_str_radix(b, 2)
            .map_err(|_| format!("Invalid binary string '{}'", b))?;
        result.push(parsed);
    }
    Ok(result)
}

/// Print bytes as Rust-style hex literals (0x..)
pub fn hexify(bytes: &[u8]) -> Result<String, String> {
    if bytes.is_empty() {
        return Err("Empty byte slice".to_string());
    }

    let hex_literals: Vec<String> = bytes
        .iter()
        .map(|b| format!("0x{:02x}", b))
        .collect();

    let result = hex_literals.join(", ");
    Ok(result)
}

/// Convert string to a list of bytes
pub fn string_to_bytes(s: String) -> Vec<u8> {
    s.into_bytes()
}
