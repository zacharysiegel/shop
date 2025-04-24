use crate::inventory_location::InventoryLocationEntity;
use sqlx::postgres::PgQueryResult;
use sqlx::{query, query_as, Error, PgPool};

pub async fn create_inventory_location(
    pgpool: &PgPool,
    inventory_location: InventoryLocationEntity,
) -> Result<PgQueryResult, Error> {
    query!(
		"\
		insert into shop.public.inventory_location (id, display_name, internal_name) \
		values ($1, $2, $3)\
	",
		inventory_location.id,
		inventory_location.display_name,
		inventory_location.internal_name
	)
        .execute(pgpool)
        .await
}

pub async fn get_all_inventory_locations(
    pgpool: &PgPool,
) -> Result<Vec<InventoryLocationEntity>, Error> {
    query_as!(
		InventoryLocationEntity,
		"\
    	select id, display_name, internal_name \
		from shop.public.inventory_location \
	"
	)
        .fetch_all(pgpool)
        .await
}
