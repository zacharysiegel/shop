use crate::marketplace::{marketplace_db, MarketplaceEntity, MarketplaceSerial};
use crate::object::JsonHttpResponse;
use crate::{ebay, unwrap_option_else_404, unwrap_result_else_400, unwrap_result_else_500, ShopModel, ShopSerial};
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse, Responder};
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;
use uuid::Uuid;

pub fn configurer(config: &mut ServiceConfig) {
    config
        .service(web::scope("/marketplace")
            .route("", web::get().to(get_all_marketplaces))
            .route("", web::post().to(create_marketplace))
            .route("/{id}", web::get().to(get_marketplace))
        )
        .configure(ebay::ebay_api::configurer)
    ;
}

async fn get_all_marketplaces(pgpool: web::Data<PgPool>) -> impl Responder {
    unwrap_result_else_500!(marketplace_db::get_all_marketplaces(&pgpool).await)
        .iter()
        .map(|marketplace| marketplace.to_serial())
        .collect::<Vec<MarketplaceSerial>>()
        .to_http_response()
}

async fn create_marketplace(
    pgpool: web::Data<PgPool>,
    marketplace: web::Json<MarketplaceSerial>,
) -> impl Responder {
    let marketplace: MarketplaceEntity = unwrap_result_else_400!(marketplace.into_inner().try_to_model());
    let query_result: PgQueryResult = unwrap_result_else_500!(marketplace_db::create_marketplace(&pgpool, &marketplace).await);
    HttpResponse::Ok().body(query_result.rows_affected().to_string())
}

async fn get_marketplace(pgpool: web::Data<PgPool>, id: web::Path<String>) -> impl Responder {
    let id = unwrap_result_else_400!(Uuid::try_parse(id.into_inner().as_str()));
    let marketplace = unwrap_result_else_500!(marketplace_db::get_marketplace(&pgpool, &id).await);
    let marketplace = unwrap_option_else_404!(marketplace);

    marketplace.to_serial().to_http_response()
}
