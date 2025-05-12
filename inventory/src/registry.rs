use reqwest::Client;
use std::sync::LazyLock;

pub struct Registry {
    pub http_client: Client,
}

pub static REGISTRY: LazyLock<Registry> = LazyLock::new(|| {
    Registry {
        http_client: Client::builder().build().unwrap(),
    }
});
