/* Cryptopals - Set 1, Challenge 4
Solution Started: August 26, 2025
Puzzle Link: https://cryptopals.com/sets/1/challenges/4
Solution by: Abbas Moosajee
Brief: [Detect single-character XOR] */

use std::collections::HashMap;
use std::{env, fs::File, error::Error, io::{BufRead, BufReader}};
use cryptopals::{hex_to_bytes, ALL_CHARS, test_xor_key, chi_square};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Set 01, Challenge 04: Detect single-character XOR");

    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "inputs/challenge_04_input.txt"
    };

    let file: File = File::open(file_path)?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut overall_min_score: f64 = f64::INFINITY;
    let mut overall_best_key: char = '\0';
    let mut overall_best_message: String = String::new();

    for line in reader.lines() {
        let line: String = line?;
        let raw_bytes: Vec<u8> = hex_to_bytes(&line)?;

        let mut cipher_dict: HashMap<String, String> = HashMap::new();

        for c in ALL_CHARS.chars() {
            let norm_key: String = c.to_ascii_lowercase().to_string();
            if cipher_dict.contains_key(&norm_key) {
                continue;
            }

            let test_cipher: String = test_xor_key(&raw_bytes, &c.to_string());
            let chi_score: f64 = chi_square(&test_cipher);

            cipher_dict.insert(norm_key.clone(), test_cipher.clone());

            if chi_score < overall_min_score {
                overall_min_score = chi_score;
                overall_best_key = c;
                overall_best_message = test_cipher;
            }
        }
    }

    println!(
        "Best key: '{}' with chi-square: {}",
        overall_best_key, overall_min_score
    );
    println!("Message: {}", overall_best_message);

    Ok(())
}