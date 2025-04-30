use super::*;
use crate::category::CategoryEntity;
use crate::item::ItemEntity;
use crate::pagination::{Direction, KeysetPaginationOptionsForString, KeysetPaginationResultForString};
use sqlx::postgres::{PgArguments, PgQueryResult, PgRow};
use sqlx::query::Map;
use sqlx::{query, query_as, Error, PgPool, Postgres};
use uuid::Uuid;

pub async fn get_all_products(pgpool: &PgPool) -> Result<Vec<ProductEntity>, Error> {
    query_as!(ProductEntity, "\
		select id, display_name, internal_name, upc, release_date, created, updated \
	 	from shop.public.product \
	")
        .fetch_all(pgpool)
        .await
}

pub async fn get_all_products_paged_display_name(
    pgpool: &PgPool,
    keyset_pagination_options: KeysetPaginationOptionsForString,
) -> Result<(Vec<ProductEntity>, KeysetPaginationResultForString), Error> {
    let limit: u32 = if keyset_pagination_options.start_value.is_none() {
        keyset_pagination_options.max_page_size
    } else {
        keyset_pagination_options.max_page_size.saturating_add(1)
    };
    let start_value: String = keyset_pagination_options.start_value.unwrap_or_default().to_string();

    /* This function always returns records in ascending order on the ordered column, but we must alternate
        between ascending and descending in order to move forward and backward between pages. */
    let query: Map<Postgres, fn(PgRow) -> Result<ProductEntity, Error>, PgArguments> = match keyset_pagination_options.direction {
        Direction::Ascending => {
            query_as!(ProductEntity, "\
        		select id, display_name, internal_name, upc, release_date, created, updated
        		from shop.public.product
        		where display_name >= $1
        		order by display_name asc
                limit $2
        	",
                start_value,
                i64::from(limit),
            )
        }
        Direction::Descending => {
            query_as!(ProductEntity, "\
                with page as (
                    select id, display_name, internal_name, upc, release_date, created, updated
                    from shop.public.product
                    where display_name <= $1
                    order by display_name desc
                    limit $2
                )
                select id, display_name, internal_name, upc, release_date, created, updated
                from page
                order by display_name asc
        	",
                start_value,
                i64::from(limit),
            )
        }
    };

    let entities: Vec<ProductEntity> = query.fetch_all(pgpool).await?;
    let relation_max_entity: Option<ProductEntity> = query_as!(ProductEntity,
        "select * from shop.public.product order by display_name desc limit 1")
        .fetch_optional(pgpool)
        .await?;
    let relation_min_entity: Option<ProductEntity> = query_as!(ProductEntity,
        "select * from shop.public.product order by display_name asc limit 1")
        .fetch_optional(pgpool)
        .await?;

    Ok(KeysetPaginationResultForString::from_entities(
        entities,
        relation_min_entity,
        relation_max_entity,
        |val| val.display_name,
        keyset_pagination_options.max_page_size as usize,
    ))
}

pub async fn get_product(
    pgpool: &PgPool,
    product_id: &Uuid,
) -> Result<Option<ProductEntity>, Error> {
    query_as!(ProductEntity, "\
		select id, display_name, internal_name, upc, release_date, created, updated \
		from shop.public.product \
		where id = $1 \
	",
		product_id
	)
        .fetch_optional(pgpool)
        .await
}

pub async fn delete_product(
    pgpool: &PgPool,
    product_id: &Uuid,
) -> Result<PgQueryResult, Error> {
    query!("
        delete from shop.public.product
        where id = $1
    ",
        product_id,
    )
        .execute(pgpool)
        .await
}

pub async fn get_product_categories(
    pgpool: &PgPool,
    product_id: &Uuid,
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
    product: &ProductEntity,
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
    product_id: &Uuid,
    category_id: &Uuid,
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

pub async fn delete_product_category_association(
    pgpool: &PgPool,
    product_id: &Uuid,
    category_id: &Uuid,
) -> Result<PgQueryResult, Error> {
    query!(
		"\
		delete from shop.public.product_category_association \
		where product_id = $1 and category_id = $2 \
	",
		product_id,
		category_id
	)
        .execute(pgpool)
        .await
}

pub async fn get_all_product_items(
    pgpool: &PgPool,
    product_id: &Uuid,
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
