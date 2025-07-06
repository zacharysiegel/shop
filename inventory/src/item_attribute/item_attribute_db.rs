use crate::item_attribute::ItemAttributeEntity;
use sqlx::postgres::PgQueryResult;
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;
use crate::error::ShopError;

pub async fn get_item_attribute(
    pgpool: &PgPool,
    item_id: &Uuid,
    key: &str,
) -> Result<Option<ItemAttributeEntity>, ShopError> {
    query_as!(ItemAttributeEntity, "
        select item_id, key, value, visible, priority
        from shop.public.item_attribute
        where item_id = $1 and key = $2
    ",
		item_id,
		key,
	)
        .fetch_optional(pgpool)
        .await
		.map_err(|e| ShopError::from(e))
}

pub async fn get_all_item_attributes(
    pgpool: &PgPool,
    item_id: &Uuid,
) -> Result<Vec<ItemAttributeEntity>, ShopError> {
    query_as!(ItemAttributeEntity, "
        select item_id, key, value, visible, priority
        from shop.public.item_attribute
        where item_id = $1
    ",
		item_id,
	)
        .fetch_all(pgpool)
        .await
		.map_err(|e| ShopError::from(e))
}

pub async fn create_item_attribute(
    pgpool: &PgPool,
    item_attribute: &ItemAttributeEntity,
) -> Result<PgQueryResult, ShopError> {
    query!("
        insert into shop.public.item_attribute (item_id, key, value, visible, priority)
        values ($1, $2, $3, $4, $5)
    ",
		item_attribute.item_id,
		item_attribute.key,
		item_attribute.value,
		item_attribute.visible,
		item_attribute.priority,
	)
        .execute(pgpool)
        .await
		.map_err(|e| ShopError::from(e))
}

pub async fn delete_item_attribute(
    pgpool: &PgPool,
    item_id: &Uuid,
    key: &str,
) -> Result<PgQueryResult, ShopError> {
    query!("
        delete from shop.public.item_attribute
        where item_id = $1 and key = $2
    ",
		item_id,
		key,
	)
        .execute(pgpool)
        .await
		.map_err(|e| ShopError::from(e))
}
