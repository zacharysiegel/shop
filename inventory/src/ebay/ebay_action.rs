use super::{ebay_category, ebay_client};
use crate::category::Category;
use crate::error::ShopError;
use crate::inventory_location::InventoryLocation;
use crate::item::Item;
use crate::listing::{Listing, ListingStatus};
use crate::marketplace::marketplace_db;
use crate::product::Product;
use crate::{listing, ShopEntity};
use serde_json::Value;
use sqlx::PgPool;
use std::sync::OnceLock;
use uuid::Uuid;

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

pub async fn publish(
    pgpool: &PgPool,
    user_access_token: &str,
    listing: &Listing,
) -> Result<(), ShopError> {
    validate_listing(listing)?;

    let (item, product): (Item, Product) = listing::listing_action::get_item_and_product_for_listing(pgpool, listing).await?;
    log::info!("Posting listing to {}; [listing_id: {}]; [marketplace_id: {}]", MARKETPLACE_INTERNAL_NAME, listing.id, MARKETPLACE_ID.get().unwrap());

    ebay_client::create_or_replace_inventory_item(user_access_token, &item, &product).await?;

    if offer_exists(user_access_token, &item.id).await? {
        log::info!("Offer already exists; Cancelling create/publish; [{}]", item.id);
        return Ok(());
    }

    // Offers are immediately published here, so we don't bother to check if already published
    let offer_id: String = create_offer(pgpool, user_access_token, &item).await?;
    log::info!("Created ebay offer [{}]", offer_id);

    ebay_client::publish_offer(user_access_token, &offer_id).await?;
    listing::listing_action::update_listing_status(pgpool, listing, ListingStatus::Published).await?;
    log::info!("Published ebay offer [{}]", offer_id);

    Ok(())
}

pub async fn publish_all_with_status(
    pgpool: &PgPool,
    user_access_token: &str,
    status: &ListingStatus,
) -> Result<(), ShopError> {
    let listings = listing::listing_db::get_all_by_status_and_marketplace(
        pgpool,
        status,
        MARKETPLACE_ID.get()
            .ok_or_else(|| ShopError::default())?,
    )
        .await?;

    for listing in listings {
        let listing: Listing = listing.try_to_model()?;
        publish(pgpool, user_access_token, &listing).await?;
    }

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

async fn offer_exists(
    user_access_token: &str,
    item_id: &Uuid,
) -> Result<bool, ShopError> {
    let offer: Option<Value> = ebay_client::get_offers_fixed_price(user_access_token, &item_id).await?;
    let Some(offer) = offer else {
        return Ok(false);
    };

    let total: i64 = offer.get("total")
        .ok_or_else(|| ShopError::new("getting total field in get_offers response"))?
        .as_i64()
        .ok_or_else(|| ShopError::new("converting total field to i64"))?;
    Ok(total > 0)
}

async fn create_offer(
    pgpool: &PgPool,
    user_token: &str,
    item: &Item,
) -> Result<String, ShopError> {
    let categories: Vec<Category> = crate::product::product_db::get_product_categories(pgpool, &item.product_id)
        .await?
        .iter()
        .map(|entity| entity.try_to_model())
        .collect::<Result<Vec<Category>, ShopError>>()?;

    let mut ebay_categories: Vec<ebay_category::ebay_category_model::Category> = Vec::new();
    for category in &categories {
        let ebay_category = ebay_category::ebay_category_db::get_ebay_category(pgpool, &category.ebay_category_id)
            .await?
            .ok_or_else(|| ShopError::new("ebay category not found"))?
            .try_to_model()?;
        ebay_categories.push(ebay_category);
    }

    ebay_client::create_offer(user_token, item, &ebay_categories.iter().collect())
        .await
}

pub async fn sync_all_locations(pgpool: &PgPool, user_token: &str) -> Result<(), ShopError> {
    let inventory_location_vec: Vec<InventoryLocation> = crate::inventory_location::inventory_location_action::get_all_inventory_locations(pgpool).await?;

    for inventory_location in &inventory_location_vec {
        let ebay_location: Option<Value> = ebay_client::get_inventory_location(user_token, &inventory_location.id.to_string()).await?;
        if ebay_location.is_none() {
            ebay_client::create_inventory_location(user_token, &inventory_location).await?;
        } else {
            // todo: This part of eBay's service is broken. Check back in later. For now we can only create.
            // client::update_inventory_location(user_token, &inventory_location).await?;
        }
    }

    Ok(())
}
