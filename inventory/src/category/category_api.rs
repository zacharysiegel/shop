use super::*;
use crate::object::JsonHttpResponse;
use crate::{unwrap_result_else_500, ShopModel};
use actix_web::http::StatusCode;
use actix_web::{get, post, web, HttpResponseBuilder, Responder};
use sqlx::{PgPool, Pool, Postgres};
use uuid::Uuid;

pub const SCOPE_PATH: &str = "/category";

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(
        web::scope(SCOPE_PATH)
            .configure(configurer_public)
            .service(create_category)
    );
}

pub fn configurer_public(config: &mut web::ServiceConfig) {
    config
        .service(get_all_categories)
        .service(get_category)
    ;
}

#[get("")]
async fn get_all_categories(pgpool: web::Data<PgPool>) -> impl Responder {
    let all_categories: Vec<CategoryEntity> = unwrap_result_else_500!(
        category_db::get_all_categories(pgpool.get_ref()).await
    );
    all_categories
        .iter()
        .map(|category| category.to_serial())
        .collect::<Vec<CategorySerial>>()
        .to_http_response()
}

#[get("/{category_id}")]
async fn get_category(pgpool: web::Data<PgPool>, category_id: web::Path<String>) -> impl Responder {
    let Ok(category_id) = Uuid::try_parse(category_id.into_inner().as_str()) else {
        return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).finish();
    };

    let category: Option<CategoryEntity> = unwrap_result_else_500!(
        category_db::get_category(pgpool.get_ref(), category_id).await
    );
    category
        .map(|category| category.to_serial().to_http_response())
        .unwrap_or(HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish())
}

#[post("")]
async fn create_category(
    pgpool: web::Data<Pool<Postgres>>,
    body: web::Json<CategorySerial>,
) -> impl Responder {
    let Ok(category) = CategoryEntity::try_from_serial(&body.into_inner()) else {
        return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).finish();
    };

    let result = unwrap_result_else_500!(
        category_db::create_category(&pgpool, category).await
    );
    HttpResponseBuilder::new(StatusCode::CREATED)
        .body(result.rows_affected().to_string())
}
