use super::*;
use crate::server::JsonHttpResponse;
use crate::{ShopEntity, ShopModel, ShopSerial};
use actix_web::http::StatusCode;
use actix_web::{get, post, web, HttpResponseBuilder, Responder};
use sqlx::PgPool;
use uuid::Uuid;

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(web::scope("/item").service(get_item).service(create_item));
}

#[get("/{item_id}")]
async fn get_item(pgpool: web::Data<PgPool>, item_id: web::Path<String>) -> impl Responder {
    let Ok(item_id) = Uuid::try_parse(item_id.into_inner().as_str()) else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
    };

    let Ok(item) = item_db::get_item(&pgpool, item_id).await else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
    };

    let item = match item {
        None => {
            return HttpResponseBuilder::new(StatusCode::NOT_FOUND).finish();
        }
        Some(item) => item.try_to_model(),
    };

    match item {
        Ok(item) => item.to_serial().to_http_response(),
        Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
    }
}

#[post("")]
async fn create_item(pgpool: web::Data<PgPool>, item: web::Json<ItemSerial>) -> impl Responder {
    let Ok(item) = item.into_inner().try_to_model() else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
    };

    let query_result = item_db::create_item(&pgpool, item.to_entity()).await;
    match query_result {
        Ok(query_result) => {
            HttpResponseBuilder::new(StatusCode::OK).body(query_result.rows_affected().to_string())
        }
        Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
    }
}
