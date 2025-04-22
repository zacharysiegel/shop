use super::*;
use crate::category::CategoryEntity;
use crate::item::ItemEntity;
use sqlx::postgres::PgQueryResult;
use sqlx::{Error, PgPool, query, query_as};
use uuid::Uuid;

pub async fn get_product(
	pgpool: &PgPool,
	product_id: Uuid,
) -> Result<Option<ProductEntity>, Error> {
	query_as!(
		ProductEntity,
		"\
		select id, display_name, internal_name, upc, release_date, created, updated \
		from shop.public.product \
		where id = $1 \
		",
		product_id
	)
	.fetch_optional(pgpool)
	.await
}

pub async fn get_product_categories(
	pgpool: &PgPool,
	product_id: Uuid,
) -> Result<Vec<CategoryEntity>, Error> {
	query_as!(CategoryEntity, "
        select category.id, category.display_name, category.internal_name, category.parent_id
		from shop.public.category
        inner join shop.public.product_category_association on category.id = product_category_association.category_id
        where product_category_association.product_id = $1
    ", product_id)
        .fetch_all(pgpool)
        .await
}

pub async fn create_product(
	pgpool: &PgPool,
	product: ProductEntity,
) -> Result<PgQueryResult, Error> {
	query!(
		"\
		insert into shop.public.product (id, display_name, internal_name, upc, release_date, created, updated)\
		values ($1, $2, $3, $4, $5, $6, $7)\
		",
		product.id,
		product.display_name,
		product.internal_name,
		product.upc,
		product.release_date,
		product.created,
		product.updated
	)
	.execute(pgpool)
	.await
}

pub async fn create_product_category_association(
	pgpool: &PgPool,
	product_id: Uuid,
	category_id: Uuid,
) -> Result<PgQueryResult, Error> {
	query!(
		"\
		insert into shop.public.product_category_association (category_id, product_id)\
		values ($1, $2)\
		",
		category_id,
		product_id,
	)
	.execute(pgpool)
	.await
}

pub async fn get_product_items(
	pgpool: &PgPool,
	product_id: Uuid,
) -> Result<Vec<ItemEntity>, Error> {
	query_as!(
		ItemEntity,
		"\
		select id, product_id, inventory_location_id, condition, status, price_cents, priority, note, acquisition_datetime, acquisition_price_cents, acquisition_location, created, updated \
		from shop.public.item \
		where product_id = $1 \
		",
		product_id
	)
	.fetch_all(pgpool)
	.await
}
