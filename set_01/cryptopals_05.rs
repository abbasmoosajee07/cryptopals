/* Cryptopals - Set 1, Challenge 5
Solution Started: August 26, 2025
Puzzle Link: https://cryptopals.com/sets/1/challenges/5
Solution by: Abbas Moosajee
Brief: [Implement repeating-key XOR] */

use std::{env, error::Error};
use cryptopals::{select_input, string_to_bytes, bytes_to_hex, xor_bytes_repeating};
// See the relevant functions in: cryptopals\src\basics\xor_bytes.rs


fn main() -> Result<(), Box<dyn Error>> {
    println!("Set 01, Challenge 05: Implement repeating-key XOR");

    let default_input: &'static str = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal^ICE";

    // Get the first command-line argument
    let args: Vec<String> = env::args().collect();
    let parsed_args: Option<&str> = args.get(1).map(|s: &String| s.as_str());

    let use_input: String = select_input(parsed_args, default_input);

    // Split into the two hex strings
    let parts: Vec<&str> = use_input.split('^').collect();
    let main_str: &str = parts.get(0).unwrap_or(&"").trim();
    let comp_str: &str = parts.get(1).unwrap_or(&"").trim();

    // Convert string → bytes
    let byte_list: Vec<u8> = string_to_bytes(main_str.to_string());
    let comp_bytes: Vec<u8> = string_to_bytes(comp_str.to_string());

    let xor_result: Vec<u8> = xor_bytes_repeating(&byte_list, &comp_bytes);

    println!("String Input:{:?}", use_input);
    println!("  XOR Result:{:?}", bytes_to_hex(&xor_result));

    Ok(())
}
