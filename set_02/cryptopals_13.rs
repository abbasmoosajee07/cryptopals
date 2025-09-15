/* Cryptopals - Set 2, Challenge 13
Solution Started: September 13, 2025
Puzzle Link: https://cryptopals.com/sets/2/challenges/13
Solution by: Abbas Moosajee
Brief: [ECB cut-and-paste] */

use std::{env, error::Error};
use cryptopals::{select_input};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Set 02, Challenge 13: ECB cut-and-paste");

    // Get the first command-line argument, or default to "Lang06_input.txt"
    let args: Vec<String> = env::args().collect();
    let default_suffix: &'static str = "foo=bar&baz=qux&zap=zazzle";
    let parsed_args: Option<&str> = args.get(1).map(|s| s.as_str());
    let parsed_input: String = select_input(parsed_args, default_suffix);
    println!("{}", parsed_input);
    Ok(())
}
