use crate::label::{label_db, LabelSerial};
use crate::object::JsonHttpResponse;
use crate::{ShopModel, ShopSerial};
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

pub fn configurer(config: &mut ServiceConfig) {
    config.service(
        web::scope("/label")
            .route("", web::post().to(create_label))
            .route("", web::get().to(get_all_labels))
            .route("/{label_id}", web::get().to(get_label)),
    );
}

async fn get_label(pgpool: web::Data<PgPool>, label_id: web::Path<String>) -> impl Responder {
    let Ok(label_id) = Uuid::parse_str(label_id.into_inner().as_ref()) else {
        return HttpResponse::BadRequest().finish();
    };

    let Ok(label) = label_db::get_label(&pgpool, label_id).await else {
        return HttpResponse::InternalServerError().finish();
    };

    match label {
        None => HttpResponse::NotFound().finish(),
        Some(label) => label.to_serial().to_http_response(),
    }
}

async fn get_all_labels(pgpool: web::Data<PgPool>) -> impl Responder {
    let Ok(labels) = label_db::get_all_labels(&pgpool).await else {
        return HttpResponse::InternalServerError().finish();
    };

    labels
        .iter()
        .map(|label| label.to_serial())
        .collect::<Vec<LabelSerial>>()
        .to_http_response()
}

async fn create_label(pgpool: web::Data<PgPool>, label: web::Json<LabelSerial>) -> impl Responder {
    let result = label
        .into_inner()
        .try_to_model()
        .map(|label| label.to_entity()) // superfluous in this case
        .map(async |label_entity| label_db::create_label(&pgpool, &label_entity).await);

    let Ok(db_result) = result else {
        return HttpResponse::InternalServerError().finish();
    };
    let Ok(query_result) = db_result.await else {
        return HttpResponse::InternalServerError().finish();
    };

    HttpResponse::Ok().body(query_result.rows_affected().to_string())
}
