use actix_web::http::StatusCode;
use actix_web::web::ServiceConfig;
use actix_web::{HttpResponseBuilder, web};

pub fn configuration(config: &mut ServiceConfig) -> () {
    config //
        .route(
            "/test",
            web::get().to(|| async { HttpResponseBuilder::new(StatusCode::OK).body("test body") }),
        );
}
