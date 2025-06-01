use crate::error::ShopError;
use crate::listing::{ListingEntity, ListingStatus};
use sqlx::postgres::PgQueryResult;
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;

pub async fn create_listing(
    pgpool: &PgPool,
    listing: &ListingEntity,
) -> Result<PgQueryResult, ShopError> {
    query!("
        insert into shop.public.listing (id, item_id, marketplace_id, status, created, updated)
        values ($1, $2, $3, $4, $5, $6)
    ",
        listing.id,
        listing.item_id,
        listing.marketplace_id,
        listing.status,
        listing.created,
        listing.updated
    )
        .execute(pgpool)
        .await
        .map_err(|e| ShopError::from(e))
}

pub async fn get_listing(
    pgpool: &PgPool,
    listing_id: &Uuid,
) -> Result<Option<ListingEntity>, ShopError> {
    query_as!(ListingEntity, "
        select *
        from shop.public.listing
        where id = $1
    ",
		listing_id
	)
        .fetch_optional(pgpool)
        .await
        .map_err(|e| ShopError::from(e))
}

pub async fn update_listing(
    pgpool: &PgPool,
    listing: &ListingEntity,
) -> Result<PgQueryResult, ShopError> {
    query!("
        update shop.public.listing
        set (status, updated) = ($2, $3)
        where id = $1
    ",
        listing.id,
        listing.status,
        listing.updated,
    )
        .execute(pgpool)
        .await
        .map_err(|e| ShopError::from(e))
}

pub async fn get_all_by_status_and_marketplace(
    pgpool: &PgPool,
    status: &ListingStatus,
    marketplace_id: &Uuid,
) -> Result<Vec<ListingEntity>, ShopError> {
    query_as!(ListingEntity, "
        select *
        from shop.public.listing
        where status = $1 and marketplace_id = $2
    ",
        i32::from(status as u8),
        marketplace_id,
    )
        .fetch_all(pgpool)
        .await
        .map_err(|e| ShopError::from(e))
}

// todo: Paginated get_all_marketplace_listings(_page) query
// todo: refactor all _db modules to return ShopError like this one
