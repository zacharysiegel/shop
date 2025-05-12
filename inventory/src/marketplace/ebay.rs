use crate::error::ShopError;
use crate::listing::{ListingModel, ListingStatus};
use crate::marketplace::marketplace_db;
use sqlx::PgPool;
use std::sync::OnceLock;
use uuid::Uuid;

const MARKETPLACE_INTERNAL_NAME: &'static str = "ebay";
static MARKETPLACE_ID: OnceLock<Uuid> = OnceLock::new();

/// Should be called only once.
pub async fn init(pgpool: &PgPool) {
    let entity = marketplace_db::get_marketplace_by_internal_name(pgpool, MARKETPLACE_INTERNAL_NAME)
        .await
        .expect(&format!("Error querying database for marketplace initialization; [{}]", MARKETPLACE_INTERNAL_NAME))
        .expect(&format!("No marketplace matching the given name; [{}]", MARKETPLACE_INTERNAL_NAME));
    _ = MARKETPLACE_ID.set(entity.id);
}

pub fn publish(pgpool: &PgPool, listing: &ListingModel) -> Result<(), ShopError> {
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

    log::info!("Publishing listing to {}; [listing_id: {}]; [marketplace_id: {}]", MARKETPLACE_INTERNAL_NAME, listing.id, MARKETPLACE_ID.get().unwrap());

    Err(ShopError::default())
}