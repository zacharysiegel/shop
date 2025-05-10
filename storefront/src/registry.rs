use inventory::environment::RuntimeEnvironment;
use reqwest::Client;
use std::env;
use std::sync::LazyLock;

pub struct Registry {
    pub http_client: Client,
    pub remote_url: String,
}

pub static REGISTRY: LazyLock<Registry> = LazyLock::new(|| {
    Registry {
        http_client: create_http_client(),
        remote_url: env::var("REMOTE_URL").unwrap_or("https://127.0.0.1:1443/api".to_string()),
    }
});

fn create_http_client() -> Client {
    let mut builder = Client::builder();

    if RuntimeEnvironment::default() == RuntimeEnvironment::Local {
        builder = builder.danger_accept_invalid_certs(true);
    }

    builder.build().unwrap()
}
