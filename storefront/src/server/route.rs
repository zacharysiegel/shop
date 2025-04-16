use crate::server::state::AppState;
use actix_web::web::ServiceConfig;
use actix_web::{get, web, HttpResponse, Responder};

pub fn conf(service_config: &mut ServiceConfig) -> () {
    service_config
        .service(hello)
        .route("/me", web::get().to(me));
}

#[get("/hello")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(format!("[{}] Hello world!", data.artifact_id))
}

async fn me() -> impl Responder {
    HttpResponse::Ok().body("Zachary Siegel")
}
