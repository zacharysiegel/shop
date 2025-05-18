use crate::listing::{listing_db, Listing, ListingEntity, ListingSerial};
use crate::object::JsonHttpResponse;
use crate::{marketplace, unwrap_option_else_404, unwrap_result_else_400, unwrap_result_else_500, ShopEntity, ShopModel, ShopSerial};
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse, Responder};
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;
use uuid::Uuid;

pub fn configurer(config: &mut ServiceConfig) {
    config.service(
        web::scope("/listing")
            .route("", web::post().to(create_listing))
            .route("/{listing_id}", web::get().to(get_listing))
            .route("/{listing_id}", web::put().to(update_listing))
            .route("/{listing_id}/publish", web::put().to(publish_listing))
    );
}

async fn create_listing(
    pgpool: web::Data<PgPool>,
    listing: web::Json<ListingSerial>,
) -> impl Responder {
    let listing = listing.into_inner();
    let listing = unwrap_result_else_400!(listing.try_to_model());

    let query_result: PgQueryResult =
        unwrap_result_else_500!(listing_db::create_listing(&pgpool, &listing.to_entity()).await);

    // todo: refactor the create endpoints to return the created item (contains the generated id)
    HttpResponse::Ok().body(query_result.rows_affected().to_string())
}

async fn get_listing(
    pgpool: web::Data<PgPool>,
    listing_id: web::Path<String>,
) -> impl Responder {
    let listing_id = unwrap_result_else_400!(Uuid::try_parse(&listing_id.into_inner()));
    let listing = unwrap_result_else_500!(listing_db::get_listing(&pgpool, &listing_id).await);

    let listing = unwrap_option_else_404!(listing);
    let listing = unwrap_result_else_500!(listing.try_to_model());
    listing.to_serial().to_http_response()
}

async fn update_listing(
    pgpool: web::Data<PgPool>,
    listing_id: web::Path<String>,
    listing: web::Json<ListingSerial>,
) -> impl Responder {
    let listing_id: Uuid = unwrap_result_else_400!(Uuid::try_parse(&listing_id.into_inner()));
    let mut listing: Listing = unwrap_result_else_400!(listing.into_inner().try_to_model());
    listing.id = listing_id; // Listing ID is overridden with a random UUID in `try_to_model`.

    let query_result: PgQueryResult =
        unwrap_result_else_500!(listing_db::update_listing(&pgpool, &listing.to_entity()).await);
    HttpResponse::Ok().body(query_result.rows_affected().to_string())
}

async fn publish_listing(
    pgpool: web::Data<PgPool>,
    listing_id: web::Path<String>,
) -> impl Responder {
    let listing_id: Uuid = unwrap_result_else_400!(Uuid::try_parse(&listing_id.into_inner()));

    let listing_entity: Option<ListingEntity> = unwrap_result_else_500!(listing_db::get_listing(&pgpool, &listing_id).await);
    let listing_entity: ListingEntity = unwrap_option_else_404!(listing_entity);
    let listing: Listing = unwrap_result_else_500!(listing_entity.try_to_model());

    unwrap_result_else_500!(marketplace::ebay::ebay_action::publish(&pgpool, &listing).await);
    unwrap_result_else_500!(listing_db::publish_listing(&pgpool, &listing_id).await);

    HttpResponse::Ok().finish()
}
