/* Cryptopals - Set 2, Challenge 12
Solution Started: September 11, 2025
Puzzle Link: https://cryptopals.com/sets/2/challenges/12
Solution by: Abbas Moosajee
Brief: [Byte-at-a-time ECB decryption (Simple)] */

use std::{env, error::Error, cell::RefCell};
use cryptopals::{
    select_input, base64_to_bytes,
    AesStandard, pkcs7_padding, gen_key, confirm_ecb, find_block_size, find_next_byte
};

thread_local! {
    static KEY: RefCell<Option<Vec<u8>>> = RefCell::new(None);
}

/// Encryption oracle: Appends secret suffix and encrypts with ECB
/// data: User-controlled input bytes
/// suffix_bytes: Secret bytes to be decrypted
fn encryption_oracle(data: &[u8], suffix_bytes: Vec<u8>) -> Vec<u8> {
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

fn main() -> Result<(), Box<dyn Error>> {
    println!("Set 02, Challenge 12: Byte-at-a-time ECB decryption (Simple)");

    // Get secret suffix from command line or use default
    let default_suffix: &'static str = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg\naGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq\ndXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg\nYnkK";
    let args: Vec<String> = env::args().collect();
    let parsed_args: Option<&str> = args.get(1).map(|s| s.as_str());
    let base64_inp: String = select_input(parsed_args, default_suffix);
    let suffix_bytes: Vec<u8> = base64_to_bytes(&base64_inp.to_string()).expect("Invalid base64");

    // Step 1: Detect the block size used by the cipher
    let block_size: usize = find_block_size(encryption_oracle, suffix_bytes.to_vec());
    println!("Detected block size: {}", block_size);

    // Step 2: Confirm that ECB mode is being used
    confirm_ecb(encryption_oracle, suffix_bytes.to_vec(), block_size);
    println!("Confirmed ECB mode");

    // Step 3: Get expected result for verification
    let expected_secret: Vec<u8> = base64_to_bytes(default_suffix).expect("Invalid base64");

    // Step 4: Perform byte-at-a-time decryption attack
    let mut known_bytes: Vec<u8> = Vec::new();
    let total_bytes_to_decrypt: usize = expected_secret.len();

    println!("Total bytes to decrypt: {}", total_bytes_to_decrypt);

    // Decrypt one byte at a time until complete
    for i in 0..total_bytes_to_decrypt {
        if let Some(next_byte) = find_next_byte(encryption_oracle, suffix_bytes.to_vec(), &known_bytes, block_size) {
            known_bytes.push(next_byte);

            // Show progress every block
            if known_bytes.len() % 16 == 0 || known_bytes.len() == total_bytes_to_decrypt {
                let _current_text: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&known_bytes);
                // println!("Decrypted {} bytes: {}", known_bytes.len(), _current_text);
            }
        } else {
            println!("Failed to find byte at position {}", i);
            break;
        }
    }

    // Display final decrypted result
    println!("\nFinal decrypted result: \n{}", String::from_utf8_lossy(&known_bytes));

    // Verify decryption was successful
    if known_bytes == expected_secret {
        println!("SUCCESS: Decrypted secret matches expected!");
    } else {
        println!("FAILURE: Decrypted secret doesn't match expected!");
        println!("Decrypted length: {}, Expected length: {}", known_bytes.len(), expected_secret.len());

        // Show first difference for debugging
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