use crate::registry::REGISTRY;
use maud::{html, Markup};
use serde::de::DeserializeOwned;

pub async fn wrapped_get<SerialT: DeserializeOwned>(path_and_query: &str) -> Result<SerialT, Markup> {
    let result = REGISTRY.http_client
        .get(format!("{}{}", REGISTRY.remote_url, path_and_query))
        .send()
        .await;
    let response = match result {
        Ok(response) => response,
        Err(error) => {
            return Err(html!((format!("Error: {:#}", error))));
        }
    };
    let serial = match response.json::<SerialT>().await {
        Ok(element) => element,
        Err(error) => {
            return Err(html!((format!("Error: {:#}", error))));
        }
    };
    Ok(serial)
}
