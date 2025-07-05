use super::super::ebay_category::ebay_category_model::Category;
use super::ebay_client_shared;
use crate::ebay::ebay_client::ebay_client_shared::EBAY_BASE_URL;
use crate::error::ShopError;
use crate::http;
use crate::http::{WithBearer, HTTP_CLIENT};
use crate::inventory_location::InventoryLocation;
use crate::item::Item;
use crate::item_image::ItemImage;
use crate::product::Product;
use reqwest::header::{CONTENT_LANGUAGE, CONTENT_TYPE};
use reqwest::{Request, Response};
use serde_json::{json, Value};
use std::ops::{Deref, IndexMut};
use uuid::Uuid;

const INVENTORY_API_BASE_PATH: &str = "/sell/inventory/v1";

pub async fn create_or_replace_inventory_item(
    user_access_token: &str,
    item: &Item,
    product: &Product,
    item_images: &Vec<ItemImage>,
) -> Result<(), ShopError> {
    let condition: &str = super::ebay_condition::Condition::from(&item.condition).to_serial();
    let item_image_uris: Vec<String> = item_images
        .iter()
        .map(|element| element.get_item_image_uri())
        .collect::<Vec<_>>();

    let mut body: Value = json!({
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
            "description": product.display_name,
            "upc": [],
            "imageUrls": [],
        },
        // todo: product images (required for non-catalog products)
    });
    // The upc array cannot take null values (from Option::None), so we need to dynamically insert
    if let Some(upc) = &product.upc {
        let upc_array = body.index_mut("product")
            .index_mut("upc")
            .as_array_mut()
            .ok_or_else(|| ShopError::default())?;
        upc_array.push(json!(upc))
    }
    let image_urls_array = body.index_mut("product")
        .index_mut("imageUrls")
        .as_array_mut()
        .ok_or_else(|| ShopError::default())?;
    for uri in item_image_uris {
        image_urls_array.push(Value::String(uri));
    }

    let body: String = serde_json::to_string(&body)
        .map_err(|e|
            ShopError::from_error("serializing inventory item", Box::new(e))
        )?;

    let request: Request = HTTP_CLIENT
        .put(format!("{}{}/inventory_item/{}", *EBAY_BASE_URL, INVENTORY_API_BASE_PATH, item.id))
        .header(CONTENT_LANGUAGE, ebay_client_shared::EBAY_CONTENT_LANGUAGE)
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

    let response: Response = http::execute_checked(request).await?;
    let response_body: Value = response.json().await
        .map_err(|e| ShopError::from_error("deserializing inventory location response", Box::new(e)))?;
    Ok(response_body)
}

pub async fn get_inventory_location(
    user_access_token: &str,
    inventory_location_id: &str,
) -> Result<Option<Value>, ShopError> {
    let request: Request = HTTP_CLIENT
        .get(format!("{}{}/location/{}", EBAY_BASE_URL.deref(), INVENTORY_API_BASE_PATH, inventory_location_id))
        .with_bearer(user_access_token)
        .build()
        .map_err(|e| ShopError::from_error("malformed request", Box::new(e)))?;

    let response: Option<Response> = http::execute_checked_optional(request).await?;
    let Some(response) = response else {
        return Ok(None);
    };

    let response_body: Value = response.json().await
        .map_err(|e| ShopError::from_error("deserializing inventory location response", Box::new(e)))?;
    Ok(Some(response_body))
}

fn inventory_location_body(inventory_location: &InventoryLocation) -> Result<String, ShopError> {
    let body: Value = json!({
        "location": {
            "address": {
                "addressLine1": &inventory_location.street_address,
                "city": &inventory_location.municipality,
                "country": "US", // If we ever have inventory locations outside the United States, this will require a more sophisticated conversion to ISO 3166
                "postalCode": &inventory_location.postal_area,
                "stateOrProvince": &inventory_location.district,
            }
        },
        "locationTypes": [ "WAREHOUSE" ],
        // "merchantLocationStatus": "ENABLED",
        "name": &inventory_location.display_name,
        "phone": "+10000000000", // todo: manage contact information
        "timeZoneId": &inventory_location.time_zone_id
    });
    let body: String = serde_json::to_string(&body)
        .map_err(|e|
            ShopError::from_error("serializing inventory location", Box::new(e))
        )?;
    Ok(body)
}

pub async fn create_inventory_location(
    user_access_token: &str,
    inventory_location: &InventoryLocation,
) -> Result<(), ShopError> {
    let body: String = inventory_location_body(inventory_location)?;
    let merchant_location_key: &String = &inventory_location.id.to_string();
    let request: Request = HTTP_CLIENT
        .post(format!("{}{}/location/{}", *EBAY_BASE_URL, INVENTORY_API_BASE_PATH, merchant_location_key))
        .header(CONTENT_TYPE, "application/json")
        .with_bearer(user_access_token)
        .body(body)
        .build()
        .map_err(|e| ShopError::from_error("malformed request", Box::new(e)))?;

    http::execute_checked(request).await?;
    Ok(())
}

#[allow(unused)]
pub async fn update_inventory_location(
    user_access_token: &str,
    inventory_location: &InventoryLocation,
) -> Result<(), ShopError> {
    let body: String = inventory_location_body(inventory_location)?;
    let merchant_location_key: &String = &inventory_location.id.to_string();

    let request: Request = HTTP_CLIENT
        .post(format!("{}{}/location/{}/update_location_details", EBAY_BASE_URL.deref(), INVENTORY_API_BASE_PATH, merchant_location_key))
        .header(CONTENT_TYPE, "application/json")
        .with_bearer(user_access_token)
        .body(body)
        .build()
        .map_err(|e| ShopError::from_error("malformed request", Box::new(e)))?;

    http::execute_checked(request).await?;
    Ok(())
}

#[allow(unused)]
pub async fn get_offer(
    user_access_token: &str,
    offer_id: &str,
) -> Result<Option<Value>, ShopError> {
    let request: Request = HTTP_CLIENT
        .get(format!("{}{}/offer/{}", *EBAY_BASE_URL, INVENTORY_API_BASE_PATH, offer_id))
        .with_bearer(user_access_token)
        .build()
        .map_err(|e| ShopError::from_error("malformed request", Box::new(e)))?;
    let response: Option<Response> = http::execute_checked_optional(request)
        .await?;

    match response {
        Some(response) => {
            let value: Value = response.json::<Value>()
                .await
                .map_err(|e| ShopError::from_error("deserializing offer response", Box::new(e)))?;
            Ok(Some(value))
        }
        None => Ok(None),
    }
}

pub async fn get_offers_fixed_price(
    user_access_token: &str,
    item_id: &Uuid,
) -> Result<Option<Value>, ShopError> {
    let request: Request = HTTP_CLIENT
        .get(format!(
            "{}{}/offer?marketplace_id={}&sku={}&format=FIXED_PRICE",
            *EBAY_BASE_URL,
            INVENTORY_API_BASE_PATH,
            ebay_client_shared::EBAY_MARKETPLACE_ID_US,
            item_id.to_string(),
        ))
        .with_bearer(user_access_token)
        .build()
        .map_err(|e| ShopError::from_error("malformed request", Box::new(e)))?;

    let response: Option<Response> = http::execute_checked_optional(request).await?;
    let Some(response) = response else {
        return Ok(None);
    };

    let body = response.json::<Value>()
        .await
        .map_err(|e| ShopError::from_error("deserializing get_offers body", Box::new(e)))?;
    Ok(Some(body))
}

pub async fn create_offer(
    user_access_token: &str,
    item: &Item,
    ebay_categories: &Vec<&Category>,
) -> Result<String, ShopError> {
    let category_0: &Category = *ebay_categories.get(0)
        .ok_or_else(|| ShopError::new("missing category"))?;
    let price: String = dollar_string(u64::from(item.price_cents));
    let price_div_2: String = dollar_string(u64::from(item.price_cents / 2));
    let fulfillment_policy_id: &String = super::super::ebay_action::NOMINAL_FULFILLMENT_POLICY_ID
        .get()
        .ok_or_else(|| ShopError::default())?;
    let payment_policy_id: &String = super::super::ebay_action::NOMINAL_PAYMENT_POLICY_ID
        .get()
        .ok_or_else(|| ShopError::default())?;
    let return_policy_id: &String = super::super::ebay_action::NOMINAL_RETURN_POLICY_ID
        .get()
        .ok_or_else(|| ShopError::default())?;
    let body: Value = json!({
        "categoryId": category_0.ebay_category_id,
        "format": "FIXED_PRICE",
        "hideBuyerDetails": false,
        "includeCatalogProductDetails": true,
        "listingDuration": "GTC",
        "listingPolicies": {
            "bestOfferTerms": {
                "autoDeclinePrice": {
                    "currency": "USD",
                    "value": price_div_2
                },
                "bestOfferEnabled": true
            },
            "fulfillmentPolicyId": fulfillment_policy_id,
            "paymentPolicyId": payment_policy_id,
            "returnPolicyId": return_policy_id,
        },
        "marketplaceId": ebay_client_shared::EBAY_MARKETPLACE_ID_US,
        "merchantLocationKey": item.inventory_location_id,
        "pricingSummary": {
            "price": {
                "currency": "USD",
                "value": price,
            }
        },
        "sku": item.id,
        "tax": {
            "applyTax": false
        }
    });
    let body = serde_json::to_string(&body)
        .map_err(|e| ShopError::from_error("serializing offer", Box::new(e)))?;

    let request: Request = HTTP_CLIENT
        .post(format!("{}{}/offer", EBAY_BASE_URL.deref(), INVENTORY_API_BASE_PATH))
        .header(CONTENT_TYPE, "application/json")
        .header(CONTENT_LANGUAGE, ebay_client_shared::EBAY_CONTENT_LANGUAGE)
        .with_bearer(user_access_token)
        .body(body)
        .build()
        .map_err(|e| ShopError::from_error("malformed request", Box::new(e)))?;

    let response = http::execute_checked(request).await?;
    let offer_id: String = response.json::<Value>().await
        .map_err(|e| ShopError::from_error("reading offer response", Box::new(e)))?
        .get("offerId")
        .ok_or_else(|| ShopError::new("missing offerId field"))?
        .as_str()
        .ok_or_else(|| ShopError::new("offerId field is not string"))?
        .to_string();
    Ok(offer_id)
}

fn dollar_string(cents: u64) -> String {
    format!("{}.{}", cents / 100, cents % 100)
}

pub async fn publish_offer(
    user_access_token: &str,
    offer_id: &str,
) -> Result<(), ShopError> {
    let request: Request = HTTP_CLIENT
        .post(format!("{}{}/offer/{}/publish", *EBAY_BASE_URL, INVENTORY_API_BASE_PATH, offer_id))
        // todo: refactor to use bearer_auth method
        .with_bearer(user_access_token)
        .build()
        .map_err(|e| ShopError::from_error("malformed request", Box::new(e)))?;
    http::execute_checked(request).await?;
    Ok(())
}

pub async fn withdraw_offer(
    user_access_token: &str,
    offer_id: &str,
) -> Result<(), ShopError> {
    let request: Request = HTTP_CLIENT
        .post(format!("{}{}/offer/{}/withdraw", *EBAY_BASE_URL, INVENTORY_API_BASE_PATH, offer_id))
        .bearer_auth(user_access_token)
        .build()
        .map_err(|e| ShopError::from_error_default(Box::new(e)))?;
    http::execute_checked(request).await?;
    Ok(())
}
