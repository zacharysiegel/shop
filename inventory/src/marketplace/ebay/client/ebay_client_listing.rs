use crate::error::ShopError;
use crate::item::Item;
use crate::listing::Listing;
use crate::product::Product;

const INVENTORY_API_BASE_PATH: &str = "/sell/inventory/v1";

pub async fn create_or_replace_inventory_item(
    listing: &Listing,
    item: &Item,
    product: &Product,
) -> Result<(), ShopError> {
    Ok(()) // todo
}

pub async fn publish_listing(
    listing: &Listing,
    item: &Item,
    product: &Product,
) -> Result<(), ShopError> {
    Ok(()) // todo
}
