use std::fmt;
use std::error::Error;
use crate::crypto::aes_constants::{SBOX, R_CONSTANTS, INV_SBOX};

#[derive(Debug)]
pub enum AesError {
    InvalidLength(&'static str), // e.g., "Ciphertext must be multiple of 16 bytes"
    InvalidHexChar(char),
    HexStringOddLength,
}

impl fmt::Display for AesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AesError::InvalidLength(msg) => write!(f, "{}", msg),
            AesError::InvalidHexChar(c) => write!(f, "Invalid hex character '{}'", c),
            AesError::HexStringOddLength => write!(f, "Hex string must have even length"),
        }
    }
}

impl std::error::Error for AesError {}

/// | Variant | Key length | Expanded key size | Rounds |
/// | ------- | ---------- | ----------------- | ------ |
/// | AES-128 | 16 bytes   | 176 bytes         | 10     |
/// | AES-192 | 24 bytes   | 208 bytes         | 12     |
/// | AES-256 | 32 bytes   | 240 bytes         | 14     |
pub struct AesStandard {
    expanded_key: Vec<u8>, // variable-size expanded key (176/208/240)
    rounds: usize,         // number of rounds (10/12/14)
    _nk: usize,             // words in original key (4/6/8)
}

impl AesStandard {
    /// Create a new AES context: accepts 16, 24 or 32 byte keys.
    /// Defaults to AES block size = 16 bytes (Nb = 4).
    pub fn new(key: &[u8]) -> Result<Self, AesError> {
        match key.len() {
            16 | 24 | 32 => {}
            _ => return Err(AesError::InvalidLength("Invalid key size: must be 16, 24 or 32 bytes")),
        }

        let _nk: usize = key.len() / 4;         // 4, 6 or 8
        let rounds: usize = _nk + 6;            // Nr = Nk + 6 (10, 12, 14)
        let nb: usize = 4usize;                // AES standard block size: Nb = 4
        let total_words: usize = (rounds + 1) * nb; // number of 4-byte words in expanded key
        let expanded_bytes: usize = total_words * 4;

        let mut expanded_key: Vec<u8> = vec![0u8; expanded_bytes];
        Self::key_expansion_general(key, _nk, rounds, &mut expanded_key);

        Ok(Self { expanded_key, rounds, _nk })
    }

    // ----------------------------
    // Modes (ECB/CBC) — unchanged except expanded_key is Vec
    // ----------------------------
    pub fn encrypt_ecb(&self, plaintext: &[u8]) -> Result<Vec<u8>, AesError> {
        if plaintext.len() % 16 != 0 {
            return Err(AesError::InvalidLength(
                "Plaintext must be multiple of 16 bytes",
            ));
        }

        let mut out = Vec::with_capacity(plaintext.len());
        for chunk in plaintext.chunks_exact(16) {
            let mut block = [0u8; 16];
            block.copy_from_slice(chunk);
            out.extend_from_slice(&self.encrypt_block(&block));
        }
        Ok(out)
    }

    pub fn decrypt_ecb(&self, ciphertext: &[u8]) -> Result<Vec<u8>, AesError> {
        if ciphertext.len() % 16 != 0 {
            return Err(AesError::InvalidLength(
                "Ciphertext must be multiple of 16 bytes",
            ));
        }

        let mut out = Vec::with_capacity(ciphertext.len());
        for chunk in ciphertext.chunks_exact(16) {
            let mut block = [0u8; 16];
            block.copy_from_slice(chunk);
            out.extend_from_slice(&self.decrypt_block(&block));
        }
        Ok(out)
    }

    pub fn encrypt_cbc(&self, plaintext: &[u8], iv: &[u8; 16]) -> Result<Vec<u8>, AesError> {
        if plaintext.len() % 16 != 0 {
            return Err(AesError::InvalidLength(
                "Plaintext must be multiple of 16 bytes",
            ));
        }
        let mut ciphertext = Vec::with_capacity(plaintext.len());
        let mut prev = *iv;
        for chunk in plaintext.chunks_exact(16) {
            let mut block = [0u8; 16];
            block.copy_from_slice(chunk);
            Self::xor_in_place(&mut block, &prev);
            let enc = self.encrypt_block(&block);
            ciphertext.extend_from_slice(&enc);
            prev = enc;
        }
        Ok(ciphertext)
    }

    pub fn decrypt_cbc(&self, ciphertext: &[u8], iv: &[u8; 16]) -> Result<Vec<u8>, AesError> {
        if ciphertext.len() % 16 != 0 {
            return Err(AesError::InvalidLength(
                "Ciphertext must be multiple of 16 bytes",
            ));
        }
        let mut plaintext = Vec::with_capacity(ciphertext.len());
        let mut prev = *iv;
        for chunk in ciphertext.chunks_exact(16) {
            let mut block = [0u8; 16];
            block.copy_from_slice(chunk);
            let dec = self.decrypt_block(&block);
            let mut plain_block = dec;
            Self::xor_in_place(&mut plain_block, &prev);
            plaintext.extend_from_slice(&plain_block);
            prev = block;
        }
        Ok(plaintext)
    }

    // ----------------------------
    // AES Key Expansion (generalized for Nk=4,6,8; Nb fixed=4)
    // ----------------------------
    //
    // This implements the general algorithm:
    // W[i] = W[i-1]
    // if (i % Nk == 0) -> W[i] = SubWord(RotWord(W[i-1])) ^ Rcon[i/Nk]
    // else if (Nk > 6 && i % Nk == 4) -> W[i] = SubWord(W[i-1])
    // W[i] = W[i - Nk] ^ temp
    //
    fn key_expansion_general(key: &[u8], nk: usize, rounds: usize, expanded_key: &mut [u8]) {
        // Copy the original key bytes into the beginning of expanded_key
        expanded_key[0..key.len()].copy_from_slice(key);

        let nb = 4usize; // fixed for AES (Nb = 4)
        let total_words = (rounds + 1) * nb; // number of 4-byte words to produce
        // temp word buffer
        let mut temp = [0u8; 4];

        // start word index (words already present = Nk)
        let mut i = nk;

        while i < total_words {
            // temp = W[i-1]
            let start = (i - 1) * 4;
            temp.copy_from_slice(&expanded_key[start..start + 4]);

            if i % nk == 0 {
                // RotWord(temp)
                temp.rotate_left(1);
                // SubWord(temp)
                for b in temp.iter_mut() {
                    *b = SBOX[*b as usize];
                }
                // XOR Rcon (R_CONSTANTS indexed by i / Nk)
                // R_CONSTANTS should contain enough entries (at least rounds)
                let rcon_idx = i / nk;
                temp[0] ^= R_CONSTANTS[rcon_idx];
            } else if nk > 6 && (i % nk) == 4 {
                // AES-256 extra SubWord step
                for b in temp.iter_mut() {
                    *b = SBOX[*b as usize];
                }
            }

            // W[i] = W[i - Nk] ^ temp
            let prev_word_start = (i - nk) * 4;
            let out_start = i * 4;
            for j in 0..4 {
                expanded_key[out_start + j] = expanded_key[prev_word_start + j] ^ temp[j];
            }

            i += 1;
        }
    }

    pub fn rounds(&self) -> usize {
        self.rounds
    }

    // ----------------------------
    // AES Block Operations (assumes Nb=4 / 16-byte state)
    // ----------------------------
    fn encrypt_block(&self, input: &[u8; 16]) -> [u8; 16] {
        let mut state = *input;
        // Initial round key (round 0)
        Self::xor_in_place(&mut state, &self.expanded_key[0..16]);

        // main rounds 1 .. rounds-1
        for round in 1..self.rounds {
            Self::sub_bytes(&mut state);
            Self::shift_rows(&mut state);
            Self::mix_columns(&mut state);
            let rk_start = round * 16;
            Self::xor_in_place(&mut state, &self.expanded_key[rk_start..rk_start + 16]);
        }

        // final round
        Self::sub_bytes(&mut state);
        Self::shift_rows(&mut state);
        let final_rk_start = self.rounds * 16;
        Self::xor_in_place(&mut state, &self.expanded_key[final_rk_start..final_rk_start + 16]);

        state
    }

    fn decrypt_block(&self, input: &[u8; 16]) -> [u8; 16] {
        let mut state = *input;

        // Initial add round key (last)
        let last_rk_start = self.rounds * 16;
        Self::xor_in_place(&mut state, &self.expanded_key[last_rk_start..last_rk_start + 16]);

        for round in (1..self.rounds).rev() {
            Self::inv_shift_rows(&mut state);
            Self::inv_sub_bytes(&mut state);
            let rk_start = round * 16;
            Self::xor_in_place(&mut state, &self.expanded_key[rk_start..rk_start + 16]);
            Self::inv_mix_columns(&mut state);
        }

        // final
        Self::inv_shift_rows(&mut state);
        Self::inv_sub_bytes(&mut state);
        Self::xor_in_place(&mut state, &self.expanded_key[0..16]);

        state
    }

    // ----------------------------
    // AES Transformations (same as your previous)
    // ----------------------------
    fn xor_in_place(state: &mut [u8; 16], key_slice: &[u8]) {
        for (s, k) in state.iter_mut().zip(key_slice.iter()) {
            *s ^= k;
        }
    }

    fn sub_bytes(state: &mut [u8; 16]) {
        for byte in state.iter_mut() {
            *byte = SBOX[*byte as usize];
        }
    }

    fn shift_rows(state: &mut [u8; 16]) {
        let temp = *state;
        state[0] = temp[0];
        state[1] = temp[5];
        state[2] = temp[10];
        state[3] = temp[15];

        state[4] = temp[4];
        state[5] = temp[9];
        state[6] = temp[14];
        state[7] = temp[3];

        state[8] = temp[8];
        state[9] = temp[13];
        state[10] = temp[2];
        state[11] = temp[7];

        state[12] = temp[12];
        state[13] = temp[1];
        state[14] = temp[6];
        state[15] = temp[11];
    }

    fn mix_columns(state: &mut [u8; 16]) {
        let temp = *state;
        for c in 0..4 {
            let i = c * 4;
            state[i]   = Self::gmul(temp[i], 0x02) ^ Self::gmul(temp[i+1], 0x03) ^ temp[i+2] ^ temp[i+3];
            state[i+1] = temp[i] ^ Self::gmul(temp[i+1], 0x02) ^ Self::gmul(temp[i+2], 0x03) ^ temp[i+3];
            state[i+2] = temp[i] ^ temp[i+1] ^ Self::gmul(temp[i+2], 0x02) ^ Self::gmul(temp[i+3], 0x03);
            state[i+3] = Self::gmul(temp[i], 0x03) ^ temp[i+1] ^ temp[i+2] ^ Self::gmul(temp[i+3], 0x02);
        }
    }

    fn inv_sub_bytes(state: &mut [u8; 16]) {
        for byte in state.iter_mut() {
            *byte = INV_SBOX[*byte as usize];
        }
    }

    fn inv_shift_rows(state: &mut [u8; 16]) {
        let temp = *state;
        state[0] = temp[0];
        state[4] = temp[4];
        state[8] = temp[8];
        state[12] = temp[12];

        state[1] = temp[13];
        state[5] = temp[1];
        state[9] = temp[5];
        state[13] = temp[9];

        state[2] = temp[10];
        state[6] = temp[14];
        state[10] = temp[2];
        state[14] = temp[6];

        state[3] = temp[7];
        state[7] = temp[11];
        state[11] = temp[15];
        state[15] = temp[3];
    }

    fn inv_mix_columns(state: &mut [u8; 16]) {
        let temp = *state;
        for c in 0..4 {
            let i = c * 4;
            state[i]   = Self::gmul(temp[i], 0x0e) ^ Self::gmul(temp[i+1], 0x0b) ^ Self::gmul(temp[i+2], 0x0d) ^ Self::gmul(temp[i+3], 0x09);
            state[i+1] = Self::gmul(temp[i], 0x09) ^ Self::gmul(temp[i+1], 0x0e) ^ Self::gmul(temp[i+2], 0x0b) ^ Self::gmul(temp[i+3], 0x0d);
            state[i+2] = Self::gmul(temp[i], 0x0d) ^ Self::gmul(temp[i+1], 0x09) ^ Self::gmul(temp[i+2], 0x0e) ^ Self::gmul(temp[i+3], 0x0b);
            state[i+3] = Self::gmul(temp[i], 0x0b) ^ Self::gmul(temp[i+1], 0x0d) ^ Self::gmul(temp[i+2], 0x09) ^ Self::gmul(temp[i+3], 0x0e);
        }
    }

    // ----------------------------
    // GF(2^8) multiply
    // ----------------------------
    fn gmul(mut a: u8, mut b: u8) -> u8 {
        let mut result: u8 = 0;
        const IRREDUCIBLE_POLY: u8 = 0x1b;
        while b != 0 {
            if b & 1 != 0 {
                result ^= a;
            }
            let high_bit = a & 0x80;
            a <<= 1;
            if high_bit != 0 {
                a ^= IRREDUCIBLE_POLY;
            }
            b >>= 1;
        }
        result
    }
}



fn _test_aes() -> Result<(), Box<dyn Error>> {

    let base_state: [u8; 16] = [
        0x00, 0x11, 0x22, 0x33,
        0x44, 0x55, 0x66, 0x77,
        0x88, 0x99, 0xaa, 0xbb,
        0xcc, 0xdd, 0xee, 0xff,
    ];

    let encrypted_state: [u8; 16] = [
        0x69, 0xc4, 0xe0, 0xd8,
        0x6a, 0x7b, 0x04, 0x30,
        0xd8, 0xcd, 0xb7, 0x80,
        0x70, 0xb4, 0xc5, 0x5a,
    ];

    let encryption_key: [u8; 16] = [
        0x00, 0x01, 0x02, 0x03,
        0x04, 0x05, 0x06, 0x07,
        0x08, 0x09, 0x0a, 0x0b,
        0x0c, 0x0d, 0x0e, 0x0f,
    ];

    // AES-128 (16 bytes)
    let key128: [u8; 16] = [0x00; 16];
    let aes128: AesStandard = AesStandard::new(&key128)?;
    println!("AES-128 rounds: {}", aes128.rounds()); // 10 rounds

    // AES-192 (24 bytes)
    let key192: [u8; 24] = [0x00; 24];
    let aes192 = AesStandard::new(&key192)?;
    println!("AES-192 rounds: {}", aes192.rounds()); // 12 rounds

    // AES-256 (32 bytes)
    let key256: [u8; 32] = [0x00; 32];
    let aes256 = AesStandard::new(&key256)?;
    println!("AES-256 rounds: {}", aes256.rounds()); // 14 rounds

    // --- Use the struct here ---
    let aes: AesStandard = AesStandard::new(&encryption_key)?;

    let c: Vec<u8> = aes.encrypt_ecb(&base_state)?;
    if c == encrypted_state {
        println!("Encrypted matches ciphertext");
    } else {
        println!("Encryption mismatch!");
    }

    let decrypted: Vec<u8> = aes.decrypt_ecb(&encrypted_state)?;
    if decrypted == base_state {
        println!("Plaintext and decrypted match.");
    } else {
        println!("Decryption mismatch!");
    }
    Ok(())
}
