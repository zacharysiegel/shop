use super::*;
use crate::item_audit::{item_audit_db, ItemAuditModel, ItemAuditSerial};
use crate::item_image::ItemImageSerial;
use crate::label::LabelSerial;
use crate::object::JsonHttpResponse;
use crate::{unwrap_result_else_400, unwrap_result_else_500, ShopEntity, ShopModel, ShopSerial};
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, HttpResponseBuilder, Responder};
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;
use uuid::Uuid;

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/item")
            .route("", web::post().to(create_item))
            .route("/{item_id}", web::get().to(get_item))
            .route("/{item_id}/image", web::get().to(get_all_item_images))
            .route("/{item_id}/label", web::get().to(get_all_item_labels))
            .route(
                "/{item_id}/label/{label_id}",
                web::post().to(create_item_label_association),
            )
            .route(
                "/{item_id}/label/{label_id}",
                web::delete().to(delete_item_label_association),
            )
            .route(
                "/{item_id}/item_audit",
                web::get().to(get_all_item_item_audits),
            )
            .route("/{item_id}/listing", web::get().to(get_all_item_listings)),
    )
        .route("/item_condition", web::get().to(get_all_item_conditions));
}

// todo: refactor to use unwrap_http macros

async fn get_item(pgpool: web::Data<PgPool>, item_id: web::Path<String>) -> impl Responder {
    let Ok(item_id) = Uuid::try_parse(item_id.into_inner().as_str()) else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
    };

    let Ok(item) = item_db::get_item(&pgpool, &item_id).await else {
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

async fn create_item(pgpool: web::Data<PgPool>, item: web::Json<ItemSerial>) -> impl Responder {
    let Ok(item) = item.into_inner().try_to_model() else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
    };

    let query_result = item_db::create_item(&pgpool, &item.to_entity()).await;
    match query_result {
        Ok(query_result) => {
            HttpResponseBuilder::new(StatusCode::OK).body(query_result.rows_affected().to_string())
        }
        Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
    }
}

async fn get_all_item_images(
    pgpool: web::Data<PgPool>,
    item_id: web::Path<String>,
) -> impl Responder {
    let Ok(item_id) = Uuid::try_parse(item_id.into_inner().as_str()) else {
        return HttpResponse::BadRequest().finish();
    };

    let result = crate::item_image::item_image_db::get_all_item_images(&pgpool, item_id).await;
    let Ok(item_images) = result else {
        return HttpResponse::InternalServerError().finish();
    };

    item_images
        .iter()
        .map(|item_image| item_image.to_serial())
        .collect::<Vec<ItemImageSerial>>()
        .to_http_response()
}

async fn get_all_item_labels(
    pgpool: web::Data<PgPool>,
    item_id: web::Path<String>,
) -> impl Responder {
    let item_id = unwrap_result_else_400!(Uuid::parse_str(&item_id.into_inner().as_str()));
    unwrap_result_else_500!(item_db::get_all_item_labels(&pgpool, &item_id).await)
        .iter()
        .map(|label| label.to_serial())
        .collect::<Vec<LabelSerial>>()
        .to_http_response()
}

async fn create_item_label_association(
    pgpool: web::Data<PgPool>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (item_id, label_id) = path.into_inner();
    let item_id = unwrap_result_else_400!(Uuid::parse_str(&item_id));
    let label_id = unwrap_result_else_400!(Uuid::parse_str(&label_id));

    let query_result: PgQueryResult = unwrap_result_else_500!(
		item_db::create_item_label_association(&pgpool, &item_id, &label_id).await
	);
    HttpResponse::Ok().body(query_result.rows_affected().to_string())
}

async fn delete_item_label_association(
    pgpool: web::Data<PgPool>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (item_id, label_id) = path.into_inner();
    let item_id = unwrap_result_else_400!(Uuid::parse_str(&item_id));
    let label_id = unwrap_result_else_400!(Uuid::parse_str(&label_id));

    let query_result: PgQueryResult = unwrap_result_else_500!(
		item_db::delete_item_label_association(&pgpool, &item_id, &label_id).await
	);
    HttpResponse::Ok().body(query_result.rows_affected().to_string())
}

async fn get_all_item_item_audits(
    pgpool: web::Data<PgPool>,
    item_id: web::Path<String>,
) -> impl Responder {
    let item_id = unwrap_result_else_400!(Uuid::parse_str(&item_id));

    let item_audit_entity_vec =
        unwrap_result_else_500!(item_audit_db::get_all_item_item_audits(&pgpool, &item_id).await);
    let mut item_audit_model_vec: Vec<ItemAuditModel> = Vec::new();
    for item_audit_entity in item_audit_entity_vec {
        item_audit_model_vec.push(unwrap_result_else_500!(item_audit_entity.try_to_model()));
    }

    item_audit_model_vec
        .iter()
        .map(|audit_model| audit_model.to_serial())
        .collect::<Vec<ItemAuditSerial>>()
        .to_http_response()
}

async fn get_all_item_listings(
    pgpool: web::Data<PgPool>,
    item_id: web::Path<String>,
) -> impl Responder {
    let item_id = unwrap_result_else_400!(Uuid::try_parse(item_id.into_inner().as_str()));
    let listing_vec =
        unwrap_result_else_500!(item_db::get_all_item_listings(&pgpool, &item_id).await);

    let mut listing_serial_vec = Vec::new();
    for listing_entity in listing_vec {
        let listing_model = unwrap_result_else_500!(listing_entity.try_to_model());
        listing_serial_vec.push(listing_model.to_serial());
    }

    listing_serial_vec.to_http_response()
}

async fn get_all_item_conditions() -> impl Responder {
    let body: String = ItemCondition::get_json_spec();
    HttpResponseBuilder::new(StatusCode::OK).body(body)
}
