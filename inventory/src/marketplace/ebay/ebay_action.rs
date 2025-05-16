use crate::decrypt::master_decrypt;
use crate::environment::RuntimeEnvironment;
use crate::error::ShopError;
use crate::item::Item;
use crate::listing::{listing_action, Listing, ListingStatus};
use crate::marketplace::marketplace_db;
use crate::product::Product;
use crate::registry::{BASE64, REGISTRY};
use base64::Engine;
use sqlx::PgPool;
use std::sync::{LazyLock, OnceLock};
use uuid::Uuid;

static MARKETPLACE_ID: OnceLock<Uuid> = OnceLock::new();
static EBAY_BASE_URL: LazyLock<&str> = LazyLock::new(|| match RuntimeEnvironment::default() {
    RuntimeEnvironment::Local | RuntimeEnvironment::Stage => "https://api.sandbox.ebay.com/",
    RuntimeEnvironment::Production => "https://api.ebay.com/"
});
// This value pertains to my testing "zach" account. Presumably this will change to an official eBay account.
// This value pertains to my testing "zach" account. Presumably this will change to an official eBay account.
/// Presented as a UTF-8-encoded string because this secret must be re-encoded with the client ID in base64 to form the basic authentication credential
static EBAY_CLIENT_SECRET: LazyLock<String> = LazyLock::new(|| String::from_utf8(master_decrypt("ebay__zach.sandbox.cert_id").unwrap()).unwrap());

const MARKETPLACE_INTERNAL_NAME: &str = "ebay";
const INVENTORY_API_BASE_PATH: &str = "https://api.ebay.com/sell/inventory/v1";
const EBAY_CLIENT_ID: &str = "ZacharyS-shop-SBX-9a6e149a0-59597965";
const EBAY_MARKETPLACE_US: &str = "EBAY_US"; // https://developer.ebay.com/api-docs/sell/account/types/ba:MarketplaceIdEnum
const EBAY_CATEGORY_STANDARD: &str = "ALL_EXCLUDING_MOTORS_VEHICLES"; // https://developer.ebay.com/api-docs/sell/account/types/api:CategoryTypeEnum
const EBAY_CONTENT_LANGUAGE: &str = "en-US";

/// Should be called only once.
pub async fn init(pgpool: &PgPool) {
    let entity = marketplace_db::get_marketplace_by_internal_name(pgpool, MARKETPLACE_INTERNAL_NAME)
        .await
        .expect(&format!("Error querying database for marketplace initialization; [{}]", MARKETPLACE_INTERNAL_NAME))
        .expect(&format!("No marketplace matching the given name; [{}]", MARKETPLACE_INTERNAL_NAME));
    _ = MARKETPLACE_ID.set(entity.id);
}

/// Returns the base64-encoded basic authentication value.
fn basic_auth() -> String {
    let raw: String = format!("{}:{}", EBAY_CLIENT_ID, *EBAY_CLIENT_SECRET);
    BASE64.encode(raw.as_bytes())
}

pub async fn post(pgpool: &PgPool, listing: &Listing) -> Result<(), ShopError> {
    Err(ShopError::new("todo")) // todo
}

/// https://developer.ebay.com/api-docs/sell/inventory/resources/inventory_item/methods/createOrReplaceInventoryItem
pub async fn publish(pgpool: &PgPool, listing: &Listing) -> Result<(), ShopError> {
    if listing.status != ListingStatus::Draft {
        return Err(ShopError::new("Invalid listing; Attempted to publish non-draft listing;"));
    } else if listing.marketplace_id.ne(MARKETPLACE_ID.get().unwrap()) {
        return Err(ShopError::new(&format!(
            "Invalid listing; Listing marketplace ID does not match \"{}\"; [{}]",
            MARKETPLACE_INTERNAL_NAME,
            MARKETPLACE_ID.get().unwrap(),
        )))
    }

    let (item, product): (Item, Product) = listing_action::get_item_and_product_for_listing(pgpool, listing).await?;

    log::info!("Publishing listing to {}; [listing_id: {}]; [marketplace_id: {}]", MARKETPLACE_INTERNAL_NAME, listing.id, MARKETPLACE_ID.get().unwrap());

    // todo: oauth
    let _ = REGISTRY.http_client
        .put(format!("{}/inventory_item/{}", INVENTORY_API_BASE_PATH, item.id))
        .header("content-language", "en-US")
        .header("content-type", "application/json")
        .build();

    Err(ShopError::default())
}
