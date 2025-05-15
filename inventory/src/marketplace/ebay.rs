use crate::error::ShopError;
use crate::item::{item_db, Item, ItemEntity};
use crate::listing::{ListingModel, ListingStatus};
use crate::marketplace::marketplace_db;
use crate::product::{product_db, ProductEntity};
use crate::registry::REGISTRY;
use crate::ShopEntity;
use sqlx::PgPool;
use std::sync::OnceLock;
use uuid::Uuid;

static MARKETPLACE_ID: OnceLock<Uuid> = OnceLock::new();

const MARKETPLACE_INTERNAL_NAME: &str = "ebay";
const INVENTORY_API_BASE_PATH: &str = "https://api.ebay.com/sell/inventory/v1";


/// Should be called only once.
pub async fn init(pgpool: &PgPool) {
    let entity = marketplace_db::get_marketplace_by_internal_name(pgpool, MARKETPLACE_INTERNAL_NAME)
        .await
        .expect(&format!("Error querying database for marketplace initialization; [{}]", MARKETPLACE_INTERNAL_NAME))
        .expect(&format!("No marketplace matching the given name; [{}]", MARKETPLACE_INTERNAL_NAME));
    _ = MARKETPLACE_ID.set(entity.id);
}

/// https://developer.ebay.com/api-docs/sell/inventory/resources/inventory_item/methods/createOrReplaceInventoryItem
pub async fn publish(pgpool: &PgPool, listing: &ListingModel) -> Result<(), ShopError> {
    if listing.status != ListingStatus::Draft {
        return Err(ShopError {
            message: String::from("Invalid listing; Attempted to publish non-draft listing;")
        });
    } else if listing.marketplace_id.ne(MARKETPLACE_ID.get().unwrap()) {
        return Err(ShopError {
            message: format!("Invalid listing; Listing marketplace ID does not match \"{}\"; [{}]",
                             MARKETPLACE_INTERNAL_NAME,
                             MARKETPLACE_ID.get().unwrap()),
        })
    }

    let item: Option<ItemEntity> = match item_db::get_item(pgpool, &listing.item_id).await {
        Ok(entity) => entity,
        Err(error) => return Err(ShopError::from(error)),
    };
    let Some(item): Option<ItemEntity> = item else {
        return Err(ShopError::new(&format!("Item not found for listing; [{}]", listing.id)));
    };
    let item: Item = item.try_to_model()?;

    let product: Option<ProductEntity> = match product_db::get_product(pgpool, &item.product_id).await {
        Ok(entity) => entity,
        Err(error) => return Err(ShopError::from(error)),
    };
    let Some(product): Option<ProductEntity> = product else {
        return Err(ShopError::new(&format!("Product not found for item; [{}]", item.id)));
    };

    log::info!("Publishing listing to {}; [listing_id: {}]; [marketplace_id: {}]", MARKETPLACE_INTERNAL_NAME, listing.id, MARKETPLACE_ID.get().unwrap());

    // todo: oauth
    let _ = REGISTRY.http_client
        .put(format!("{}/inventory_item/{}", INVENTORY_API_BASE_PATH, item.id))
        .header("content-language", "en-US")
        .header("content-type", "application/json")
        .build();

    Err(ShopError::default())
}
