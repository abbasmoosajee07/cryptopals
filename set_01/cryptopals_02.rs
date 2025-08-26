/* Cryptopals - Set 1, Challenge 2
Solution Started: August 25, 2025
Puzzle Link: https://cryptopals.com/sets/1/challenges/2
Solution by: Abbas Moosajee
Brief: [Fixed XOR] */

use std::{env, io};
use cryptopals::{select_input, hex_to_bytes, bytes_to_hex, xor_bytes};
// See the relevant functions in: cryptopals\src\basics\fixed_xor.rs

fn main() -> io::Result<()> {

    println!("Set 01, Challenge 02: Fixed XOR");
    let default_input: &'static str = "1c0111001f010100061a024b53535009181c^686974207468652062756c6c277320657965";

    // Get the first command-line argument
    let args: Vec<String> = env::args().collect();
    let parsed_args: Option<&str> = args.get(1).map(|s: &String| s.as_str());

    let use_input: String = select_input(parsed_args, default_input);

    // Split into the two hex strings
    let parts: Vec<&str> = use_input.split('^').collect();
    let inp_1: &str = parts.get(0).unwrap_or(&"").trim();
    let inp_2: &str = parts.get(1).unwrap_or(&"").trim();

    // Convert hex → bytes
    let hex_1: Vec<u8> = hex_to_bytes(inp_1);
    let hex_2: Vec<u8> = hex_to_bytes(inp_2);

    // XOR
    let xor_out: Vec<u8> = xor_bytes(&hex_1, &hex_2);

    // Print results
    println!("XOR Input: {} ^ {}", inp_1, inp_2);
    println!("XOR result (hex): {}", bytes_to_hex(&xor_out));
    println!("XOR result (text): {}", String::from_utf8_lossy(&xor_out));

    Ok(())
}


