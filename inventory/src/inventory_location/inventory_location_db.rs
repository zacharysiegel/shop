use crate::error::ShopError;
use crate::inventory_location::InventoryLocationEntity;
use sqlx::postgres::PgQueryResult;
use sqlx::{query, query_as, PgPool};

pub async fn create_inventory_location(
    pgpool: &PgPool,
    inventory_location: InventoryLocationEntity,
) -> Result<PgQueryResult, ShopError> {
    query!("
		insert into shop.public.inventory_location (id, display_name, internal_name, time_zone_id, street_address, municipality, district, postal_area, country) 
		values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
	",
		inventory_location.id,
		inventory_location.display_name,
		inventory_location.internal_name,
        inventory_location.time_zone_id,
        inventory_location.street_address,
        inventory_location.municipality,
        inventory_location.district,
        inventory_location.postal_area,
        inventory_location.country,
	)
        .execute(pgpool)
        .await
        .map_err(|e| ShopError::from(e))
}

pub async fn get_all_inventory_locations(
    pgpool: &PgPool,
) -> Result<Vec<InventoryLocationEntity>, ShopError> {
    query_as!(InventoryLocationEntity, "
    	select id, display_name, internal_name, time_zone_id, street_address, municipality, district, postal_area, country 
		from shop.public.inventory_location 
	")
        .fetch_all(pgpool)
        .await
        .map_err(|e| ShopError::from(e))
}
