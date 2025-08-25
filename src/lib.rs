pub mod basics {
    pub mod hex_base64;
    pub mod fixed_xor;
}
pub mod utils;
pub use utils::{select_input, run_python};

pub use basics::hex_base64::{hex_to_base64, hex_to_bytes, bytes_to_base64};
pub use basics::fixed_xor::{bytes_to_hex, xor_bytes};
