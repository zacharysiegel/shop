use super::*;
use crate::category::CategorySerial;
use crate::error::ShopError;
use crate::item::{Item, ItemSerial};
use crate::object::JsonHttpResponse;
use crate::pagination::{pagination_guard, KeysetPaginationOptionsForStr};
use crate::{unwrap_result_else_400, unwrap_result_else_500, ShopModel};
use actix_web::guard::fn_guard;
use actix_web::http::StatusCode;
use actix_web::{guard, web, HttpResponse, HttpResponseBuilder, Responder};
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;
use uuid::Uuid;

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/product")
            .route("", web::get()
                .guard(fn_guard(pagination_guard))
                .to(get_all_products_paged_display_name))
            .route("", web::get().to(get_all_products))
            .route("", web::post()
                .guard(guard::Header("content-type", "application/json"))
                .to(create_product_json))
            .route("/{product_id}", web::get().to(get_product))
            .route("/{product_id}/category", web::get().to(get_product_categories))
            .route("/{product_id}/category/{category_id}", web::post().to(create_product_category_association))
            .route("/{product_id}/category/{category_id}", web::delete().to(delete_product_category_association))
            .route("/{product_id}/item", web::get().to(get_product_items)),
    );
}

async fn get_all_products(
    pgpool: web::Data<PgPool>,
) -> impl Responder {
    let product_entity_vec = unwrap_result_else_500!(product_db::get_all_products(&pgpool.into_inner()).await);
    product_entity_vec
        .iter()
        .map(|model| model.to_serial())
        .collect::<Vec<ProductSerial>>()
        .to_http_response()
}

async fn get_all_products_paged_display_name<'a>(
    pgpool: web::Data<PgPool>,
    query: web::Query<KeysetPaginationOptionsForStr<'a>>,
) -> impl Responder {
    let query_result = product_db::get_all_products_paged_display_name(
        &pgpool.into_inner(),
        unwrap_result_else_400!(query.into_inner().validated()),
    ).await;

    let pagination_response = unwrap_result_else_500!(query_result);
    pagination_response
        .iter()
        .map(|model| model.to_serial())
        .collect::<Vec<ProductSerial>>()
        .to_http_response()
}

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
async fn create_product_json(
    pgpool: web::Data<PgPool>,
    body: web::Json<ProductSerial>,
) -> impl Responder {
    create_product(pgpool, body.into_inner()).await
}

async fn create_product(
    pgpool: web::Data<PgPool>,
    product_serial: ProductSerial,
) -> HttpResponse {
    let Ok(product) = ProductEntity::try_from_serial(&product_serial) else {
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
