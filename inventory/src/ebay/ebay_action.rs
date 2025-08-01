use super::{ebay_category, ebay_client};
use crate::category::Category;
use crate::error::ShopError;
use crate::inventory_location::InventoryLocation;
use crate::item::Item;
use crate::item_image::{item_image_db, ItemImage};
use crate::listing::{Listing, ListingStatus};
use crate::marketplace::marketplace_db;
use crate::product::Product;
use crate::{listing, ShopEntity};
use listing::listing_action;
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
    validate_listing_marketplace(listing)?;
    if !(listing.status == ListingStatus::Draft || listing.status == ListingStatus::Cancelled) {
        return Err(ShopError::new("listing is not draft"));
    }

    let (item, product): (Item, Product) = listing_action::get_item_and_product_for_listing(pgpool, listing).await?;
    let item_images: Vec<ItemImage> = item.get_all_item_images(pgpool).await?;

    log::info!("Posting listing to {}; [listing_id: {}]; [marketplace_id: {}]", MARKETPLACE_INTERNAL_NAME, listing.id, MARKETPLACE_ID.get().unwrap());

    ebay_client::create_or_replace_inventory_item(user_access_token, &item, &product, &item_images).await?;

    let mut offer: Option<Value> = get_offer(user_access_token, &item.id).await?;
    let offer_id: String;
    if let Some(offer) = &offer {
        log::info!("Offer already exists; Skipping creation; Attempting to publish; [{}]", item.id);
        offer_id = offer["offerId"]
            .as_str()
            .ok_or_else(|| ShopError::default())?
            .to_string();
    } else {
        // Offers are immediately published here, so we don't bother to check if already published
        offer_id = create_offer(pgpool, user_access_token, &item).await?
            .to_string();
        log::info!("Created ebay offer [{}]", offer_id);

        offer = get_offer(user_access_token, &item.id).await?;
    }
    let offer: Value = offer.ok_or_else(|| ShopError::default())?;

    if offer_published(&offer).await? {
        log::info!("Offer already published; Cancelling publish; [{}]", item.id);
        return Ok(());
    }

    ebay_client::publish_offer(user_access_token, &offer_id).await?;
    listing_action::update_listing_status(pgpool, listing, ListingStatus::Published).await?;
    log::info!("Published ebay offer [{}]", offer_id);

    Ok(())
}

pub async fn withdraw(
    pgpool: &PgPool,
    user_access_token: &str,
    listing: &Listing,
) -> Result<(), ShopError> {
    validate_listing_marketplace(listing)?;
    if listing.status != ListingStatus::Published {
        return Err(ShopError::new("listing is not published"));
    }

    let offer_id: Option<String> = get_offer_id(user_access_token, &listing.item_id).await?;
    let Some(offer_id) = offer_id else {
        return Err(ShopError::new("offer ID lookup failed"))
    };

    ebay_client::withdraw_offer(user_access_token, &offer_id).await?;
    listing_action::update_listing_status(pgpool, listing, ListingStatus::Cancelled).await?;
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

fn validate_listing_marketplace(listing: &Listing) -> Result<(), ShopError> {
    if listing.marketplace_id.ne(MARKETPLACE_ID.get().unwrap()) {
        Err(ShopError::new(&format!(
            "Invalid listing; Listing marketplace ID does not match \"{}\"; [{}]",
            MARKETPLACE_INTERNAL_NAME,
            MARKETPLACE_ID.get().ok_or_else(|| ShopError::default())?,
        )))
    } else {
        Ok(())
    }
}

async fn get_offer(
    user_access_token: &str,
    item_id: &Uuid,
) -> Result<Option<Value>, ShopError> {
    let offers_response: Option<Value> = ebay_client::get_offers_fixed_price(user_access_token, item_id).await?;
    let Some(offers_response) = offers_response else {
        return Ok(None);
    };

    let first_offer = &offers_response["offers"][0];
    Ok(Some(first_offer.clone()))
}

async fn offer_published(
    offer: &Value,
) -> Result<bool, ShopError> {
    let first_offer_status = offer["status"]
        .as_str()
        .ok_or_else(|| ShopError::default())?;
    Ok(first_offer_status == "PUBLISHED")
}

/// Get the first offer ID to appear in the list returned by getOffers (for the given SKU)
async fn get_offer_id(
    user_access_token: &str,
    item_id: &Uuid,
) -> Result<Option<String>, ShopError> {
    let response: Option<Value> = ebay_client::get_offers_fixed_price(user_access_token, &item_id).await?;
    let Some(response) = response else {
        return Ok(None);
    };

    let first_offer_id: String = response["offers"][0]["offerId"]
        .as_str()
        .ok_or_else(|| ShopError::default())?
        .to_string();
    Ok(Some(first_offer_id))
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

pub async fn upload_image(
    pgpool: &PgPool,
    user_token: &str,
    item_image_id: &Uuid,
) -> Result<(), ShopError> {
    let item_image: ItemImage = item_image_db::get_item_image(pgpool, item_image_id)
        .await?
        .ok_or_else(|| ShopError::new(&format!("Item image expected; [{}]", item_image_id)))?
        .try_to_model()?;

    _ = ebay_client::upload_image(user_token, &item_image).await?;

    Ok(())
}
