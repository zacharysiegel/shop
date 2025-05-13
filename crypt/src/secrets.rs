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
        key_base64: String::from("redacted"),
        nonce_base64: String::from("Q97zM4h1mSBpR04u"),
        ciphertext_base64: String::from("iichSrBjy7Rm03whyX5K89jFfPrOrIin4+DKUmIcAcPnQgGO+pdwFGKGl8mf7HhNNrihQA=="),
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

pub fn decrypt(secret_name: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let secrets = SECRETS;
    let secret: &Secret = secrets.get(secret_name)
        .ok_or(format!(r#"Secret "{}" does not exist"#, secret_name))?;

    let key = BASE64.decode(&secret.key_base64)?;
    let nonce = BASE64.decode(&secret.nonce_base64)?;
    let ciphertext = BASE64.decode(&secret.ciphertext_base64)?;

    Ok(decrypt_raw(
        chacha20poly1305::Key::from_slice(key.as_slice()),
        chacha20poly1305::Nonce::from_slice(nonce.as_slice()),
        &ciphertext,
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

pub fn list_secrets() -> Vec<String> {
    SECRETS.keys()
        .into_iter()
        .map(|key| key.to_string())
        .collect::<Vec<String>>()
}
