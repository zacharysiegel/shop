use crate::admin::product_page;
use crate::admin::structure::{form, page, split};
use actix_web::web;
use actix_web::web::ServiceConfig;
use maud::{html, Markup};
use reqwest::Method;

pub const RELATIVE_PATH: &str = "/admin/product/{product_id}/item";

pub fn configurer(config: &mut ServiceConfig) {
    config
        .route("/product/{product_id}/item", web::get().to(render))
    ;
}

async fn render(
    product_id: web::Path<String>,
) -> Markup {
    page::page(
        &vec!(
            (product_page::RELATIVE_PATH, "Product"),
            (&RELATIVE_PATH.replace("{product_id}", product_id.to_string().as_str()), "Item"),
        ),
        split::split(left(), right()),
    )
}

fn left() -> Markup {
    html!()
}

fn right() -> Markup {
    form::form("Create item", "/item", Method::POST, html! {
        
    })
}
