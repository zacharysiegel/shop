use crate::ebay::ebay_category::ebay_category_model::CategoryEntity;
use crate::error::ShopError;
use sqlx::{query_as, PgPool};
use uuid::Uuid;

pub async fn get_ebay_category(pgpool: &PgPool, ebay_category_id: &Uuid) -> Result<Option<CategoryEntity>, ShopError> {
    query_as!(CategoryEntity, "
        select id, ebay_category_id, ebay_category_tree_id, ebay_category_tree_version, ebay_category_name
        from shop.ebay.category
        where id = $1
    ",
        ebay_category_id
    )
        .fetch_optional(pgpool)
        .await
        .map_err(|e| ShopError::from(e))
}
