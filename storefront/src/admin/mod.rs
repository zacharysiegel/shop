mod index;
mod page;
mod item;
mod product;

use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn configurer(config: &mut ServiceConfig) -> () {
    config
        .service(web::scope("/admin")
            .route("", web::get().to(index::render))
            .route("/index.html", web::get().to(index::render))
            .route("/item", web::get().to(item::render))
        )
    ;
}
