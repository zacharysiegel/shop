use super::*;
use crate::error::ShopError;
use sqlx::postgres::PgQueryResult;
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;

pub async fn get_item_image(
    pgpool: &PgPool,
    item_image_id: &Uuid,
) -> Result<Option<ItemImageEntity>, ShopError> {
    query_as!(
		ItemImageEntity,
		"\
        select id, item_id, alt_text, priority, original_file_name \
        from shop.public.item_image \
        where id = $1 \
    ",
		item_image_id
	)
        .fetch_optional(pgpool)
        .await
        .map_err(|e| ShopError::from(e))
}

pub async fn create_item_image(
    pgpool: &PgPool,
    item_image: &ItemImageEntity,
) -> Result<PgQueryResult, ShopError> {
    query!(
		"\
        insert into shop.public.item_image (id, item_id, alt_text, priority, original_file_name) \
        values ($1, $2, $3, $4, $5) \
    ",
		item_image.id,
		item_image.item_id,
		item_image.alt_text,
		item_image.priority,
		item_image.original_file_name,
	)
        .execute(pgpool)
        .await
        .map_err(|e| ShopError::from(e))
}

pub async fn delete_item_image(
    pgpool: &PgPool,
    item_image_id: &Uuid,
) -> Result<PgQueryResult, ShopError> {
    query!("
		delete
		from shop.public.item_image
		where id = $1
	",
		item_image_id,
	)
        .execute(pgpool)
        .await
        .map_err(|e| ShopError::from(e))
}

pub async fn get_all_item_images(
    pgpool: &PgPool,
    item_id: &Uuid,
) -> Result<Vec<ItemImageEntity>, ShopError> {
    query_as!(ItemImageEntity, "
        select id, item_id, alt_text, priority, original_file_name
        from shop.public.item_image
        where item_id = $1
    ",
		item_id
	)
        .fetch_all(pgpool)
        .await
        .map_err(|e| ShopError::from(e))
}
