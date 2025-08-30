/* Cryptopals - Set 1, Challenge 8
Solution Started: August 30, 2025
Puzzle Link: https://cryptopals.com/sets/1/challenges/8
Solution by: Abbas Moosajee
Brief: [Detect AES in ECB mode] */

use std::collections::HashSet;
use cryptopals::{hex_to_bytes};
use std::{env, fs::File, error::Error, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Set 01, Challenge 08: AES in ECB mode");

    // Get the first command-line argument, or default to "Lang06_input.txt"
    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]  // Use the provided file path
    } else {
        "inputs/challenge_08_input.txt"  // Default file
    };

    let file: File = File::open(file_path)?;
    let reader: BufReader<File> = BufReader::new(file);

    for (line_no, line_result) in reader.lines().enumerate() {
        let line: String = line_result?;
        match hex_to_bytes(&line) {
            Ok(decoded) => {
                let mut blocks: Vec<Vec<u8>> = Vec::new();
                let mut reported: HashSet<Vec<u8>> = HashSet::new();

                for block in decoded.chunks(16) {
                    let block_vec = block.to_vec();
                    if blocks.iter().any(|x| *x == block_vec) {
                        if reported.insert(block_vec.clone()) {
                            println!(
                                "Repeated block {:?} found at line {}",
                                block_vec, line_no
                            );
                        }
                    }
                    blocks.push(block_vec);
                }
            }
            Err(e) => eprintln!("Failed to decode line {}: {} ({})", line_no + 1, line, e),
        }
    }

    Ok(())
}
