/* Cryptopals - Set 1, Challenge 7
Solution Started: August 27, 2025
Puzzle Link: https://cryptopals.com/sets/1/challenges/7
Solution by: Abbas Moosajee
Brief: [AES in ECB mode] */

use std::{env, fs::File, error::Error, io::{BufRead, BufReader}};
use cryptopals::{base64_to_bytes, string_to_bytes, AesStandard};

fn vec_to_array_16(bytes: Vec<u8>) -> [u8; 16] {
    bytes.try_into().expect("Key must be exactly 16 bytes")
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Set 01, Challenge 07: AES in ECB mode");

    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]  // Use the provided file path
    } else {
        "inputs/challenge_07_input.txt"  // Default file
    };

    let file: File = File::open(file_path)?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut bytes_list: Vec<u8> = Vec::new();
    for line_result in reader.lines() {
        let line: String = line_result?;
        match base64_to_bytes(&line) {
            Ok(decoded) => bytes_list.extend(decoded),
            Err(e) => eprintln!("Failed to decode line: {} ({})", line, e),
        }
    }

    let encryption_key: &'static str= "YELLOW SUBMARINE";
    let encryption_bytes: Vec<u8> = string_to_bytes(encryption_key.to_string());
    let enc_key: [u8; 16] = vec_to_array_16(encryption_bytes);

    let aes = AesStandard::new(&enc_key)?;

    let ciphertext = aes.decrypt_ecb(&bytes_list).unwrap();
    println!("\n{}", String::from_utf8_lossy(&ciphertext));

    Ok(())
}


