use reqwest::Client;
use std::sync::LazyLock;

pub const BASE64: base64::engine::GeneralPurpose = crypt::BASE64;

pub struct Registry {
    pub http_client: Client,
}

pub static REGISTRY: LazyLock<Registry> = LazyLock::new(|| {
    Registry {
        http_client: Client::builder().build().unwrap(),
    }
});

// todo: rename module "http". add global client.execute function.