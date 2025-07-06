use super::*;
use crate::error::ShopError;
use sqlx::postgres::PgQueryResult;
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;

pub async fn get_label(pgpool: &PgPool, label_id: Uuid) -> Result<Option<LabelEntity>, ShopError> {
    query_as!(
		LabelEntity,
		"\
        select id, display_name, internal_name \
        from shop.public.label \
        where id = $1 \
    ",
		label_id
	)
        .fetch_optional(pgpool)
        .await
		.map_err(|e| ShopError::from(e))
}

pub async fn create_label(pgpool: &PgPool, label: &LabelEntity) -> Result<PgQueryResult, ShopError> {
    query!(
		"\
        insert into shop.public.label (id, display_name, internal_name) \
        values ($1, $2, $3) \
    ",
		label.id,
		label.display_name,
		label.internal_name,
	)
        .execute(pgpool)
        .await
		.map_err(|e| ShopError::from(e))
}

pub async fn get_all_labels(pgpool: &PgPool) -> Result<Vec<LabelEntity>, ShopError> {
    query_as!(
		LabelEntity,
		"\
        select id, display_name, internal_name \
        from shop.public.label \
    "
	)
        .fetch_all(pgpool)
        .await
		.map_err(|e| ShopError::from(e))
}
