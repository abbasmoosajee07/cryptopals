/* Cryptopals - Set 1, Challenge 1
Solution Started: August 25, 2025
Puzzle Link: https://cryptopals.com/sets/1/challenges/1
Solution by: Abbas Moosajee
Brief: [Convert Hex to Base64] */

use std::{env, io};
use cryptopals::{select_input, hex_to_base64, hex_to_bytes, bytes_to_base64};
// See the relevant functions in: cryptopals\src\basics\hex_base64.rs

fn main() -> io::Result<()> {
    println!("Set 01, Challenge 01: Convert Hex to Base64");

    let args: Vec<String> = env::args().collect();
    let parsed_args: Option<&str> = args.get(1).map(|s| s.as_str());

    // Default hex string if no input is given
    let default_hex: &'static str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    // Determine input: argument, file content, or default
    let hex_string: String = select_input(parsed_args, default_hex);

    let raw_bytes: Vec<u8> = hex_to_bytes(&hex_string);
    let manual_b64: String = bytes_to_base64(&raw_bytes);
    let lib_b64: String = hex_to_base64(&hex_string);

    println!("   Bytes(Text): b'{:?}'", String::from_utf8_lossy(&raw_bytes));
    println!("     Hex Input: {}", hex_string);

    // --- Cross-check --- //
    if manual_b64 == lib_b64 {
        println!(" Manual Base64: {}", manual_b64);
    } else {
        println!("✗ Mismatch! Something’s wrong.");
        println!("Library Base64: {}", lib_b64);
    }
    Ok(())
}
