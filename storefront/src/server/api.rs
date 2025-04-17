use actix_web::http::Method;
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse, Responder};

pub fn configuration(service_config: &mut ServiceConfig) -> () {
    service_config //
        .route("/me", web::route().method(Method::GET).to(me));
}

async fn me() -> impl Responder {
    HttpResponse::Ok().body("Zachary Siegel")
}
