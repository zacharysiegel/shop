use super::*;
use sqlx::postgres::PgQueryResult;
use sqlx::{query, query_as, Error, PgPool, Pool, Postgres};
use uuid::Uuid;

pub async fn get_all_categories(pool: &PgPool) -> Result<Vec<CategoryEntity>, Error> {
    query_as!(CategoryEntity, "
		select id, display_name, internal_name, parent_id, ebay_category_id
		from shop.public.category
	")
        .fetch_all(pool)
        .await
}

pub async fn get_category(
    pool: &Pool<Postgres>,
    id: Uuid,
) -> Result<Option<CategoryEntity>, Error> {
    query_as!(CategoryEntity, "
		select id, display_name, internal_name, parent_id, ebay_category_id
		from shop.public.category
		where id = $1
	",
		id
	)
        .fetch_optional(pool)
        .await
}

pub async fn create_category(
    pool: &Pool<Postgres>,
    category: CategoryEntity,
) -> Result<PgQueryResult, Error> {
    query!("
		insert into shop.public.category (id, display_name, internal_name, parent_id)
		values ($1, $2, $3, $4)
	",
		category.id,
		category.display_name,
		category.internal_name,
		category.parent_id
	)
        .execute(pool)
        .await
}
