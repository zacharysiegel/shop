use super::*;
use crate::{unwrap_result_else_400, unwrap_result_else_500, ShopModel};
use crate::category::CategorySerial;
use crate::error::ShopError;
use crate::item::{Item, ItemSerial};
use crate::server::JsonHttpResponse;
use actix_web::http::StatusCode;
use actix_web::{HttpResponseBuilder, Responder, get, post, web, delete, HttpResponse};
use sqlx::PgPool;
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/product")
            .service(get_product)
            .service(create_product)
            .service(get_product_categories)
            .service(create_product_category_association)
            .service(delete_product_category_association)
            .service(get_product_items),
    );
}

#[get("/{product_id}")]
async fn get_product(pgpool: web::Data<PgPool>, product_id: web::Path<String>) -> impl Responder {
    let Ok(product_id) = Uuid::try_parse(product_id.into_inner().as_str()) else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
    };

    let Ok(product) = product_db::get_product(&pgpool, &product_id).await else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
    };

    product
        .map(|product| product.to_serial().to_http_response())
        .unwrap_or(HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish())
}

// todo: restrict to authenticated administrator
#[post("")]
async fn create_product(
    pgpool: web::Data<PgPool>,
    body: web::Json<ProductSerial>,
) -> impl Responder {
    let Ok(product) = ProductEntity::try_from_serial(&body.into_inner()) else {
        return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).finish();
    };

    let result: Result<PgQueryResult, sqlx::Error> =
        product_db::create_product(&pgpool, &product).await;
    match result {
        Ok(query_result) => HttpResponseBuilder::new(StatusCode::CREATED)
            .body(query_result.rows_affected().to_string()),
        Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
    }
}

#[get("/{product_id}/category")]
async fn get_product_categories(
    pgpool: web::Data<PgPool>,
    product_id: web::Path<String>,
) -> impl Responder {
    let Ok(product_id) = Uuid::try_parse(product_id.into_inner().as_str()) else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
    };

    let Ok(product_categories) = product_db::get_product_categories(&pgpool, &product_id).await
    else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
    };

    product_categories
        .iter()
        .map(|category| category.to_serial())
        .collect::<Vec<CategorySerial>>()
        .to_http_response()
}

// todo: restrict to authenticated administrator
#[post("/{product_id}/category/{category_id}")]
async fn create_product_category_association(
    pgpool: web::Data<PgPool>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (product_id, category_id) = path.into_inner();
    let Ok(product_id) = Uuid::try_parse(product_id.as_str()) else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
    };
    let Ok(category_id) = Uuid::try_parse(category_id.as_str()) else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
    };

    let result =
        product_db::create_product_category_association(&pgpool, &product_id, &category_id).await;
    match result {
        Ok(query_result) => HttpResponseBuilder::new(StatusCode::CREATED)
            .body(query_result.rows_affected().to_string()),
        Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
    }
}

#[delete("/{product_id}/category/{category_id}")]
async fn delete_product_category_association(
    pgpool: web::Data<PgPool>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (product_id, category_id) = path.into_inner();
    let product_id = unwrap_result_else_400!(Uuid::try_parse(product_id.as_str()));
    let category_id = unwrap_result_else_400!(Uuid::try_parse(category_id.as_str()));

    let query_result = unwrap_result_else_500!(
		product_db::delete_product_category_association(&pgpool, &product_id, &category_id).await
	);
    HttpResponse::Ok().body(query_result.rows_affected().to_string())
}

#[get("/{product_id}/item")]
async fn get_product_items(
    pgpool: web::Data<PgPool>,
    product_id: web::Path<String>,
) -> impl Responder {
    let Ok(product_id) = Uuid::try_parse(product_id.into_inner().as_str()) else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
    };

    let Ok(items) = product_db::get_all_product_items(&pgpool, &product_id).await else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
    };

    let item_serial_vec = items
        .iter()
        .map(|item| Item::try_from_entity(item))
        .map(|item_result| item_result.map(|item| item.to_serial()))
        .collect::<Result<Vec<ItemSerial>, ShopError>>();

    match item_serial_vec {
        Ok(item_serial_vec) => item_serial_vec.to_http_response(),
        Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
    }
}
