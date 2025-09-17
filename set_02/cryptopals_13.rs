/* Cryptopals - Set 2, Challenge 13
Solution Started: September 13, 2025
Puzzle Link: https://cryptopals.com/sets/2/challenges/13
Solution by: Abbas Moosajee
Brief: [ECB cut-and-paste]
*/

use std::collections::HashMap;
use std::error::Error;

use cryptopals::{gen_key, pkcs7_padding, pkcs7_unpadding, AesStandard};

// --- helper: sanitize email ---
fn sanitize(input: &[u8]) -> Vec<u8> {
    input.iter().copied().filter(|&b| b != b'&' && b != b'=').collect()
}

// --- helper: encode key-value pairs ---
fn encode_profile(fields: &[(&[u8], &[u8])]) -> Vec<u8> {
    let mut out = Vec::new();
    for (i, (k, v)) in fields.iter().enumerate() {
        out.extend_from_slice(k);
        out.push(b'=');
        out.extend_from_slice(v);
        if i != fields.len() - 1 {
            out.push(b'&');
        }
    }
    out
}

// --- helper: parse key-value pairs ---
fn build_cookie_object(input: &[u8]) -> HashMap<String, String> {
    let s = String::from_utf8_lossy(input);
    let mut map = HashMap::new();
    for pair in s.split('&') {
        let mut parts = pair.splitn(2, '=');
        let k = parts.next().unwrap_or("").to_string();
        let v = parts.next().unwrap_or("").to_string();
        map.insert(k, v);
    }
    map
}

// --- profile_for ---
fn profile_for(email: &[u8], uid: &[u8], role: &[u8]) -> Vec<u8> {
    let clean_email = sanitize(email);
    let fields = [
        (b"email".as_ref(), clean_email.as_slice()),
        (b"uid".as_ref(), uid),
        (b"role".as_ref(), role),
    ];
    encode_profile(&fields)
}

// --- encryption ---
fn encrypt_profile(email: &[u8], rand_key: &[u8]) -> Vec<u8> {
    let encoded_profile: Vec<u8> = profile_for(email, b"10", b"user");
    let padded_profile: Vec<u8> = pkcs7_padding(&encoded_profile, 16);

    let cipher = AesStandard::new(rand_key).unwrap();
    cipher.encrypt_ecb(&padded_profile).unwrap()
}

// --- decryption ---
fn decrypt_profile(ciphertext: &[u8], rand_key: &[u8]) -> Vec<u8> {
    let cipher = AesStandard::new(rand_key).unwrap();
    let decrypted = cipher.decrypt_ecb(ciphertext).unwrap();
    pkcs7_unpadding(&decrypted).unwrap()
}


// --- perform the ECB cut-and-paste attack ---
fn do_evil(rand_key: &[u8]) -> Vec<u8> {
    // Blocksize = 16
    // We want to make "admin" its own block with correct PKCS#7 padding

    let normal = encrypt_profile(b"foooo@bar.com", rand_key);


    let mut crafted = b"foooo@bar.".to_vec();
    let mut admin_block = b"admin".to_vec();
    // pad admin to 16
    let pad_len = 16 - (admin_block.len() % 16);
    admin_block.extend(std::iter::repeat(pad_len as u8).take(pad_len));
    crafted.extend_from_slice(&admin_block);
    crafted.extend_from_slice(b"com");

    let crafted_ct = encrypt_profile(&crafted, rand_key);

    // Step 3: forge ciphertext
    // First two blocks (email+uid) from normal, then 'admin' block from crafted
    let mut forged = Vec::new();
    forged.extend_from_slice(&normal[0..32]);        // first two blocks
    forged.extend_from_slice(&crafted_ct[16..32]);   // 'admin' block
    forged
}

// --- main demo ---
fn main() -> Result<(), Box<dyn Error>> {
    println!("Set 02, Challenge 13: ECB cut-and-paste");

    let rand_key: Vec<u8> = gen_key(16);

    // Normal encode/decode
    let test_email = b"foo@bar.com";
    let encrypted = encrypt_profile(test_email, &rand_key);
    let decrypted = decrypt_profile(&encrypted, &rand_key);
    println!("Decrypted normal: {}", String::from_utf8_lossy(&decrypted));

    let parsed = build_cookie_object(&decrypted);
    println!("Parsed normal: {:?}", parsed);

    // Do the attack
    let evil_ct = do_evil(&rand_key);
    let evil_pt = decrypt_profile(&evil_ct, &rand_key);
    println!("Decrypted forged: {}", String::from_utf8_lossy(&evil_pt));
    let parsed_evil = build_cookie_object(&evil_pt);
    println!("Parsed forged: {:?}", parsed_evil);

    Ok(())
}
