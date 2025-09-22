/* Cryptopals - Set 2, Challenge 14
Solution Started: September 22, 2025
Puzzle Link: https://cryptopals.com/sets/2/challenges/14
Solution by: Abbas Moosajee
Brief: [Byte-at-a-time ECB decryption (Harder)] */

use std::{env, error::Error};

use cryptopals::{
    select_input, base64_to_bytes, confirm_ecb,
    encryption_oracle, find_block_size, find_prefix_len
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Set 02, Challenge 14: Byte-at-a-time ECB decryption (Harder)");

    // Get secret suffix from command line or use default
    let default_suffix: &'static str = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg\naGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq\ndXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg\nYnkK";
    let args: Vec<String> = env::args().collect();
    let parsed_args: Option<&str> = args.get(1).map(|s| s.as_str());
    let base64_inp: String = select_input(parsed_args, default_suffix);
    let suffix_bytes: Vec<u8> = base64_to_bytes(&base64_inp.to_string()).expect("Invalid base64");

    // Step 1: Detect the block size used by the cipher
    let block_size: usize = find_block_size(encryption_oracle, suffix_bytes.clone());
    println!("Detected block size: {}", block_size);

    // Step 2: Confirm that ECB mode is being used
    confirm_ecb(encryption_oracle, suffix_bytes.clone(), block_size);
    println!("Confirmed ECB mode");

    // Step 3: Detect random prefix length
    let prefix_len: usize = find_prefix_len(encryption_oracle, suffix_bytes.clone(), block_size);
    println!("Detected random prefix length: {}", prefix_len);

    // Step 4: compute alignment padding so that our controlled input starts at a block boundary
    let align_pad: usize = (block_size - (prefix_len % block_size)) % block_size;
    println!("Computed alignment padding: {} (so controlled bytes start at block boundary)", align_pad);

    // Step 5: Get expected result for verification (for progress/debugging)
    let expected_secret: Vec<u8> = base64_to_bytes(default_suffix).expect("Invalid base64");

    // Step 6: Byte-at-a-time decryption, accounting for prefix and alignment
    let mut known_bytes: Vec<u8> = Vec::new();
    let total_bytes_to_decrypt: usize = expected_secret.len();

    println!("Total bytes to decrypt: {}", total_bytes_to_decrypt);

    // The block index offset of the first controllable block in ciphertext
    // after applying align_pad is:
    let offset_blocks: usize = (prefix_len + align_pad) / block_size;

    for i in 0..total_bytes_to_decrypt {
        // number of filler 'A's to make the next unknown byte appear at the end of a block
        let in_block_index = known_bytes.len() % block_size;
        let pad_len = (block_size - 1) - in_block_index;
        let attack_prefix = vec![b'A'; align_pad + pad_len];

        // Build dictionary: for candidate byte b, oracle(attack_prefix + known + [b]) -> ciphertext block
        let target_block_index = offset_blocks + (known_bytes.len() / block_size);
        // produce target ciphertext for actual unknown byte
        let ct_target = encryption_oracle(&attack_prefix, suffix_bytes.clone());
        let start = target_block_index * block_size;
        let end = start + block_size;
        if end > ct_target.len() {
            // shouldn't happen, but guard
            println!("Ciphertext too short when computing target block (i={})", i);
            break;
        }
        let target_block = &ct_target[start..end];

        // Build dictionary
        let mut found_byte: Option<u8> = None;
        for b in 0u8..=255u8 {
            // construct probe = attack_prefix + known_bytes + [b]
            let mut probe: Vec<u8> = Vec::with_capacity(attack_prefix.len() + known_bytes.len() + 1);
            probe.extend_from_slice(&attack_prefix);
            probe.extend_from_slice(&known_bytes);
            probe.push(b);

            let ct_probe = encryption_oracle(&probe, suffix_bytes.clone());
            let probe_block = &ct_probe[start..end];
            if probe_block == target_block {
                found_byte = Some(b);
                break;
            }
        }

        if let Some(nb) = found_byte {
            known_bytes.push(nb);
            // Optionally show progress per block
            if known_bytes.len() % block_size == 0 || known_bytes.len() == total_bytes_to_decrypt {
                let _current_text = String::from_utf8_lossy(&known_bytes);
                // println!("Decrypted {} bytes: {}", known_bytes.len(), _current_text);
            }
        } else {
            println!("Failed to find byte at position {}", i);
            break;
        }
    }

    println!("\nFinal decrypted result:\n{}", String::from_utf8_lossy(&known_bytes));

    // Verify
    if known_bytes == expected_secret {
        println!("SUCCESS: Decrypted secret matches expected!");
    } else {
        println!("FAILURE: Decrypted secret doesn't match expected!");
        println!("Decrypted length: {}, Expected length: {}", known_bytes.len(), expected_secret.len());

        let min_len: usize = known_bytes.len().min(expected_secret.len());
        for i in 0..min_len {
            if known_bytes[i] != expected_secret[i] {
                println!("First difference at byte {}: got {}, expected {}", i, known_bytes[i], expected_secret[i]);
                println!("Context: '{}' vs '{}'",
                    String::from_utf8_lossy(&known_bytes[i.saturating_sub(5)..(i+5).min(known_bytes.len())]),
                    String::from_utf8_lossy(&expected_secret[i.saturating_sub(5)..(i+5).min(expected_secret.len())])
                );
                break;
            }
        }

        if known_bytes.len() > expected_secret.len() {
            println!("Extra bytes at end: {:?}", &known_bytes[expected_secret.len()..]);
        }
    }

    Ok(())
}
