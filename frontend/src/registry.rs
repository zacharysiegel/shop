use inventory::environment::RuntimeEnvironment;
use reqwest::Client;
use std::env;
use std::sync::LazyLock;

pub struct Registry {
    pub http_client: Client,
    /// Used for server-side API requests. Does not require authentication.
    pub inventory_internal_path: String,
    /// Used for client-side API requests. Requires authentication.
    pub inventory_external_path: String,
}

pub static REGISTRY: LazyLock<Registry> = LazyLock::new(|| {
    Registry {
        http_client: create_http_client(),
        inventory_internal_path: env::var("INVENTORY_INTERNAL_URL").unwrap_or("http://127.0.0.1:11001".to_string()),
        inventory_external_path: env::var("INVENTORY_EXTERNAL_URL").unwrap_or("https://127.0.0.1:1443/api".to_string()),
    }
});

fn create_http_client() -> Client {
    let mut builder = Client::builder();

    if RuntimeEnvironment::default() == RuntimeEnvironment::Local {
        builder = builder.danger_accept_invalid_certs(true);
    }

    builder.build().unwrap()
}
