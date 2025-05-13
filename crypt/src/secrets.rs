use base64::engine::DecodePaddingMode;
use base64::Engine;
use std::collections::BTreeMap;
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
pub struct Secret<T> {
    pub key: T,
    pub nonce: T,
    pub ciphertext: T,
}

pub type SecretBase64 = Secret<String>;
pub type SecretBytes = Secret<Vec<u8>>;

impl SecretBase64 {
    pub fn base64_decode(&self) -> Result<SecretBytes, base64::DecodeError> {
        Ok(SecretBytes {
            key: BASE64.decode(&self.key)?,
            nonce: BASE64.decode(&self.nonce)?,
            ciphertext: BASE64.decode(&self.ciphertext)?,
        })
    }
}

impl SecretBytes {
    pub fn base64_encode(&self) -> SecretBase64 {
        SecretBase64 {
            key: BASE64.encode(&self.key),
            nonce: BASE64.encode(&self.nonce),
            ciphertext: BASE64.encode(&self.ciphertext),
        }
    }
}

impl Display for SecretBase64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut display: String = String::from("SecretBase64 {\n");
        display.push_str(&format!("    key: String::from(\"{}\"),\n", self.key));
        display.push_str(&format!("    nonce: String::from(\"{}\"),\n", self.nonce));
        display.push_str(&format!("    ciphertext: String::from(\"{}\"),\n", self.ciphertext));
        display.push_str("}");
        write!(f, "{}", display)
    }
}

pub fn list_secret_names() -> Vec<String> {
    SECRETS.keys()
        .into_iter()
        .map(|key| key.to_string())
        .collect::<Vec<String>>()
}
