use crate::error::ShopError;
use crate::item::{item_db, Item, ItemEntity};
use crate::listing::{Listing, ListingStatus};
use crate::product::{product_db, Product, ProductEntity};
use crate::{listing, ShopEntity, ShopModel};
use sqlx::PgPool;

pub async fn get_item_and_product_for_listing(
    pgpool: &PgPool,
    listing: &Listing,
) -> Result<(Item, Product), ShopError> {
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

    Ok((item, product))
}

pub async fn update_listing_status(
    pgpool: &PgPool,
    listing: &Listing,
    new_status: ListingStatus,
) -> Result<(), ShopError> {
    let mut updated_listing: Listing = listing.clone();
    updated_listing.status = new_status;

    listing::listing_db::update_listing(pgpool, &updated_listing.to_entity())
        .await?;

    Ok(())
}
