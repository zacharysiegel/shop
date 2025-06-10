use crate::admin::structure::error_text::error_markup;
use crate::registry::REGISTRY;
use maud::Markup;
use serde::de::DeserializeOwned;

pub async fn wrapped_get<SerialT: DeserializeOwned>(path_and_query: &str) -> Result<SerialT, Markup> {
    let result = REGISTRY.http_client
        .get(format!("{}{}", REGISTRY.inventory_internal_path, path_and_query))
        .send()
        .await;
    let response = match result {
        Ok(response) => response,
        Err(error) => {
            log::error!("{:#}", error);
            return Err(error_markup(error));
        }
    };
    let text: String = match response.text().await {
        Ok(value) => value,
        Err(error) => {
            log::error!("{:#}", error);
            return Err(error_markup(error));
        }
    };
    let json = match serde_json::from_str(text.as_str()) {
        Ok(value) => value,
        Err(error) => {
            log::error!("{:#}; [body: \"{}\"]", error, text);
            return Err(error_markup(error));
        }
    };
    Ok(json)
}
