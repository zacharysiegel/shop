use crate::customer::{customer_db, CustomerSerial};
use crate::{unwrap_option_else_404, unwrap_result_else_400, unwrap_result_else_500, ShopEntity, ShopModel, ShopSerial};
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use crate::server::JsonHttpResponse;

pub fn configurer(config: &mut ServiceConfig) {
    config.service(
        web::scope("/customer")
            .route("", web::post().to(create_customer))
            .route("/{id}", web::get().to(get_customer))
    );
}

async fn create_customer(
    pgpool: web::Data<PgPool>,
    customer: web::Json<CustomerSerial>,
) -> impl Responder {
    let customer = unwrap_result_else_400!(customer.into_inner().try_to_model());
    let query_result = unwrap_result_else_500!(customer_db::create_customer(&pgpool, &customer.to_entity()).await);

    HttpResponse::Ok().body(query_result.rows_affected().to_string())
}

async fn get_customer(
    pgpool: web::Data<PgPool>,
    id: web::Path<String>,
) -> impl Responder {
    let id = unwrap_result_else_400!(Uuid::try_parse(id.into_inner().as_str()));
    let customer = unwrap_result_else_500!(customer_db::get_customer(&pgpool, &id).await);
    let customer = unwrap_option_else_404!(customer);
    let customer = unwrap_result_else_500!(customer.try_to_model());

    customer
        .to_serial()
        .to_http_response()
}
