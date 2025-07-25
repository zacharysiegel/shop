use super::*;
use crate::purchase::PurchaseEntity;
use sqlx::postgres::PgQueryResult;
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;
use crate::error::ShopError;

pub async fn get_customer(pgpool: &PgPool, id: &Uuid) -> Result<Option<CustomerEntity>, ShopError> {
    query_as!(CustomerEntity, "\
        select id, email_address, phone_number, password_hash, display_name, role, status, shipping_street_address, shipping_municipality, shipping_district, shipping_postal_area, shipping_country, billing_street_address, billing_municipality, billing_district, billing_postal_area, billing_country, created, updated \
        from shop.public.customer \
        where id = $1 \
    ",
        id,
    )
        .fetch_optional(pgpool)
        .await
        .map_err(|e| ShopError::from(e))
}

pub async fn create_customer(
    pgpool: &PgPool,
    customer: &CustomerEntity,
) -> Result<PgQueryResult, ShopError> {
    query!("\
        insert into shop.public.customer (id, email_address, phone_number, password_hash, display_name, role, status, shipping_street_address, shipping_municipality, shipping_district, shipping_postal_area, shipping_country, billing_street_address, billing_municipality, billing_district, billing_postal_area, billing_country, created, updated)  \
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
    ",
        customer.id,
        customer.email_address,
        customer.phone_number,
        customer.password_hash,
        customer.display_name,
        customer.role,
        customer.status,
        customer.shipping_street_address,
        customer.shipping_municipality,
        customer.shipping_district,
        customer.shipping_postal_area,
        customer.shipping_country,
        customer.billing_street_address,
        customer.billing_municipality,
        customer.billing_district,
        customer.billing_postal_area,
        customer.billing_country,
        customer.created,
        customer.updated,
    )
        .execute(pgpool)
        .await
        .map_err(|e| ShopError::from(e))
}

pub async fn get_all_customer_purchases(
    pgpool: &PgPool,
    customer_id: &Uuid,
) -> Result<Vec<PurchaseEntity>, ShopError> {
    query_as!(PurchaseEntity, "\
        select id, marketplace_id, external_id, customer_id, contact_email_address, listing_id, status, cost_subtotal_cents, cost_tax_cents, cost_shipping_cents, cost_discount_cents, seller_cost_total_cents, shipping_method, payment_method, note, shipping_street_address, shipping_municipality, shipping_district, shipping_postal_area, shipping_country, billing_street_address, billing_municipality, billing_district, billing_postal_area, billing_country, created, updated \
        from shop.public.purchase \
        where customer_id = $1 \
    ",
        customer_id,
    )
        .fetch_all(pgpool)
        .await
        .map_err(|e| ShopError::from(e))
}
