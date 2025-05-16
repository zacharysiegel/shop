use crate::error::ShopError;
use crate::item::{item_db, Item, ItemEntity};
use crate::listing::Listing;
use crate::product::{product_db, Product, ProductEntity};
use crate::ShopEntity;
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
