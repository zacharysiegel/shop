use chacha20poly1305::aead::{Aead, Payload};
use chacha20poly1305::{aead, AeadCore, ChaCha20Poly1305, KeyInit};
use std::collections::BTreeMap;
use std::sync::LazyLock;

pub const SECRETS: LazyLock<BTreeMap<&'static str, Secret>> = LazyLock::new(|| {
    let mut map: BTreeMap<&'static str, Secret> = BTreeMap::new();
    map.insert("ebay_cert_id_zach", Secret {
        key: chacha20poly1305::Key::default(),
        nonce: chacha20poly1305::Nonce::default(),
        ciphertext: b"",
    });
    map
});

pub struct Secret {
    key: chacha20poly1305::Key,
    nonce: chacha20poly1305::Nonce,
    ciphertext: &'static [u8],
}

pub fn encrypt(plaintext: &[u8]) -> (
    chacha20poly1305::Key,
    chacha20poly1305::Nonce,
    aead::Result<Vec<u8>>,
) {
    let key: chacha20poly1305::Key = ChaCha20Poly1305::generate_key(aead::OsRng);
    let nonce: chacha20poly1305::Nonce = ChaCha20Poly1305::generate_nonce(aead::OsRng);
    let cipher: ChaCha20Poly1305 = ChaCha20Poly1305::new(&key);
    let payload = Payload {
        msg: plaintext,
        aad: &[],
    };

    let ciphertext = cipher.encrypt(&nonce, payload);
    (key, nonce, ciphertext)
}

pub fn decrypt(
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

pub fn list_secrets() -> Vec<String> {
    SECRETS.keys()
        .into_iter()
        .map(|key| key.to_string())
        .collect::<Vec<String>>()
}
