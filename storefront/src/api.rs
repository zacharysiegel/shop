mod me;

use actix_web::http::Method;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn configuration(service_config: &mut ServiceConfig) -> () {
    service_config //
        .route("/me", web::route().method(Method::GET).to(me::me));
}
