use crate::error::ShopError;
use crate::http;
use crate::http::{WithBearer, HTTP_CLIENT};
use crate::item::Item;
use crate::marketplace::ebay::client::ebay_client_shared::EBAY_BASE_URL;
use crate::product::Product;
use reqwest::header::{AUTHORIZATION, CONTENT_LANGUAGE, CONTENT_TYPE};
use reqwest::{Request, Response};
use serde_json::{json, Value};

const INVENTORY_API_BASE_PATH: &str = "/sell/inventory/v1";

pub async fn create_or_replace_inventory_item(
    user_access_token: &str,
    item: &Item,
    product: &Product,
) -> Result<(), ShopError> {
    let condition: &str = super::ebay_condition::Condition::from(&item.condition).to_serial();
    let body: serde_json::Value = json!({
        "availability": {
            "shipToLocationAvailability": {
                "availabilityDistributions": [
                    {
                        "merchantLocationKey": item.inventory_location_id,
                        "quantity": 1,
                    }
                ],
                "quantity": 1,
            }
        },
        "condition": condition,
        "product": {
            "title": product.display_name,
            "upc": [ product.upc ],
        },
        // todo: product images (required for non-catalog products)
    });
    let body: String = serde_json::to_string(&body)
        .map_err(|e|
            ShopError::from_error("serializing inventory item", Box::new(e))
        )?;

    let request: Request = http::HTTP_CLIENT
        .put(format!("{}{}/inventory_item/{}", *EBAY_BASE_URL, INVENTORY_API_BASE_PATH, item.id))
        .header(CONTENT_LANGUAGE, super::ebay_client_shared::EBAY_CONTENT_LANGUAGE)
        .header(CONTENT_TYPE, "application/json")
        .with_bearer(user_access_token)
        .body(body)
        .build()
        .map_err(|e| ShopError::from_error("malformed request", Box::new(e)))?;

    http::execute_checked(request).await?;
    Ok(())
}

pub async fn get_inventory_item(
    user_access_token: &str,
    item_id: &str,
) -> Result<Value, ShopError> {
    let request: Request = HTTP_CLIENT
        .get(format!("{}{}/inventory_item/{}", *EBAY_BASE_URL, INVENTORY_API_BASE_PATH, item_id))
        .with_bearer(user_access_token)
        .build()
        .map_err(|e| ShopError::from_error("malformed request", Box::new(e)))?;

    let response: Response = http::execute_checked(request).await?;
    let response_body: Value = response.json()
        .await
        .map_err(|e| ShopError::from_error("deserializing inventory item response", Box::new(e)))?;
    Ok(response_body)
}

pub async fn get_all_inventory_locations(
    user_access_token: &str,
) -> Result<Value, ShopError> {
    let request: Request = HTTP_CLIENT
        .get(format!("{}{}/location", *EBAY_BASE_URL, INVENTORY_API_BASE_PATH))
        .with_bearer(user_access_token)
        .build()
        .map_err(|e| ShopError::from_error("malformed request", Box::new(e)))?;

    ""
}

