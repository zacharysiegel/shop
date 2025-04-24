use crate::purchase::{purchase_db, PurchaseSerial};
use crate::server::JsonHttpResponse;
use crate::{unwrap_option_else_404, unwrap_result_else_400, unwrap_result_else_500, ShopEntity, ShopModel, ShopSerial};
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

pub fn configurer(config: &mut ServiceConfig) {
    config.service(
        web::scope("/purchase")
            .route("", web::post().to(create_purchase))
            .route("/{purchase_id}", web::get().to(get_purchase))
            .route("/{purchase_id}/listing", web::get().to(get_purchase_listing))
    );
}

async fn create_purchase(
    pgpool: web::Data<PgPool>,
    purchase: web::Json<PurchaseSerial>,
) -> impl Responder {
    let purchase = unwrap_result_else_400!(purchase.into_inner().try_to_model());
    let query_result = unwrap_result_else_500!(purchase_db::create_purchase(&pgpool, &purchase.to_entity()).await);

    HttpResponse::Ok().body(query_result.rows_affected().to_string())
}

async fn get_purchase(
    pgpool: web::Data<PgPool>,
    purchase_id: web::Path<String>,
) -> impl Responder {
    let purchase_id = unwrap_result_else_400!(Uuid::try_parse(purchase_id.into_inner().as_str()));
    let purchase = unwrap_result_else_500!(purchase_db::get_purchase(&pgpool, &purchase_id).await);
    let purchase = unwrap_option_else_404!(purchase);
    let purchase = unwrap_result_else_500!(purchase.try_to_model());

    purchase
        .to_serial()
        .to_http_response()
}

async fn get_purchase_listing(
    pgpool: web::Data<PgPool>,
    path: web::Path<String>,
) -> impl Responder {
    let purchase_id = unwrap_result_else_400!(Uuid::try_parse(path.into_inner().as_str()));

    let listing = unwrap_result_else_500!(purchase_db::get_purchase_listing(&pgpool, &purchase_id).await);
    let listing = unwrap_option_else_404!(listing);
    let listing = unwrap_result_else_500!(listing.try_to_model());
    listing.to_serial().to_http_response()
}
