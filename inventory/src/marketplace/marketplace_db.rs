use crate::marketplace::MarketplaceEntity;
use sqlx::postgres::PgQueryResult;
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;
use crate::error::ShopError;

pub async fn get_marketplace(
    pgpool: &PgPool,
    id: &Uuid,
) -> Result<Option<MarketplaceEntity>, ShopError> {
    query_as!(MarketplaceEntity, "
        select id, display_name, internal_name, uri
        from shop.public.marketplace
        where id = $1
    ",
		id
	)
        .fetch_optional(pgpool)
        .await
        .map_err(|e| ShopError::from(e))
}

pub async fn get_all_marketplaces(pgpool: &PgPool) -> Result<Vec<MarketplaceEntity>, ShopError> {
    query_as!(MarketplaceEntity, "
        select id, display_name, internal_name, uri
        from shop.public.marketplace
        order by internal_name asc, id asc
    ")
        .fetch_all(pgpool)
        .await
        .map_err(|e| ShopError::from(e))
}

pub async fn create_marketplace(
    pgpool: &PgPool,
    marketplace: &MarketplaceEntity,
) -> Result<PgQueryResult, ShopError> {
    query!("
		INSERT INTO shop.public.marketplace (id, display_name, internal_name, uri)
		VALUES ($1, $2, $3, $4)
	",
		marketplace.id,
		marketplace.display_name,
		marketplace.internal_name,
		marketplace.uri,
	)
        .execute(pgpool)
        .await
        .map_err(|e| ShopError::from(e))
}

pub async fn get_marketplace_by_internal_name(
    pgpool: &PgPool,
    internal_name: &str,
) -> Result<Option<MarketplaceEntity>, ShopError> {
    query_as!(MarketplaceEntity, "
        select id, display_name, internal_name, uri
        from shop.public.marketplace
        where internal_name = $1
    ",
		internal_name
	)
        .fetch_optional(pgpool)
        .await
        .map_err(|e| ShopError::from(e))
}
