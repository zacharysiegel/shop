use super::*;
use crate::object::JsonHttpResponse;
use crate::{
    unwrap_option_else_404, unwrap_result_else_400, unwrap_result_else_500, ShopModel, ShopSerial,
};
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

pub fn configurer(config: &mut ServiceConfig) {
    config.service(
        web::scope("/metric/counter")
            .route("", web::post().to(create_metric_counter))
            .route("/{id}", web::get().to(get_metric_counter))
            .route("/{id}", web::put().to(increment_metric_counter)),
    );
}

async fn create_metric_counter(
    pgpool: web::Data<PgPool>,
    body: web::Json<MetricCounterSerial>,
) -> impl Responder {
    let metric_counter = unwrap_result_else_400!(body.into_inner().try_to_model());
    let result = unwrap_result_else_500!(
		metric_counter_db::create_metric_counter(&pgpool, &metric_counter).await
	);

    HttpResponse::Ok().body(result.rows_affected().to_string())
}

async fn get_metric_counter(pgpool: web::Data<PgPool>, id: web::Path<String>) -> impl Responder {
    let id = unwrap_result_else_400!(Uuid::try_parse(id.into_inner().as_str()));
    let metric_counter =
        unwrap_result_else_500!(metric_counter_db::get_metric_counter(&pgpool, &id).await);

    unwrap_option_else_404!(metric_counter)
        .to_serial()
        .to_http_response()
}

async fn increment_metric_counter(
    pgpool: web::Data<PgPool>,
    id: web::Path<String>,
    params: web::Query<IncrementMetricCounterQueryParams>,
) -> impl Responder {
    let id: Uuid = unwrap_result_else_400!(Uuid::try_parse(id.into_inner().as_str()));
    let increment: i64 = params.increment.unwrap_or(1);
    let query_result = unwrap_result_else_500!(
		metric_counter_db::increment_metric_counter(&pgpool, &id, &increment).await
	);

    HttpResponse::Ok().body(query_result.rows_affected().to_string())
}

#[derive(Debug, Deserialize)]
struct IncrementMetricCounterQueryParams {
    pub increment: Option<i64>,
}
