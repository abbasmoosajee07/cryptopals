/* Cryptopals - Set 1, Challenge 6
Solution Started: August 26, 2025
Puzzle Link: https://cryptopals.com/sets/1/challenges/6
Solution by: Abbas Moosajee
Brief: [Break repeating-key XOR] */

use std::env;
use std::fs::File;
use std::collections::HashMap;
use std::io::{self, BufRead, BufReader};
use cryptopals::{base64_to_bytes, compute_keysize, decipher_xor_key};

fn main() -> io::Result<()> {
    println!("Set 01, Challenge 06: Break repeating-key XOR");

    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "inputs/challenge_06_input.txt"
    };

    println!("Reading file: {}", file_path);

    let file: File = File::open(file_path)?;
    let reader: BufReader<File> = BufReader::new(file);

    // Collect all decoded bytes line by line
    let mut bytes_list: Vec<u8> = Vec::new();
    for line_result in reader.lines() {
        let line: String = line_result?;

        match base64_to_bytes(&line) {
            Ok(decoded) => bytes_list.extend(decoded),
            Err(e) => eprintln!("Failed to decode line: {} ({})", line, e),
        }
    }

    // Print as raw byte vector
    let keysize_dict: HashMap<usize, f32> = compute_keysize(&bytes_list, 2, 40, 5);

    // Collect into a vector and sort by value ascending
    let mut sorted: Vec<(usize, f32)> = keysize_dict.into_iter().collect();
    sorted.sort_by(|a: &(usize, f32), b: &(usize, f32)| a.1.partial_cmp(&b.1).unwrap());
    let lowest_edit: Vec<(usize, f32)> = sorted.into_iter().take(5).collect();

    let mut best_bytes: Vec<u8> = Vec::new();
    let mut best_score: f64 = f64::INFINITY;
    let mut best_key: usize = 0;

    for (test_size, _) in lowest_edit {
        let (key_bytes, avg_chi_score) = decipher_xor_key(bytes_list.clone(), test_size);
        if avg_chi_score < best_score {
            best_score = avg_chi_score;
            best_bytes = key_bytes;
            best_key = test_size;
        }
    }

    let key_str: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&best_bytes);
    println!(" Valid Key Size: {:?} with confidence of {:?}", best_key, best_score);
    println!("Decoded Message: {:?}", key_str);

    Ok(())
}
