use crate::environment::RuntimeEnvironment;
use crate::error::ShopError;
use reqwest::{Client, Request, RequestBuilder, Response, StatusCode};
use std::ops::Deref;
use std::sync::LazyLock;

pub const BASE64: base64::engine::GeneralPurpose = crypt::BASE64;

pub static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| Client::builder().build().unwrap());
pub static DOMAIN: LazyLock<&'static str> = LazyLock::new(||
    match RuntimeEnvironment::default() {
        RuntimeEnvironment::Local => "127.0.0.1",
        RuntimeEnvironment::Stage => "todo",
        RuntimeEnvironment::Production => "todo",
    }
);

/// Standard wrapper for the reqwest::Client::execute method.
/// Converts I/O errors to standard ShopError structs.
/// Converts error responses (4xx/5xx) to ShopError structs.
pub async fn execute_checked(request: Request) -> Result<Response, ShopError> {
    log::debug!("{:#?}", request);

    let response: Response = HTTP_CLIENT
        .execute(request)
        .await
        .map_err(|error| ShopError::from_error("request failed", Box::new(error)))?;

    if response.status().is_client_error() || response.status().is_server_error() {
        let status: StatusCode = response.status();
        let text: String = response.text().await
            .map_err(|error| ShopError::from_error("reading http response", Box::new(error)))?;
        return Err(ShopError::new(&format!("http error; [{}]; [{}];", status, text)));
    }

    Ok(response)
}

pub fn header_set_cookie_secure(name: &str, token: &str, lifetime: u64) -> (&'static str, String) {
    (
        "Set-Cookie",
        format!(
            "{}={}; Domain={}; HttpOnly; Max-Age={}; Path=/api/ebay; Secure; SameSite=Strict; Partitioned;",
            name,
            token,
            DOMAIN.deref(),
            lifetime,
        )
    )
}

pub trait WithBearer {
    fn with_bearer(self, token: &str) -> Self;
}

impl WithBearer for RequestBuilder {
    fn with_bearer(self, token: &str) -> Self {
        self.header("Authorization", format!("Bearer {}", token))
    }
}
