//! Encryption and decryption functions using ChaCha20Poly1305 symmetric encryption.
//! This algorithm provides an Associated Data feature which we are not currently utilizing.

use crate::secrets::{SecretBase64, SecretBytes, SECRETS};
use chacha20poly1305::aead::{Aead, Payload};
use chacha20poly1305::{aead, AeadCore, ChaCha20Poly1305, KeyInit};
use std::error::Error;

pub fn encrypt(key: &Vec<u8>, plaintext: &[u8]) -> Result<SecretBase64, Box<dyn Error>> {
    let key: &chacha20poly1305::Key = chacha20poly1305::Key::from_slice(key);
    let nonce: chacha20poly1305::Nonce = ChaCha20Poly1305::generate_nonce(aead::OsRng);
    let cipher: ChaCha20Poly1305 = ChaCha20Poly1305::new(key);
    let payload = Payload {
        msg: plaintext,
        aad: &[], // Additional Associated Data is unused.
    };
    let ciphertext: Vec<u8> = cipher.encrypt(&nonce, payload)?;

    let secret: SecretBytes = SecretBytes {
        nonce: nonce.to_vec(),
        ciphertext,
    };
    Ok(secret.base64_encode())
}

pub fn decrypt(key: &Vec<u8>, secret_name: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let secrets = SECRETS;
    let secret: SecretBytes = secrets.get(secret_name)
        .ok_or(format!(r#"Secret "{}" does not exist"#, secret_name))?
        .base64_decode()?;

    Ok(decrypt_raw(
        chacha20poly1305::Key::from_slice(key),
        chacha20poly1305::Nonce::from_slice(secret.nonce.as_slice()),
        &secret.ciphertext,
    )?)
}

fn decrypt_raw(
    key: &chacha20poly1305::Key,
    nonce: &chacha20poly1305::Nonce,
    ciphertext: &[u8],
) -> aead::Result<Vec<u8>> {
    let cipher: ChaCha20Poly1305 = ChaCha20Poly1305::new(key);
    let payload = Payload {
        msg: ciphertext,
        aad: &[],
    };

    cipher.decrypt(nonce, payload)
}

pub fn generate_key() -> Vec<u8> {
    ChaCha20Poly1305::generate_key(aead::OsRng).to_vec()
}
