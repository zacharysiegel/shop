use crate::error::ShopError;
use base64::Engine;
use std::env;

/// Decrypt a secret managed by the `crypt` application.
pub fn master_decrypt(secret_name: &str) -> Result<Vec<u8>, ShopError> {
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
