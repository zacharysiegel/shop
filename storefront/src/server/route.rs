use crate::server::state::AppState;
use actix_web::http::Method;
use actix_web::web::ServiceConfig;
use actix_web::{HttpResponse, Responder, get, web};

pub fn configuration(service_config: &mut ServiceConfig) -> () {
    service_config.service(
        web::scope("/storefront")
            .route("/me", web::route().method(Method::GET).to(me))
            .service(hello),
    );
}

#[get("/hello")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(format!("[{}] Hello world!", data.artifact_id))
}

async fn me() -> impl Responder {
    HttpResponse::Ok().body("Zachary Siegel")
}
