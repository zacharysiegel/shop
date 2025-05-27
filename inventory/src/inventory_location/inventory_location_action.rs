use crate::error::ShopError;
use crate::inventory_location::{InventoryLocation, InventoryLocationEntity};
use crate::ShopEntity;
use sqlx::PgPool;

pub async fn get_all_inventory_locations(pgpool: &PgPool) -> Result<Vec<InventoryLocation>, ShopError> {
    let inventory_location_vec: Vec<InventoryLocationEntity> = super::inventory_location_db::get_all_inventory_locations(pgpool)
        .await
        .map_err(|e| ShopError::from_error("get_all_inventory_locations", Box::new(e)))?;
    let inventory_location_vec: Result<Vec<InventoryLocation>, ShopError> = inventory_location_vec
        .iter()
        .map(|entity| entity.try_to_model())
        .collect::<Result<Vec<InventoryLocation>, ShopError>>();
    let inventory_location_vec: Vec<InventoryLocation> = inventory_location_vec
        .map_err(|e| ShopError::from_error("InventoryLocation::try_from_entity", Box::new(e)))?;
    Ok(inventory_location_vec)
}
