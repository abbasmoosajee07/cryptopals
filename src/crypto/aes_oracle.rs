use rand::{Rng, random};
use std::collections::HashSet;
use crate::crypto::aes_encyption::{AesStandard,  gen_key, pkcs7_padding};

/// Encryption oracle: randomly encrypts with ECB or CBC
pub fn encryption_oracle_random(data: &[u8]) -> (Vec<u8>, &'static str) {
    let key = gen_key(16);
    let mut rng = rand::thread_rng();

    // Add random prefix + suffix
    let prefix_len = rng.gen_range(5..=10);
    let suffix_len = rng.gen_range(5..=10);
    let mut input = Vec::with_capacity(prefix_len + data.len() + suffix_len);

    // prefix random bytes
    for _ in 0..prefix_len { input.push(random::<u8>());}
    input.extend_from_slice(data);
    // suffix random bytes
    for _ in 0..suffix_len {input.push(random::<u8>());}

    // Pad input
    let padded = pkcs7_padding(&input, 16);

    // Pick ECB or CBC
    let cipher = AesStandard::new(&key).unwrap();
    if rng.gen_bool(0.5) {
        let ciphertext = cipher.encrypt_ecb(&padded).unwrap();
        (ciphertext, "ECB")
    } else {
        let iv: [u8; 16] = gen_key(16).try_into().unwrap();
        let ciphertext = cipher.encrypt_cbc(&padded, &iv).unwrap();
        (ciphertext, "CBC")
    }
}

/// Detect if ECB was used by checking for repeated 16-byte blocks
pub fn detect_mode(ciphertext: &[u8]) -> &'static str {
    let mut seen = HashSet::new();
    for block in ciphertext.chunks(16) {
        if !seen.insert(block) {
            return "ECB";
        }
    }
    "CBC"
}


/// Confirms ECB mode by testing with repeating input blocks
/// encryption_fn: The encryption oracle function to test
/// suffix_bytes: The secret suffix bytes to append
/// block_size: The detected block size to verify
pub fn confirm_ecb(
    encryption_fn: impl Fn(&[u8], Vec<u8>) -> Vec<u8>,
    suffix_bytes: Vec<u8>,
    block_size: usize
) -> bool {
    // Create input with repeating blocks to test ECB behavior
    let input: Vec<u8> = vec![b'A'; block_size * 3];
    let ciphertext: Vec<u8> = encryption_fn(&input, suffix_bytes);

    // Check if any two consecutive blocks are identical (ECB characteristic)
    for i in 0..2 {
        let block1: &[u8] = &ciphertext[i * block_size..(i + 1) * block_size];
        let block2: &[u8] = &ciphertext[(i + 1) * block_size..(i + 2) * block_size];

        if block1 == block2 {
            return true; // ECB confirmed
        }
    }
    panic!("Not using ECB mode!");
}


/// Detects the block size by monitoring ciphertext length changes
/// encryption_fn: The encryption oracle function
/// suffix_bytes: The secret suffix bytes to append
pub fn find_block_size(
    encryption_fn: impl Fn(&[u8], Vec<u8>) -> Vec<u8>,
    suffix_bytes: Vec<u8>,
) -> usize {
    // Get initial ciphertext length with empty input
    let initial_len: usize = encryption_fn(&[], suffix_bytes.to_vec()).len();

    // Gradually increase input size until ciphertext length changes
    for i in 1..=64 {
        let input: Vec<u8> = vec![b'A'; i];
        let current_len: usize = encryption_fn(&input, suffix_bytes.to_vec()).len();

        // When length changes, the difference is the block size
        if current_len != initial_len {
            return current_len - initial_len;
        }
    }
    panic!("Could not detect block size");
}

/// Finds the next byte of the secret using byte-at-a-time attack
/// encryption_fn: The encryption oracle function
/// suffix_bytes: The secret suffix bytes to decrypt
/// known_bytes: Already decrypted bytes of the secret
/// block_size: The detected block size
pub fn find_next_byte(
    encryption_fn: impl Fn(&[u8], Vec<u8>) -> Vec<u8>,
    suffix_bytes: Vec<u8>,
    known_bytes: &[u8], 
    block_size: usize
) -> Option<u8> {
    // Calculate padding needed to position target byte at block end
    let prefix_len: usize = block_size - (known_bytes.len() % block_size) - 1;
    let prefix: Vec<u8> = vec![b'A'; prefix_len];

    // Get target ciphertext block for the current position
    let target_ciphertext: Vec<u8> = encryption_fn(&prefix, suffix_bytes.to_vec());
    let target_block_index: usize = (prefix_len + known_bytes.len()) / block_size;
    let target_block: &[u8] = &target_ciphertext[target_block_index * block_size..(target_block_index + 1) * block_size];

    // Brute-force all possible bytes (0-255) to find matching ciphertext
    for candidate in 0u8..=255 {
        let mut test_input: Vec<u8> = prefix.clone();
        test_input.extend_from_slice(known_bytes);
        test_input.push(candidate);

        let test_ciphertext: Vec<u8> = encryption_fn(&test_input, suffix_bytes.to_vec());
        let test_block: &[u8] = &test_ciphertext[target_block_index * block_size..(target_block_index + 1) * block_size];

        // If ciphertext blocks match, we found the correct byte
        if test_block == target_block {
            return Some(candidate);
        }
    }

    None // No match found (end of message or error)
}
