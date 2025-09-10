/* Cryptopals - Set 2, Challenge 10
Solution Started: September 9, 2025
Puzzle Link: https://cryptopals.com/sets/2/challenges/10
Solution by: Abbas Moosajee
Brief: [Implement CBC Mode] */


use std::{env, fs::File, error::Error, io::{BufRead, BufReader}};
use cryptopals::{base64_to_bytes, string_to_bytes, AesStandard};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Set 02, Challenge 10: Implement CBC Mode");

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
    let enc_key: [u8; 16] = encryption_bytes.try_into().expect("Key must be exactly 16 bytes");
    let iv_box = [0u8; 16]; // same as used in encryption


    let aes = AesStandard::new(&enc_key)?;

    let ciphertext = aes.decrypt_cbc(&bytes_list, &iv_box).unwrap();
    println!("\n{}", String::from_utf8_lossy(&ciphertext));

    Ok(())
}

