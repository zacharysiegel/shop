use crate::listing::ListingEntity;
use crate::purchase::PurchaseEntity;
use sqlx::postgres::PgQueryResult;
use sqlx::{query, query_as, Error, PgPool};
use uuid::Uuid;

pub async fn create_purchase(
    pgpool: &PgPool,
    purchase_entity: &PurchaseEntity,
) -> Result<PgQueryResult, Error> {
    query!("\
        insert into shop.public.purchase (id, marketplace_id, external_id, customer_id, contact_email_address, listing_id, status, cost_subtotal_cents, cost_tax_cents, cost_shipping_cents, cost_discount_cents, seller_cost_total_cents, shipping_method, payment_method, note, shipping_street_address, shipping_municipality, shipping_district, shipping_postal_area, shipping_country, billing_street_address, billing_municipality, billing_district, billing_postal_area, billing_country, created, updated) \
        values ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20,$21,$22,$23,$24,$25,$26,$27) \
    ",
        purchase_entity.id,
        purchase_entity.marketplace_id,
        purchase_entity.external_id,
        purchase_entity.customer_id,
        purchase_entity.contact_email_address,
        purchase_entity.listing_id,
        purchase_entity.status,
        purchase_entity.cost_subtotal_cents,
        purchase_entity.cost_tax_cents,
        purchase_entity.cost_shipping_cents,
        purchase_entity.cost_discount_cents,
        purchase_entity.seller_cost_total_cents,
        purchase_entity.shipping_method,
        purchase_entity.payment_method,
        purchase_entity.note,
        purchase_entity.shipping_street_address,
        purchase_entity.shipping_municipality,
        purchase_entity.shipping_district,
        purchase_entity.shipping_postal_area,
        purchase_entity.shipping_country,
        purchase_entity.billing_street_address,
        purchase_entity.billing_municipality,
        purchase_entity.billing_district,
        purchase_entity.billing_postal_area,
        purchase_entity.billing_country,
        purchase_entity.created,
        purchase_entity.updated,
    )
        .execute(pgpool)
        .await
}

pub async fn get_purchase(
    pgpool: &PgPool,
    purchase_id: &Uuid,
) -> Result<Option<PurchaseEntity>, Error> {
    query_as!(
		PurchaseEntity,
		"\
        select * \
        from shop.public.purchase \
        where id = $1 \
    ",
		purchase_id,
	)
        .fetch_optional(pgpool)
        .await
}

pub async fn get_purchase_listing(
    pgpool: &PgPool,
    purchase_id: &Uuid,
) -> Result<Option<ListingEntity>, Error> {
    query_as!(ListingEntity, "\
        select listing.id, listing.item_id, listing.marketplace_id, listing.uri, listing.status, listing.created, listing.updated \
        from shop.public.listing \
        inner join shop.public.purchase on listing.id = purchase.listing_id \
        where purchase.id = $1
    ",
        purchase_id,
    )
        .fetch_optional(pgpool)
        .await
}
