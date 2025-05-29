use crate::marketplace::ebay::ebay_category::ebay_category_model::CategoryEntity;
use sqlx::{query_as, Error, PgPool};
use uuid::Uuid;

pub async fn get_ebay_category(pgpool: &PgPool, id: &Uuid) -> Result<Option<CategoryEntity>, Error> {
    query_as!(CategoryEntity, "
        select id, ebay_category_id, ebay_category_tree_id, ebay_category_tree_version, ebay_category_name
        from shop.ebay.category
        where id = $1
    ",
        id
    )
        .fetch_optional(pgpool)
        .await
}
