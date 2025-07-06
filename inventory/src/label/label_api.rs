use crate::label::{label_db, LabelEntity, LabelSerial};
use crate::object::JsonHttpResponse;
use crate::{unwrap_option_else_404, unwrap_result_else_500, ShopModel, ShopSerial};
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

    let label: LabelEntity = unwrap_option_else_404!(
        unwrap_result_else_500!(label_db::get_label(&pgpool, label_id).await)
    );
    label.to_serial().to_http_response()
}

async fn get_all_labels(pgpool: web::Data<PgPool>) -> impl Responder {
    let labels = unwrap_result_else_500!(
        label_db::get_all_labels(&pgpool).await
    );

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
