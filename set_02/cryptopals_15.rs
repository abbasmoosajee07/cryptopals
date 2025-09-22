/* Cryptopals - Set 2, Challenge 15
Solution Started: September 22, 2025
Puzzle Link: https://cryptopals.com/sets/2/challenges/15
Solution by: Abbas Moosajee
Brief: [PKCS#7 padding validation] */

use std::{io};
use cryptopals::{pkcs7_unpadding};

fn main() -> io::Result<()> {
    println!("Set 02, Challenge 15: PKCS#7 padding validation");

    let test_cases = vec![
        b"ICE ICE BABY\x04\x04\x04\x04" as &[u8],
        b"ICE ICE BABY\x05\x05\x05\x05",
        b"ICE ICE BABY\x01\x02\x03\x04",
    ];

    for case in test_cases {
        match pkcs7_unpadding(case) {
            Ok(stripped) => {
                println!("{:?} -> {:?}", case, String::from_utf8_lossy(&stripped));
            }
            Err(e) => {
                println!("{:?} -> ERROR: {}", case, e);
            }
        }
    }

    Ok(())
}
