use crate::admin::{page, split};
use actix_web::web;
use actix_web::web::ServiceConfig;
use maud::{html, Markup};

pub const RELATIVE_PATH: &str = "/admin/product";

pub fn configurer(config: &mut ServiceConfig) {
    config.route("/product", web::get().to(render));
}

async fn render() -> Markup {
    html! {
        (page::page(
            split::split(html!("left side"), html!("right side"))
        ))
    }
}
