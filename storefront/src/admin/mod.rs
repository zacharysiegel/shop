use actix_web::http::StatusCode;
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponseBuilder};

pub fn configuration(config: &mut ServiceConfig) -> () {
    config
        .service(web::scope("/admin")
            .route(
                "/test",
                web::get().to(|| async { HttpResponseBuilder::new(StatusCode::OK).body("test body") }),
            )
        );
}
