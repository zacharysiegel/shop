use crate::marketplace::{marketplace_db, MarketplaceSerial};
use crate::server::JsonHttpResponse;
use crate::{unwrap_option_else_404, unwrap_result_else_400, unwrap_result_else_500, ShopModel};
use actix_web::web::ServiceConfig;
use actix_web::{web, Responder};
use sqlx::PgPool;
use uuid::Uuid;

pub fn configurer(config: &mut ServiceConfig) {
    config.service(
        web::scope("/marketplace")
            .route("", web::get().to(get_all_marketplaces))
            .route("/{id}", web::get().to(get_marketplace))
    )
    ;
}

async fn get_marketplace(
    pgpool: web::Data<PgPool>,
    id: web::Path<String>,
) -> impl Responder {
    let id = unwrap_result_else_400!(Uuid::try_parse(id.into_inner().as_str()));
    let marketplace = unwrap_result_else_500!(marketplace_db::get_marketplace(&pgpool, &id).await);
    let marketplace = unwrap_option_else_404!(marketplace);

    marketplace
        .to_serial()
        .to_http_response()
}

async fn get_all_marketplaces(
    pgpool: web::Data<PgPool>,
) -> impl Responder {
    unwrap_result_else_500!(marketplace_db::get_all_marketplaces(&pgpool).await)
        .iter()
        .map(|marketplace| marketplace.to_serial())
        .collect::<Vec<MarketplaceSerial>>()
        .to_http_response()
}
