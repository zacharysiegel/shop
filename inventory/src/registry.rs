use crate::environment::RuntimeEnvironment;
use crate::error::ShopError;
use base64::Engine;
use reqwest::Client;
use std::env;
use std::sync::LazyLock;

pub const BASE64: base64::engine::GeneralPurpose = crypt::BASE64;

pub struct Registry {
    pub http_client: Client,
    pub ebay_base_url: &'static str,
    pub ebay_client_id: &'static str,
    /// Presented as a UTF-8-encoded string because this secret must be re-encoded with the client ID in base64 to form the basic authentication credential
    pub ebay_client_secret: String,
}

pub static REGISTRY: LazyLock<Registry> = LazyLock::new(|| {
    let environment = RuntimeEnvironment::default();

    Registry {
        http_client: Client::builder().build().unwrap(),
        ebay_base_url: match environment {
            RuntimeEnvironment::Local | RuntimeEnvironment::Stage => "https://api.sandbox.ebay.com/",
            RuntimeEnvironment::Production => "https://api.ebay.com/"
        },
        // This value pertains to my testing "zach" account. Presumably this will change to an official eBay account.
        ebay_client_id: "ZacharyS-shop-SBX-9a6e149a0-59597965",
        // This value pertains to my testing "zach" account. Presumably this will change to an official eBay account.
        ebay_client_secret: String::from_utf8(master_decrypt("ebay__zach.sandbox.cert_id").unwrap()).unwrap(),
    }
});

/// Decrypt a secret managed by the `crypt` application.
fn master_decrypt(secret_name: &'static str) -> Result<Vec<u8>, ShopError> {
    let variable_name = "MASTER_SECRET";
    let master_secret: String = match env::var(variable_name) {
        Ok(value) => value,
        Err(error) => return Err(ShopError::from_error(&format!("fetching environment variable; [{}]", variable_name), Box::new(error))),
    };
    let master_secret: Vec<u8> = match crypt::BASE64.decode(master_secret) {
        Ok(value) => value,
        Err(error) => return Err(ShopError::from_error("decoding master secret", Box::new(error))),
    };
    let secret: Vec<u8> = match crypt::cryptography::decrypt(&master_secret, secret_name) {
        Ok(value) => value,
        Err(error) => return Err(ShopError::from_error(&format!("decrypting secret; [{}]", secret_name), error)),
    };
    Ok(secret)
}
