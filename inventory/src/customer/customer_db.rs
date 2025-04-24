use super::*;
use sqlx::{query, query_as, Error, PgPool};
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

pub async fn get_customer(
    pgpool: &PgPool,
    id: &Uuid,
) -> Result<Option<CustomerEntity>, Error> {
    query_as!(CustomerEntity, "\
        select id, email_address, phone_number, password_hash, display_name, role, status, shipping_street_address, shipping_municipality, shipping_district, shipping_postal_area, shipping_country, billing_street_address, billing_municipality, billing_district, billing_postal_area, billing_country, created, updated \
        from shop.public.customer \
        where id = $1 \
    ",
        id,
    )
        .fetch_optional(pgpool)
        .await
}

pub async fn create_customer(
    pgpool: &PgPool,
    customer: &CustomerEntity,
) -> Result<PgQueryResult, Error> {
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
}
