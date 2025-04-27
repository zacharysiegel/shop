use crate::admin::item::create_item;
use crate::admin::structure::page;
use actix_web::web;
use actix_web::web::ServiceConfig;
use maud::{html, Markup};

pub const RELATIVE_PATH: &str = "/admin/item";

pub fn configurer(config: &mut ServiceConfig) {
    config.route("/item", web::get().to(render));
}

async fn render() -> Markup {
    page::page(
        html!(
            div {
                "<item page>"
            }
            (create_item::create_item())
        )
    )
}
