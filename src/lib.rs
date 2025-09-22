// All printable ASCII characters for XOR/key cracking
pub const ALL_CHARS: &str =
    " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

// Core modules
pub mod basics {
    pub mod use_rust_libs;
    pub mod encodings;
    pub mod xor_bytes;
    pub mod xor_cipher;
    pub mod repeating_key;

}
pub mod crypto{
    pub mod aes_constants;
    pub mod aes_encyption;
    pub mod aes_oracle;
}
pub mod utils;

// Re-export common utilities
pub use utils::{select_input, run_python, read_file};

// Re-export basics submodules and functions
pub use basics::use_rust_libs::{hex_to_base64_lib_func};
pub use basics::encodings::{
    hex_to_bytes, bytes_to_hex,
    base64_to_bytes, bytes_to_base64,
    bytes_to_bits, bits_to_bytes,
    hexify, string_to_bytes,
};

pub use basics::xor_bytes::{xor_bytes_fixed, xor_bytes_repeating};
pub use basics::xor_cipher::{test_xor_key, english_frequencies, chi_square};
pub use basics::repeating_key::{
    edit_distance, compute_keysize, transpose_chunks, decipher_xor_key,
};

pub use crypto::aes_encyption::{
    AesStandard, AesError, pkcs7_padding, pkcs7_unpadding, gen_key
};
pub use crypto::aes_oracle::{
    encryption_oracle_random, detect_mode, find_block_size,
    find_next_byte, confirm_ecb, encryption_oracle, find_prefix_len
};
