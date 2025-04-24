use crate::listing::{listing_db, ListingSerial};
use crate::server::JsonHttpResponse;
use crate::{unwrap_option_else_404, unwrap_result_else_400, unwrap_result_else_500, ShopEntity, ShopModel, ShopSerial};
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

pub fn configurer(config: &mut ServiceConfig) {
    config.service(
        web::scope("/listing")
            .route("", web::post().to(create_listing))
            .route("/{id}", web::get().to(get_listing))
    );
}

async fn create_listing(
    pgpool: web::Data<PgPool>,
    listing: web::Json<ListingSerial>,
) -> impl Responder {
    let listing = listing.into_inner();
    let listing = unwrap_result_else_400!(listing.try_to_model());

    let query_result = unwrap_result_else_500!(listing_db::create_listing(&pgpool, &listing.to_entity()).await);
    HttpResponse::Ok().body(query_result.rows_affected().to_string())
}

async fn get_listing(
    pgpool: web::Data<PgPool>,
    id: web::Path<String>,
) -> impl Responder {
    let id = unwrap_result_else_400!(Uuid::try_parse(id.into_inner().as_str()));
    let listing = unwrap_result_else_500!(
        listing_db::get_listing(&pgpool, &id).await
    );

    let listing = unwrap_option_else_404!(listing);
    let listing = unwrap_result_else_500!(listing.try_to_model());
    listing.to_serial().to_http_response()
}
