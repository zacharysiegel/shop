use crate::error::ShopError;
use crate::item::Item;
use crate::listing::{listing_action, Listing, ListingStatus};
use crate::marketplace::marketplace_db;
use crate::product::Product;
use sqlx::PgPool;
use std::sync::OnceLock;
use serde_json::Value;
use uuid::Uuid;
use crate::inventory_location::{InventoryLocation, InventoryLocationEntity};
use super::client;

pub static NOMINAL_FULFILLMENT_POLICY_ID: OnceLock<String> = OnceLock::new();
pub static NOMINAL_PAYMENT_POLICY_ID: OnceLock<String> = OnceLock::new();
pub static NOMINAL_RETURN_POLICY_ID: OnceLock<String> = OnceLock::new();

static MARKETPLACE_ID: OnceLock<Uuid> = OnceLock::new();

const MARKETPLACE_INTERNAL_NAME: &str = "ebay";

/// Should be called only once.
pub async fn init(pgpool: &PgPool) {
    let entity = marketplace_db::get_marketplace_by_internal_name(pgpool, MARKETPLACE_INTERNAL_NAME)
        .await
        // Panicking on application initialization is fine
        .expect(&format!("Error querying database for marketplace initialization; [{}]", MARKETPLACE_INTERNAL_NAME))
        .expect(&format!("No marketplace matching the given name; [{}]", MARKETPLACE_INTERNAL_NAME));
    MARKETPLACE_ID.set(entity.id).ok();
    // todo: use the Account v1 API to fetch live object IDs. (these are sandbox values)
    NOMINAL_FULFILLMENT_POLICY_ID.set("6209442000".to_string()).ok();
    NOMINAL_PAYMENT_POLICY_ID.set("6209443000".to_string()).ok();
    NOMINAL_RETURN_POLICY_ID.set("6209449000".to_string()).ok();
}

/// https://developer.ebay.com/api-docs/sell/inventory/resources/inventory_item/methods/createOrReplaceInventoryItem
pub async fn post(pgpool: &PgPool, user_access_token: &str, listing: &Listing) -> Result<(), ShopError> {
    validate_listing(listing)?;

    let (item, product): (Item, Product) = listing_action::get_item_and_product_for_listing(pgpool, listing).await?;
    log::info!("Posting listing to {}; [listing_id: {}]; [marketplace_id: {}]", MARKETPLACE_INTERNAL_NAME, listing.id, MARKETPLACE_ID.get().unwrap());

    client::create_or_replace_inventory_item(user_access_token, &item, &product).await
}

pub async fn publish(pgpool: &PgPool, listing: &Listing) -> Result<(), ShopError> {
    validate_listing(listing)?;

    let (item, product): (Item, Product) = listing_action::get_item_and_product_for_listing(pgpool, listing).await?;
    log::info!("Publishing listing to {}; [listing_id: {}]; [marketplace_id: {}]", MARKETPLACE_INTERNAL_NAME, listing.id, MARKETPLACE_ID.get().unwrap());

    // todo: publish offer
    Ok(())
}

fn validate_listing(listing: &Listing) -> Result<(), ShopError> {
    if listing.status != ListingStatus::Draft {
        return Err(ShopError::new("Invalid listing; Attempted to publish non-draft listing;"));
    } else if listing.marketplace_id.ne(MARKETPLACE_ID.get().unwrap()) {
        return Err(ShopError::new(&format!(
            "Invalid listing; Listing marketplace ID does not match \"{}\"; [{}]",
            MARKETPLACE_INTERNAL_NAME,
            MARKETPLACE_ID.get()
                .ok_or_else(|| ShopError::default())?,
        )))
    }
    Ok(())
}

pub async fn sync_all_locations(pgpool: &PgPool, user_token: &str) -> Result<(), ShopError> {
    let inventory_location_vec: Vec<InventoryLocation> = crate::inventory_location::inventory_location_action::get_all_inventory_locations(pgpool).await?;

    for inventory_location in &inventory_location_vec {
        let ebay_location: Option<Value> = client::get_inventory_location(user_token, &inventory_location.id.to_string()).await?;
        if (ebay_location.is_none()) {
            client::create_inventory_location(user_token, &inventory_location).await?;
        } else {
            // todo: This part of eBay's service is broken. Check back in later. For now we can only create.
            // client::update_inventory_location(user_token, &inventory_location).await?;
        }
    }

    Ok(())
}
