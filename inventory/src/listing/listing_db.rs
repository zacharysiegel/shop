use crate::listing::ListingEntity;
use sqlx::postgres::PgQueryResult;
use sqlx::{query, query_as, Error, PgPool};
use uuid::Uuid;

pub async fn create_listing(
    pgpool: &PgPool,
    listing: &ListingEntity,
) -> Result<PgQueryResult, Error> {
    query!("\
        insert into shop.public.listing (id, item_id, marketplace_id, uri, status, created, updated)  \
        values ($1, $2, $3, $4, $5, $6, $7) \
    ",
        listing.id,
        listing.item_id,
        listing.marketplace_id,
        listing.uri,
        listing.status,
        listing.created,
        listing.updated
    )
        .execute(pgpool)
        .await
}

pub async fn get_listing(
    pgpool: &PgPool,
    listing_id: &Uuid,
) -> Result<Option<ListingEntity>, Error> {
    query_as!(
		ListingEntity,
		"\
        select * \
        from shop.public.listing \
        where id = $1 \
    ",
		listing_id
	)
        .fetch_optional(pgpool)
        .await
}

// todo: figure out pagination in Postgres for a get_all_marketplace_listings(_page) query
