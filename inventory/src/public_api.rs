use crate::category::category_api;
use crate::product::product_api;
use actix_web::web;

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(web::scope("/public")
        .service(web::scope(category_api::SCOPE_PATH)
            .configure(category_api::configurer_public))
        .service(web::scope(product_api::SCOPE_PATH)
            .configure(product_api::configurer_public))
    );
}