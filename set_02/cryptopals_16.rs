/* Cryptopals - Set 2, Challenge 16
Solution Started: September 22, 2025
Puzzle Link: https://cryptopals.com/sets/2/challenges/16
Solution by: Abbas Moosajee
Brief: [CBC bitflipping attacks] */

use cryptopals::{pkcs7_unpadding, AesStandard, pkcs7_padding, gen_key};

/// Build and encrypt param string (mimics Python version)
fn encrypt_params(userdata: &str, enc_key: &[u8], iv_box: &[u8; 16]) -> Vec<u8> {
    // sanitize userdata
    let safe: String = userdata.replace(';', "%3B").replace('=', "%3D");

    let prefix: &'static [u8; 32] = b"comment1=cooking%20MCs;userdata=";
    let suffix: &'static [u8; 42] = b";comment2=%20like%20a%20pound%20of%20bacon";

    let mut params: Vec<u8> = Vec::new();
    params.extend_from_slice(prefix);
    params.extend_from_slice(safe.as_bytes());
    params.extend_from_slice(suffix);

    // pad and encrypt with your AesStandard::encrypt_cbc
    let padded: Vec<u8> = pkcs7_padding(&params, 16);

    let aes: AesStandard = AesStandard::new(enc_key).expect("Failed to create AES");
    aes.encrypt_cbc(&padded, iv_box).expect("encrypt_cbc failed")
}

/// Decrypt and check for `;admin=true;` using your decrypt_cbc + unpadding
fn decrypt_params_and_check_admin(ciphertext: &[u8], enc_key:&[u8], iv_box: &[u8; 16]) -> bool {
    let aes: AesStandard = AesStandard::new(enc_key).expect("Failed to create AES");
    let decrypted: Vec<u8> = aes.decrypt_cbc(ciphertext, iv_box).expect("decrypt_cbc failed");

    // unpad (returns Result<Vec<u8>, &str>)
    match pkcs7_unpadding(&decrypted) {
        Ok(plain) => plain.windows(b";admin=true;".len()).any(|w| w == b";admin=true;"),
        Err(_) => false,
    }
}
fn main() {
    println!("Set 02, Challenge 16: CBC bitflipping attack");
    let rand_key = gen_key(16);
    let iv_box: [u8; 16] = [0u8; 16];

    // Craft controlled input
    let mut ct = encrypt_params("XXXXXXXXXXXXXXXX:admin<true:XXXX", &rand_key, &iv_box);

    // Flip bits in ciphertext to turn ":admin<true:" into ";admin=true;"
    ct[32] ^= 1;
    ct[38] ^= 1;
    ct[43] ^= 1;
    println!("{:?}", ct);
    let is_admin = decrypt_params_and_check_admin(&ct, &rand_key, &iv_box);
    println!("Is admin? {}", is_admin);
}
