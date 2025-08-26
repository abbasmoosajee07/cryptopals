/* Cryptopals - Set 1, Challenge 3
Solution Started: August 25, 2025
Puzzle Link: https://cryptopals.com/sets/1/challenges/3
Solution by: Abbas Moosajee
Brief: [Single-byte XOR Cipher] */

use std::{env, io};
use std::collections::HashMap;
use cryptopals::{select_input, hex_to_bytes, ALL_CHARS, test_xor_key, chi_square};
// See the relevant functions in: cryptopals\src\basics\xor_cipher.rs

fn main() -> io::Result<()> {

    println!("Set 01, Challenge 03: Single-byte XOR cipher");
    let default_input: &'static str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    let args: Vec<String> = env::args().collect();
    let parsed_args: Option<&str> = args.get(1).map(|s: &String| s.as_str());

    let hex_input: String = select_input(parsed_args, default_input);
    let raw_bytes: Vec<u8> = hex_to_bytes(&hex_input);

    println!("Hex Input:\\n{}", hex_input);
    let mut cipher_dict: HashMap<String, String> = HashMap::new();
    let mut min_score: f64 = f64::INFINITY; // start with infinity
    let mut best_key: char = '\0'; // placeholder

    for c in ALL_CHARS.chars() {
        // normalize key (lowercase)
        let norm_key = c.to_ascii_uppercase().to_string();

        // skip if this normalized key already exists
        if cipher_dict.contains_key(&norm_key) {
            continue;
        }

        let test_cipher = test_xor_key(&raw_bytes, c.to_string().as_str());
        let chi_score: f64 = chi_square(&test_cipher);

        cipher_dict.insert(norm_key.clone(), test_cipher);

        if chi_score < min_score {
            min_score = chi_score;
            best_key = c;
        }
    }

    println!("Best key: {} with chi-square: {}", best_key, min_score);
    let key_str = best_key.to_string(); // convert char → String
    if let Some(message) = cipher_dict.get(&key_str.to_uppercase()) {
        println!("Message: {}", message);
    } else {
        println!("Key not found in cipher_dict");
    }

    Ok(())
}
