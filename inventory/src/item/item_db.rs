use super::*;
use sqlx::postgres::PgQueryResult;
use sqlx::{Error, PgPool, query, query_as};
use uuid::Uuid;

pub async fn get_item(pgpool: &PgPool, item_id: Uuid) -> Result<Option<ItemEntity>, Error> {
    query_as!(ItemEntity, "\
		select id, product_id, inventory_location_id, condition, status, price_cents, priority, note, acquisition_datetime, acquisition_price_cents, acquisition_location, created, updated \
		from item \
		where id = $1 \
		",
		item_id
	)
        .fetch_optional(pgpool)
        .await
}

pub async fn create_item(pgpool: &PgPool, item: ItemEntity) -> Result<PgQueryResult, Error> {
    query!("\
		insert into item (id, product_id, inventory_location_id, condition, status, price_cents, priority, note, acquisition_datetime, acquisition_price_cents, acquisition_location, created, updated)\
		values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)\
		",
	item.id,
	item.product_id,
	item.inventory_location_id,
	item.condition,
	item.status,
	item.price_cents,
	item.priority,
	item.note,
	item.acquisition_datetime,
	item.acquisition_price_cents,
	item.acquisition_location,
	item.created,
	item.updated,
	)
        .execute(pgpool)
        .await
}
