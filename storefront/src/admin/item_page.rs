use crate::admin::structure::{form, page, split};
use actix_web::web;
use actix_web::web::ServiceConfig;
use maud::{html, Markup};
use reqwest::Method;

pub const RELATIVE_PATH: &str = "/admin/item";

pub fn configurer(config: &mut ServiceConfig) {
    config.route("/item", web::get().to(render));
}

async fn render() -> Markup {
    page::page(
        Some("Item"),
        split::split(left(), right())
    )
}

fn left() -> Markup {
    html!()
}

fn right() -> Markup {
    form::form("Create item", "/item", Method::POST, html! {
        
    })
}
