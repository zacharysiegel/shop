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
            return Err(error_markup(error));
        }
    };
    let serial = match response.json::<SerialT>().await {
        Ok(element) => element,
        Err(error) => {
            return Err(error_markup(error));
        }
    };
    Ok(serial)
}
