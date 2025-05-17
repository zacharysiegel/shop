use crate::error::ShopError;
use reqwest::{Client, Request, Response, StatusCode};
use std::sync::LazyLock;

pub const BASE64: base64::engine::GeneralPurpose = crypt::BASE64;

pub static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| Client::builder().build().unwrap());

// todo: add global client.execute function.

/// Standard wrapper for the reqwest::Client::execute method.
/// Converts I/O errors to standard ShopError structs.
/// Converts error responses (4xx/5xx) to ShopError structs.
pub async fn execute_checked(request: Request) -> Result<Response, ShopError> {
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
