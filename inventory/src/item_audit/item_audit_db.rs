use super::*;
use crate::error::ShopError;
use sqlx::{query_as, PgPool};
use uuid::Uuid;

pub async fn get_all_item_item_audits(
    pgpool: &PgPool,
    item_id: &Uuid,
) -> Result<Vec<ItemAuditEntity>, ShopError> {
    query_as!(
		ItemAuditEntity,
		"\
        select id, item_id, status_before, status_after, initiated_by_admin, note, created \
        from shop.public.item_audit \
        where item_audit.item_id = $1
    ",
		item_id
	)
        .fetch_all(pgpool)
        .await
		.map_err(|e| ShopError::from(e))
}
