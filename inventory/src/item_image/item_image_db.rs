use super::*;
use sqlx::postgres::PgQueryResult;
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;

pub async fn get_item_image(
    pgpool: &PgPool,
    item_image_id: &Uuid,
) -> Result<Option<ItemImageEntity>, sqlx::Error> {
    query_as!(
		ItemImageEntity,
		"\
        select id, item_id, alt_text, priority \
        from shop.public.item_image \
        where id = $1 \
    ",
		item_image_id
	)
        .fetch_optional(pgpool)
        .await
}

pub async fn create_item_image(
    pgpool: &PgPool,
    item_image: &ItemImageEntity,
) -> Result<PgQueryResult, sqlx::Error> {
    query!(
		"\
        insert into shop.public.item_image (id, item_id, alt_text, priority) \
        values ($1, $2, $3, $4) \
    ",
		item_image.id,
		item_image.item_id,
		item_image.alt_text,
		item_image.priority,
	)
        .execute(pgpool)
        .await
}

pub async fn get_all_item_images(
    pgpool: &PgPool,
    item_id: &Uuid,
) -> Result<Vec<ItemImageEntity>, sqlx::Error> {
    query_as!(
		ItemImageEntity,
		"\
        select id, item_id, alt_text, priority \
        from shop.public.item_image \
        where item_id = $1 \
    ",
		item_id
	)
        .fetch_all(pgpool)
        .await
}
