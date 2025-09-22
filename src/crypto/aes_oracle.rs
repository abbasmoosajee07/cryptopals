use rand::{Rng, random};
use std::collections::HashSet;
use crate::crypto::aes_encyption::{AesStandard,  gen_key, pkcs7_padding};
use std::cell::RefCell;

thread_local! {
    pub static KEY: RefCell<Option<Vec<u8>>> = RefCell::new(None);
    pub static PREFIX: RefCell<Option<Vec<u8>>> = RefCell::new(None);
}
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
    oracle: impl Fn(&[u8], Vec<u8>) -> Vec<u8>,
    suffix_bytes: Vec<u8>,
    block_size: usize
) -> bool {
    // Feed in a long string of 'A's (long enough to cover any prefix misalignment)
    let probe: Vec<u8> = vec![b'A'; block_size * 64];
    let ct: Vec<u8> = oracle(&probe, suffix_bytes);

    let num_blocks: usize = ct.len() / block_size;
    let mut blocks: Vec<&[u8]> = Vec::with_capacity(num_blocks);

    for i in 0..num_blocks {
        blocks.push(&ct[i * block_size..(i + 1) * block_size]);
    }

    // Look for any repeated adjacent block
    for i in 0..(blocks.len() - 1) {
        if blocks[i] == blocks[i + 1] {
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

/// Encryption oracle: Appends secret suffix and encrypts with ECB
/// data: User-controlled input bytes
/// suffix_bytes: Secret bytes to be decrypted
pub fn encryption_oracle(data: &[u8], suffix_bytes: Vec<u8>) -> Vec<u8> {
    KEY.with(|key_cell: &RefCell<Option<Vec<u8>>>| {
        let mut key_opt: std::cell::RefMut<'_, Option<Vec<u8>>> = key_cell.borrow_mut();

        // Generate key once and reuse it (thread-local storage)
        if key_opt.is_none() {
            *key_opt = Some(gen_key(16));
        }
        let key: &Vec<u8> = key_opt.as_ref().unwrap();

        // Combine user input with secret suffix
        let mut plaintext: Vec<u8> = Vec::new();
        plaintext.extend_from_slice(data);
        plaintext.extend_from_slice(&suffix_bytes);

        // PKCS#7 pad to block size and encrypt with ECB
        let padded: Vec<u8> = pkcs7_padding(&plaintext, 16);
        let cipher: AesStandard = AesStandard::new(key).unwrap();
        cipher.encrypt_ecb(&padded).unwrap()
    })
}


/// Find prefix length (unknown random bytes before our controllable input)
/// Strategy:
/// For pad in 0..block_size:
///   send input: 'A' * (2*block_size + pad)
///   find first index i where block i == block i+1
///   then prefix_len = i*block_size - pad
pub fn find_prefix_len(
    oracle: impl Fn(&[u8], Vec<u8>) -> Vec<u8>,
    suffix_bytes: Vec<u8>,
    block_size: usize
) -> usize {
    // We'll create a buffer of 2*block_size + pad 'A's and search for two identical adjacent blocks.
    for pad in 0..block_size {
        let probe: Vec<u8> = vec![b'A'; block_size * 2 + pad];
        let ct: Vec<u8> = oracle(&probe, suffix_bytes.clone());

        // split into blocks
        let num_blocks: usize = ct.len() / block_size;
        let mut blocks: Vec<&[u8]> = Vec::with_capacity(num_blocks);
        for i in 0..num_blocks {
            blocks.push(&ct[i*block_size..(i+1)*block_size]);
        }

        // find adjacent identical blocks
        for i in 0..(blocks.len().saturating_sub(1)) {
            if blocks[i] == blocks[i+1] {
                // found repeated adjacent blocks at index i
                // prefix length = i*block_size - pad
                let prefix_len: usize = i * block_size;
                if prefix_len >= pad {
                    return prefix_len - pad;
                } else {
                    // shouldn't happen, but guard
                    return 0;
                }
            }
        }
    }

    // if not found, fallback to 0
    0
}

