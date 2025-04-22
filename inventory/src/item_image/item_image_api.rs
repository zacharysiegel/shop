use super::*;
use crate::item_image::ItemImageSerial;
use crate::server::JsonHttpResponse;
use crate::{ShopEntity, ShopModel, ShopSerial};
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

pub fn configurer(config: &mut ServiceConfig) {
    config.service(
        web::scope("/item_image")
            .route("", web::post().to(create_item_image))
            .route("/{item_image_id}", web::get().to(get_item_image))
    );
}

async fn get_item_image(
    pgpool: web::Data<PgPool>,
    item_image_id: web::Path<String>,
) -> impl Responder {
    let Ok(item_image_id) = Uuid::try_parse(item_image_id.into_inner().as_str()) else {
        return HttpResponse::BadRequest().finish();
    };

    let Ok(item_image) = item_image_db::get_item_image(&pgpool, item_image_id).await else {
        return HttpResponse::InternalServerError().finish();
    };

    let item_image = match item_image {
        None => {
            return HttpResponse::NotFound().finish();
        }
        Some(item_image) => item_image.try_to_model(),
    };

    match item_image {
        Ok(item) => item.to_serial().to_http_response(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn create_item_image(
    pgpool: web::Data<PgPool>,
    item_image: web::Json<ItemImageSerial>,
) -> impl Responder {
    let Ok(item_image) = item_image.into_inner().try_to_model() else {
        return HttpResponse::BadRequest().finish();
    };

    let query_result = item_image_db::create_item_image(&pgpool, item_image).await;
    match query_result {
        Ok(query_result) => {
            HttpResponse::Ok().body(query_result.rows_affected().to_string())
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// todo: macro to retrieve uuid and return bad request on failure
// todo: macro to extract Option<Result<ShopEntity, E>> and return HttpResponse::NotFound().finish() if None
// todo: macro to construct this match expression from a Result<PgQueryResult, sqlx::Error>
