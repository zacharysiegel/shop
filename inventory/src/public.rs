use crate::category::category_api;
use actix_web::web;

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(web::scope("/public")
        .service(web::scope("/category")
            .service(category_api::get_all_categories)
            .service(category_api::get_category))
    );
}