use super::*;
use crate::label::Label;
use sqlx::postgres::PgQueryResult;
use sqlx::{query, query_as, Error, PgPool};
use uuid::Uuid;
use crate::listing::ListingEntity;

pub async fn get_item(pgpool: &PgPool, item_id: &Uuid) -> Result<Option<ItemEntity>, Error> {
    query_as!(ItemEntity, "\
		select id, product_id, inventory_location_id, condition, status, price_cents, priority, note, acquisition_datetime, acquisition_price_cents, acquisition_location, created, updated \
		from shop.public.item \
		where id = $1 \
		",
		item_id
	)
        .fetch_optional(pgpool)
        .await
}

pub async fn create_item(pgpool: &PgPool, item: &ItemEntity) -> Result<PgQueryResult, Error> {
    query!("\
		insert into shop.public.item (id, product_id, inventory_location_id, condition, status, price_cents, priority, note, acquisition_datetime, acquisition_price_cents, acquisition_location, created, updated)\
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

pub async fn get_all_item_labels(
    pgpool: &PgPool,
    item_id: &Uuid,
) -> Result<Vec<Label>, Error> {
    query_as!(Label, "
        select id, display_name, internal_name
		from shop.public.label
        inner join shop.public.item_label_association on label.id = item_label_association.label_id
        where item_label_association.item_id = $1
    ", item_id)
        .fetch_all(pgpool)
        .await
}

pub async fn create_item_label_association(
    pgpool: &PgPool,
    item_id: &Uuid,
    label_id: &Uuid,
) -> Result<PgQueryResult, Error> {
    query!("\
		insert into shop.public.item_label_association (item_id, label_id) \
		values ($1, $2) \
	",
		item_id,
		label_id
	)
        .execute(pgpool)
        .await
}

pub async fn delete_item_label_association(pgpool: &PgPool, item_id: &Uuid, label_id: &Uuid) -> Result<PgQueryResult, Error> {
    query!("\
		delete from shop.public.item_label_association \
		where item_id = $1 and label_id = $2 \
	",
		item_id,
		label_id
	)
        .execute(pgpool)
        .await
}

pub async fn get_all_item_listings(pgpool: &PgPool, item_id: &Uuid) -> Result<Vec<ListingEntity>, Error> {
	query_as!(ListingEntity, "\
		select * \
		from shop.public.listing \
		where item_id = $1 \
	",
		item_id,
	)
		.fetch_all(pgpool)
		.await
}
