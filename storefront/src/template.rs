use actix_web::web;
use actix_web::web::ServiceConfig;

pub mod index;

pub fn configuration(config: &mut ServiceConfig) -> () {
    config //
        .route("/index.html", web::get().to(index::render));
}
