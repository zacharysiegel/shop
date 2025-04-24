mod index;
mod page;

use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn configurer(config: &mut ServiceConfig) -> () {
    config
        .route("/index.html", web::get().to(index::render));
}
