/* Cryptopals - Set 2, Challenge 9
Solution Started: September 9, 2025
Puzzle Link: https://cryptopals.com/sets/2/challenges/9
Solution by: Abbas Moosajee
Brief: [Implement PKCS#7 padding] */

use std::{env, error::Error};
use cryptopals::{select_input, string_to_bytes, pkcs7_padding};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Set 02, Challenge 09: Implement PKCS#7 padding");
    let args: Vec<String> = env::args().collect();
    let parsed_args: Option<&str> = args.get(1).map(|s| s.as_str());

    // Default hex string if no input is given
    let default_hex: &'static str = "YELLOW SUBMARINE";

    // Determine input: argument, file content, or default
    let input: String = select_input(parsed_args, default_hex);
    let key: Vec<u8> = string_to_bytes(input);
    let padded: Vec<u8> = pkcs7_padding(&key, 20);

    println!(" Padded Bytes: {:?}", padded);
    println!("Padded String: {:?}", String::from_utf8_lossy(&padded));
    Ok(())
}
