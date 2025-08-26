use crate::basics::xor_bytes::xor_bytes_fixed;
use std::collections::HashMap;

/// XOR a byte slice with a single-character key (repeated).
pub fn test_xor_key(bytes_data: &[u8], key_str: &str) -> String {
    let key_char = key_str.chars().next().expect("Empty key string");
    let key_val = key_char as u8;

    // Build a "repeated key" the same length as the input
    let key_stream: Vec<u8> = std::iter::repeat(key_val)
        .take(bytes_data.len())
        .collect();

    // Use xor_bytes
    let xor_out = xor_bytes_fixed(bytes_data, &key_stream);

    // Convert back to lossy UTF-8 string (like Python `chr`)
    let test_cipher = String::from_utf8_lossy(&xor_out).to_string();
    test_cipher
}

/// Frequencies of letters in the english alphabet to form words
pub fn english_frequencies() -> HashMap<u8, f64> {
    let mut freq = HashMap::new();
    freq.insert(b'a', 0.07743208627550165);
    freq.insert(b'b', 0.01402241586697527);
    freq.insert(b'c', 0.02665670667329359);
    freq.insert(b'd', 0.04920785702311875);
    freq.insert(b'e', 0.13464518994079883);
    freq.insert(b'f', 0.025036247121552113);
    freq.insert(b'g', 0.017007472935972733);
    freq.insert(b'h', 0.05719839895067157);
    freq.insert(b'i', 0.06294794236928244);
    freq.insert(b'j', 0.001267546400727001);
    freq.insert(b'k', 0.005084890317533608);
    freq.insert(b'l', 0.03706176274237046);
    freq.insert(b'm', 0.030277007414117114);
    freq.insert(b'n', 0.07125316518982316);
    freq.insert(b'o', 0.07380002176297765);
    freq.insert(b'p', 0.017513315119093483);
    freq.insert(b'q', 0.0009499245648139707);
    freq.insert(b'r', 0.06107162078305546);
    freq.insert(b's', 0.061262782073188304);
    freq.insert(b't', 0.08760480785349399);
    freq.insert(b'u', 0.030426995503298266);
    freq.insert(b'v', 0.01113735085743191);
    freq.insert(b'w', 0.02168063124398945);
    freq.insert(b'x', 0.0019880774173815607);
    freq.insert(b'y', 0.022836421813561863);
    freq.insert(b'z', 0.0006293617859758195);
    freq
}

/// Score a string by comparing its letter frequency distribution
/// to expected English frequencies.
/// Lower scores are better (closer to natural English).
pub fn chi_square(text: &str) -> f64 {
    let bytes = text.as_bytes();
    let freqs = english_frequencies();
    let l = bytes.len() as f64;
    if l == 0.0 {
        return f64::MAX; // avoid division by zero
    }

    let mut score = 0.0;

    for (&letter, &expected) in freqs.iter() {
        let actual =
            bytes.iter().filter(|&&c| c == letter).count() as f64 / l;
        let err = (expected - actual).abs();
        score += err;
    }

    score
}

