use crate::unwrap_result_else_500;
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse, Responder};

pub fn configurer(config: &mut ServiceConfig) {
    config.service(
        web::scope("/ebay")
            .route("/auth/application_token", web::get().to(get_application_token))
    );
}

async fn get_application_token() -> impl Responder {
    let token: String = unwrap_result_else_500!(super::ebay_client::get_application_token().await);
    HttpResponse::Ok().body(token)
}
