
use itertools::Itertools; // 0.10 or latest version
use std::collections::HashMap;
use crate::{ALL_CHARS};
use crate::basics::xor_cipher::{test_xor_key, chi_square};

/// Compute Hamming (edit) distance between two byte slices
pub fn edit_distance(bytes_1: Vec<u8>, bytes_2: Vec<u8>) -> u32 {
    assert_eq!(bytes_1.len(), bytes_2.len(), "Inputs must be the same length");

    bytes_1
        .iter()
        .zip(bytes_2.iter())
        .map(|(&b1, &b2)| (b1 ^ b2).count_ones())
        .sum()
}

/// Compute the average normalized edit distance for a list of key sizes
pub fn compute_keysize(bytes_list: &[u8], min_keysize: usize, max_keysize: usize, num_chunks: usize) -> HashMap<usize, f32> {
    let keysize_list: Vec<usize> = (min_keysize..=max_keysize).collect();
    let mut keysize_dict: HashMap<usize, f32> = HashMap::new();

    for &test_keysize in &keysize_list {
        // Take the first `num_chunks` chunks of size `test_keysize`
        let chunks: Vec<&[u8]> = bytes_list.chunks(test_keysize).take(num_chunks).collect();

        // Generate all combinations of 2 chunks and compute edit distances
        let mut edit_list: Vec<i32> = Vec::new();
        for combo in chunks.iter().combinations(2) {
            let combo_edit: u32 = edit_distance(combo[0].to_vec(), combo[1].to_vec());
            edit_list.push(combo_edit as i32);
        }

        if !edit_list.is_empty() {
            let edit_sum: i32 = edit_list.iter().sum();
            let edit_len: usize = edit_list.len();
            // Convert to f32 for float division
            let normalized_edits: f32 = (edit_sum as f32 / edit_len as f32) / test_keysize as f32;

            // Update the HashMap storing floats
            keysize_dict.insert(test_keysize, normalized_edits as f32);
        }
    }

    keysize_dict
}

/// Transpose chunks of text into blocks
pub fn transpose_chunks(bytes_list: &Vec<u8>, keysize: usize) -> Vec<Vec<u8>> {
    let mut chunks: Vec<Vec<u8>> = Vec::with_capacity(keysize);

    for i in 0..keysize {
        let mut chunk = Vec::new();
        let mut j = i;
        while j < bytes_list.len() {
            chunk.push(bytes_list[j]);
            j += keysize;
        }
        chunks.push(chunk);
    }

    chunks
}

pub fn decipher_xor_key(bytes_list: Vec<u8>, test_key: usize) -> (Vec<u8>, f64) {
    let chunks: Vec<Vec<u8>> = transpose_chunks(&bytes_list, test_key);
    let mut key_bytes: Vec<u8> = Vec::new();
    let mut total_score: f64 = 0.00;

    for (_, raw_bytes) in chunks.iter().enumerate() {
        let mut cipher_dict: HashMap<String, String> = HashMap::new();
        let mut min_score: f64 = f64::INFINITY;
        let mut key_char: char = '\0';

        for c in ALL_CHARS.chars() {
            let norm_key: String = c.to_string();
            if cipher_dict.contains_key(&norm_key) {
                continue;
            }

            let test_cipher: String = test_xor_key(&raw_bytes, &c.to_string());
            let chi_score: f64 = chi_square(&test_cipher);
            cipher_dict.insert(norm_key.clone(), test_cipher.clone());

            if chi_score <= min_score {
                min_score = chi_score;
                key_char = c;
            }
        }

        // push the byte instead of the char
        key_bytes.push(key_char as u8);
        total_score += min_score;
    }

    let average_score: f64 = total_score / test_key as f64;
    (key_bytes, average_score)

}
