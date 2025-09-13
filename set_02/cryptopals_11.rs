/* Cryptopals - Set 2, Challenge 11
Solution Started: September 10, 2025
Puzzle Link: https://cryptopals.com/sets/2/challenges/11
Solution by: Abbas Moosajee
Brief: [An ECB/CBC detection oracle] */

use std::{env, error::Error};
use cryptopals::{select_input, string_to_bytes, pkcs7_padding, encryption_oracle_random, detect_mode};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Set 02, Challenge 11: An ECB/CBC detection oracle");
    let plain_text: &'static str = "Hello,World!";

    let args: Vec<String> = env::args().collect();
    let parsed_args: Option<&str> = args.get(1).map(|s| s.as_str());

    // Determine input: argument, file content, or default
    let input: String = select_input(parsed_args, plain_text);

    // Input with many repeating blocks → reveals ECB
    let base_bytes: Vec<u8> = string_to_bytes(input.to_string());
    let plain_bytes: Vec<u8> = pkcs7_padding(&base_bytes, 64);
    println!("Test Text: {}", input);
    for _ in 0..13 {
        let (ciphertext, actual) = encryption_oracle_random(&plain_bytes);
        let detected: &'static str = detect_mode(&ciphertext);
        let check_oracle: bool = actual == detected;
        println!("Validity = {} | Actual: {:<3}  Detected: {:<3}", check_oracle, actual, detected);
    }
    Ok(())
}
