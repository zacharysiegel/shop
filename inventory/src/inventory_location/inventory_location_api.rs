use super::*;
use crate::object::JsonHttpResponse;
use crate::{unwrap_result_else_500, ShopModel};
use actix_web::http::StatusCode;
use actix_web::{get, post, web, HttpResponseBuilder, Responder};
use sqlx::postgres::PgQueryResult;
use sqlx::{Pool, Postgres};

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/inventory_location")
            .service(create_inventory_location)
            .service(get_all_inventory_locations),
    );
}

#[post("")]
async fn create_inventory_location(
    pgpool: web::Data<Pool<Postgres>>,
    body: web::Json<InventoryLocationSerial>,
) -> impl Responder {
    let inventory_location = body.into_inner();
    let Ok(inventory_location) = InventoryLocationEntity::try_from_serial(&inventory_location)
    else {
        return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).finish();
    };

    let result: PgQueryResult = unwrap_result_else_500!(
        inventory_location_db::create_inventory_location(&pgpool, inventory_location).await
    );
    HttpResponseBuilder::new(StatusCode::CREATED).body(result.rows_affected().to_string())
}

#[get("")]
async fn get_all_inventory_locations(pgpool: web::Data<Pool<Postgres>>) -> impl Responder {
    let inventory_locations: Vec<InventoryLocationEntity> = unwrap_result_else_500!(
        inventory_location_db::get_all_inventory_locations(&pgpool).await
    );

    inventory_locations
        .iter()
        .map(|inventory_location| InventoryLocationEntity::to_serial(inventory_location))
        .collect::<Vec<InventoryLocationSerial>>()
        .to_http_response()
}
