use crate::error::ShopError;
use crate::item::Item;
use crate::listing::{listing_action, Listing, ListingStatus};
use crate::marketplace::marketplace_db;
use crate::product::Product;
use sqlx::PgPool;
use std::sync::OnceLock;
use uuid::Uuid;
use crate::marketplace::ebay::ebay_header::XEbayAuthorization;

static MARKETPLACE_ID: OnceLock<Uuid> = OnceLock::new();

const MARKETPLACE_INTERNAL_NAME: &str = "ebay";

/// Should be called only once.
pub async fn init(pgpool: &PgPool) {
    let entity = marketplace_db::get_marketplace_by_internal_name(pgpool, MARKETPLACE_INTERNAL_NAME)
        .await
        // Panicking on application initialization is fine
        .expect(&format!("Error querying database for marketplace initialization; [{}]", MARKETPLACE_INTERNAL_NAME))
        .expect(&format!("No marketplace matching the given name; [{}]", MARKETPLACE_INTERNAL_NAME));
    _ = MARKETPLACE_ID.set(entity.id);
}

pub async fn post(ebay_auth: XEbayAuthorization, pgpool: &PgPool, listing: &Listing) -> Result<(), ShopError> {
    validate_listing(listing)?;

    let (item, product): (Item, Product) = listing_action::get_item_and_product_for_listing(pgpool, listing).await?;
    log::info!("Posting listing to {}; [listing_id: {}]; [marketplace_id: {}]", MARKETPLACE_INTERNAL_NAME, listing.id, MARKETPLACE_ID.get().unwrap());

    super::client::create_or_replace_inventory_item(listing, &item, &product).await
}

/// https://developer.ebay.com/api-docs/sell/inventory/resources/inventory_item/methods/createOrReplaceInventoryItem
pub async fn publish(pgpool: &PgPool, listing: &Listing) -> Result<(), ShopError> {
    validate_listing(listing)?;

    let (item, product): (Item, Product) = listing_action::get_item_and_product_for_listing(pgpool, listing).await?;
    log::info!("Publishing listing to {}; [listing_id: {}]; [marketplace_id: {}]", MARKETPLACE_INTERNAL_NAME, listing.id, MARKETPLACE_ID.get().unwrap());

    super::client::publish_listing(listing, &item, &product).await
}

fn validate_listing(listing: &Listing) -> Result<(), ShopError> {
    if listing.status != ListingStatus::Draft {
        return Err(ShopError::new("Invalid listing; Attempted to publish non-draft listing;"));
    } else if listing.marketplace_id.ne(MARKETPLACE_ID.get().unwrap()) {
        return Err(ShopError::new(&format!(
            "Invalid listing; Listing marketplace ID does not match \"{}\"; [{}]",
            MARKETPLACE_INTERNAL_NAME,
            MARKETPLACE_ID.get().unwrap(),
        )))
    }
    Ok(())
}
