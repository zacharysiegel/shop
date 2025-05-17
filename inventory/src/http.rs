use reqwest::Client;
use std::sync::LazyLock;

pub const BASE64: base64::engine::GeneralPurpose = crypt::BASE64;

pub static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| Client::builder().build().unwrap());

// todo: add global client.execute function.
