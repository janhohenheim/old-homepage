extern crate crypto as rust_crypto;

use self::rust_crypto::scrypt::{scrypt_simple, scrypt_check, ScryptParams};

pub fn encrypt(val: &str) -> String {
    let params = ScryptParams::new(14, 8, 1);
    scrypt_simple(val, &params).unwrap()
}

pub fn check(val: &str, hashed_val: &str) -> bool {
    scrypt_check(val, hashed_val).unwrap()
}
