use inventory::error::ShopError;
use reqwest::Client;
use std::cmp::PartialEq;
use std::env;
use std::sync::LazyLock;

pub struct Registry {
    pub http_client: Client,
    pub remote_url: String,
}

pub static REGISTRY: LazyLock<Registry> = LazyLock::new(|| {
    Registry {
        http_client: create_http_client(),
        remote_url: env::var("REMOTE_URL").unwrap_or("https://localhost:1443/api".to_string()),
    }
});

#[derive(PartialEq)]
pub enum RuntimeEnvironment {
    Local = 0,
    Stage,
    Production,
}

impl RuntimeEnvironment {
    pub fn from_env() -> Result<RuntimeEnvironment, ShopError> {
        RuntimeEnvironment::try_from(env::var("RUNTIME_ENVIRONMENT").unwrap_or(String::new()))
    }
}

impl Default for RuntimeEnvironment {
    fn default() -> RuntimeEnvironment {
        RuntimeEnvironment::from_env().unwrap_or(RuntimeEnvironment::Local)
    }
}

impl TryFrom<String> for RuntimeEnvironment {
    type Error = ShopError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "local" => Ok(Self::Local),
            "stage" => Ok(Self::Stage),
            "production" => Ok(Self::Production),
            _ => Err(ShopError {
                message: format!("Error parsing runtime environment [{}]", value),
            })
        }
    }
}

fn create_http_client() -> Client {
    let mut builder = Client::builder();

    if RuntimeEnvironment::default() == RuntimeEnvironment::Local {
        builder = builder.danger_accept_invalid_certs(true);
    }

    builder.build().unwrap()
}
