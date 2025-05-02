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
pub const HEADINGS: [&str; 5] = ["id", "inventory location", "condition", "status", "price (\u{00A2})"];

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
        h2 { (format!("Items for product \"{}\"", product.display_name)) }
        @if item_vec.is_empty() {
            p { "None" }
        } @else {
            (table(&item_vec))
        }
    )
}

fn right() -> Markup {
    form::form("Create item", "/item", Method::POST, html! {
        
    })
}

fn table(elements: &Vec<ItemSerial>) -> Markup {
    html! {
        table {
            thead {
                @for heading in HEADINGS {
                    th { (heading) }
                }
            }
            tbody {
                @for element in elements {
                    tr {
                        td { (element.id) }
                        td { (element.inventory_location_id) } // todo: get display_name
                        td { (element.condition) } // todo: get readable value (debug?)
                        td { (element.status) } // todo: get readable value
                        td { (element.price_cents) }
                    }
                }
            }
        }
    }
}
