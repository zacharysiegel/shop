use crate::marketplace::MarketplaceEntity;
use sqlx::{query, query_as, Error, PgPool};
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

pub async fn get_marketplace(
    pgpool: &PgPool,
    id: &Uuid,
) -> Result<Option<MarketplaceEntity>, Error> {
    query_as!(
		MarketplaceEntity,
		"\
        select id, display_name, internal_name, uri \
        from shop.public.marketplace \
        where id = $1 \
    ",
		id
	)
        .fetch_optional(pgpool)
        .await
}

pub async fn get_all_marketplaces(pgpool: &PgPool) -> Result<Vec<MarketplaceEntity>, Error> {
    query_as!(
		MarketplaceEntity,
		"\
        select id, display_name, internal_name, uri \
        from shop.public.marketplace \
        order by internal_name asc, id asc \
    "
	)
        .fetch_all(pgpool)
        .await
}

pub async fn create_marketplace(
	pgpool: &PgPool,
	marketplace: &MarketplaceEntity,
) -> Result<PgQueryResult, Error> {
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
}
