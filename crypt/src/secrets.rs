use base64::Engine;
use chacha20poly1305::aead::{Aead, Payload};
use chacha20poly1305::{aead, AeadCore, ChaCha20Poly1305, KeyInit};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::LazyLock;

pub const BASE64: base64::engine::general_purpose::GeneralPurpose = base64::engine::general_purpose::STANDARD;
pub const SECRETS: LazyLock<BTreeMap<&'static str, Secret>> = LazyLock::new(|| {
    let mut map: BTreeMap<&'static str, Secret> = BTreeMap::new();
    map.insert("ebay_cert_id_zach", Secret {
        key_base64: String::default(),
        nonce_base64: String::default(),
        ciphertext_base64: String::default(),
    });
    map
});

#[derive(Debug)]
pub struct Secret {
    key_base64: String,
    nonce_base64: String,
    ciphertext_base64: String,
}

impl Display for Secret {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

pub fn encrypt(plaintext: &[u8]) -> Result<Secret, Box<dyn Error>> {
    let key: chacha20poly1305::Key = ChaCha20Poly1305::generate_key(aead::OsRng);
    let nonce: chacha20poly1305::Nonce = ChaCha20Poly1305::generate_nonce(aead::OsRng);
    let cipher: ChaCha20Poly1305 = ChaCha20Poly1305::new(&key);
    let payload = Payload {
        msg: plaintext,
        aad: &[],
    };
    let ciphertext = cipher.encrypt(&nonce, payload)?;

    let key_base64 = BASE64.encode(&key);
    let nonce_base64 = BASE64.encode(&nonce);
    let ciphertext_base64 = BASE64.encode(&ciphertext);
    Ok(Secret {
        key_base64,
        nonce_base64,
        ciphertext_base64,
    })
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
