use super::*;
use crate::object::JsonHttpResponse;
use crate::{
    unwrap_option_else_404, unwrap_result_else_400, unwrap_result_else_500, ShopModel, ShopSerial,
};
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
pub fn configurer(config: &mut ServiceConfig) {
    config.service(
        web::scope("/item_attribute")
            .route("", web::post().to(create_item_attribute))
            .route("/{item_id}", web::get().to(get_all_item_attributes))
            .route("/{item_id}/{key}", web::get().to(get_item_attribute))
            .route("/{item_id}/{key}", web::delete().to(delete_item_attribute)),
    );
}

async fn get_item_attribute(
    pgpool: web::Data<PgPool>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (item_id, key) = path.into_inner();
    let item_id = unwrap_result_else_400! {Uuid::try_parse(item_id.as_str())};

    let item_attribute = unwrap_result_else_500!(
		item_attribute_db::get_item_attribute(&pgpool, &item_id, &key).await
	);

    unwrap_option_else_404!(item_attribute)
        .to_serial()
        .to_http_response()
}

async fn get_all_item_attributes(
    pgpool: web::Data<PgPool>,
    path: web::Path<String>,
) -> impl Responder {
    let item_id = unwrap_result_else_400!(Uuid::try_parse(path.into_inner().as_str()));
    unwrap_result_else_500!(item_attribute_db::get_all_item_attributes(&pgpool, &item_id).await)
        .iter()
        .map(|attr| attr.to_serial())
        .collect::<Vec<ItemAttributeSerial>>()
        .to_http_response()
}

async fn create_item_attribute(
    pgpool: web::Data<PgPool>,
    path: web::Json<ItemAttributeSerial>,
) -> impl Responder {
    let item_attribute = unwrap_result_else_400!(path.into_inner().try_to_model());
    let query_result = unwrap_result_else_500!(
		item_attribute_db::create_item_attribute(&pgpool, &item_attribute).await
	);
    HttpResponse::Ok().body(query_result.rows_affected().to_string())
}

async fn delete_item_attribute(
    pgpool: web::Data<PgPool>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (item_id, key) = path.into_inner();
    let item_id = unwrap_result_else_400!(Uuid::try_parse(item_id.as_str()));
    let query_result = unwrap_result_else_500!(
		item_attribute_db::delete_item_attribute(&pgpool, &item_id, &key).await
	);
    HttpResponse::Ok().body(query_result.rows_affected().to_string())
}
