// All printable ASCII characters for XOR/key cracking
pub const ALL_CHARS: &str =
    " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

// Core modules
pub mod basics {
    pub mod hex_base64;
    pub mod xor_bytes;
    pub mod xor_cipher;
    pub mod repeating_key;
}

pub mod utils;

// Re-export common utilities
pub use utils::{select_input, run_python, string_to_bytes};

// Re-export basics submodules and functions
pub use basics::hex_base64::{hex_to_base64, hex_to_bytes, bytes_to_base64};
pub use basics::xor_bytes::{bytes_to_hex, xor_bytes_fixed, xor_bytes_repeating};
pub use basics::xor_cipher::{test_xor_key, english_frequencies, chi_square};
pub use basics::repeating_key::{
    bytes_to_bits,
    edit_distance,
    base64_to_bytes,
    compute_keysize,
    transpose_chunks,
    decipher_xor_key,
};
