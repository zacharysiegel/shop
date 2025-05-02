use crate::admin::api::wrapped_get;
use crate::admin::product_page;
use crate::admin::structure::{form, page, split};
use crate::unwrap_result_else_markup;
use actix_web::web;
use actix_web::web::ServiceConfig;
use inventory::item::ItemSerial;
use inventory::product::ProductSerial;
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
    let product_id = product_id.into_inner();
    page::page(
        &vec!(
            (product_page::RELATIVE_PATH, "Product"),
            (&RELATIVE_PATH.replace("{product_id}", &product_id), "Item"),
        ),
        split::split(left(&product_id).await, right()),
    )
}

async fn left(product_id: &String) -> Markup {
    let product: ProductSerial = unwrap_result_else_markup!(
        wrapped_get::<ProductSerial>(&format!("/product/{}", product_id)).await
    );
    let item_vec: Vec<ItemSerial> = unwrap_result_else_markup!(
        wrapped_get::<Vec<ItemSerial>>(&format!("/product/{}/item", product_id)).await
    );

    html!(
        h2 { "Items for product \"\"" }
    )
}

fn right() -> Markup {
    form::form("Create item", "/item", Method::POST, html! {
        
    })
}
