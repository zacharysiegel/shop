use super::*;
use crate::object::JsonHttpResponse;
use crate::ShopModel;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, HttpResponseBuilder, Responder};
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

    let result =
        inventory_location_db::create_inventory_location(&pgpool, inventory_location).await;
    match result {
        Ok(query_result) => HttpResponseBuilder::new(StatusCode::CREATED)
            .body(query_result.rows_affected().to_string()),
        Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
    }
}

#[get("")]
async fn get_all_inventory_locations(pgpool: web::Data<Pool<Postgres>>) -> impl Responder {
    let result = inventory_location_db::get_all_inventory_locations(&pgpool).await;
    let inventory_locations = match result {
        Ok(inventory_locations) => inventory_locations,
        Err(e) => {
            log::error!("Error fetching inventory locations; [{:#}];", e);
            return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
        }
    };

    inventory_locations
        .iter()
        .map(|inventory_location| InventoryLocationEntity::to_serial(inventory_location))
        .collect::<Vec<InventoryLocationSerial>>()
        .to_http_response()
}
