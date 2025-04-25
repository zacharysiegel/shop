use std::env;
use std::sync::LazyLock;
use reqwest::Client;

pub struct Registry {
    pub http_client: Client,
    pub remote_url: String,
}

pub static REGISTRY: LazyLock<Registry>  = LazyLock::new(|| {
    Registry {
        http_client: Client::default(),
        remote_url: env::var("REMOTE_URL").unwrap_or("http://localhost:11001".to_string()),
    }
});
