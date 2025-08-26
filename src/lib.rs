pub const ALL_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

pub mod basics {
    pub mod hex_base64;
    pub mod xor_bytes;
    pub mod xor_cipher;
}
pub mod utils;
pub use utils::{select_input, run_python, string_to_bytes};

pub use basics::hex_base64::{hex_to_base64, hex_to_bytes, bytes_to_base64};
pub use basics::xor_bytes::{bytes_to_hex, xor_bytes_fixed, xor_bytes_repeating};
pub use basics::xor_cipher::{test_xor_key, english_frequencies, chi_square};
