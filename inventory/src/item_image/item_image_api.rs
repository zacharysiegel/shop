use super::*;
use crate::item_image::ItemImageSerial;
use crate::object::JsonHttpResponse;
use crate::{
    unwrap_option_else_404, unwrap_result_else_400, unwrap_result_else_500, ShopEntity, ShopModel,
    ShopSerial,
};
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

pub fn configurer(config: &mut ServiceConfig) {
    config.service(
        web::scope("/item_image")
            .route("", web::post().to(create_item_image))
            .route("/{item_image_id}", web::get().to(get_item_image)),
    );
}

async fn get_item_image(
    pgpool: web::Data<PgPool>,
    item_image_id: web::Path<String>,
) -> impl Responder {
    let item_image_id =
        unwrap_result_else_400!(Uuid::try_parse(item_image_id.into_inner().as_str()));

    let item_image =
        unwrap_result_else_500!(item_image_db::get_item_image(&pgpool, item_image_id).await);
    let item_image = unwrap_option_else_404!(item_image);

    unwrap_result_else_500!(item_image.try_to_model())
        .to_serial()
        .to_http_response()
}

async fn create_item_image(
    pgpool: web::Data<PgPool>,
    item_image: web::Json<ItemImageSerial>,
) -> impl Responder {
    let item_image = unwrap_result_else_400!(item_image.into_inner().try_to_model());

    let query_result = item_image_db::create_item_image(&pgpool, item_image).await;

    HttpResponse::Ok().body(
        unwrap_result_else_500!(query_result)
            .rows_affected()
            .to_string(),
    )
}
