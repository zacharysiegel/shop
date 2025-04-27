use crate::admin::structure::{form, page, split};
use actix_web::web;
use actix_web::web::ServiceConfig;
use maud::{html, Markup};

pub const RELATIVE_PATH: &str = "/admin/category";

pub fn configurer(config: &mut ServiceConfig) {
    config.route("/category", web::get().to(render));
}

async fn render() -> Markup {
    page::page(
        Some("Category"),
        split::split(left(), right())
    )
}

fn left() -> Markup {
    html!(
        h2 { "All categories" }
    )
}

fn right() -> Markup {
    form::form("Create category", "/category", html! {

    })
}
