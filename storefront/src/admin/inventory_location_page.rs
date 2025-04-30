use crate::admin::api::wrapped_get;
use crate::admin::structure::{form, page, split};
use actix_web::web;
use actix_web::web::ServiceConfig;
use inventory::inventory_location::InventoryLocationSerial;
use maud::{html, Markup};

pub const RELATIVE_PATH: &str = "/admin/inventory_location";

pub fn configurer(config: &mut ServiceConfig) {
    config.route("/inventory_location", web::get().to(render));
}

async fn render() -> Markup {
    page::page(
        Some("Inventory location"),
        split::split(left().await, right()),
    )
}


async fn left() -> Markup {
    let elements: Vec<InventoryLocationSerial> = match wrapped_get("/inventory_location").await {
        Ok(elements) => elements,
        Err(markup) => return markup,
    };

    html! {
        h2 { "All inventory locations" }
        ol {
            @if elements.is_empty() {
                p { "None" }
            }
            @for element in &elements {
                li {
                    (format!("{:#?}", element))
                }
            }
        }
    }
}

fn right() -> Markup {
    form::form("Create inventory location", "/inventory_location", html! {
        label {
            "Display name"
            input type="text" name="display_name";
        }
        label {
            "Internal name"
            input type="text" name="internal_name";
        }
        input type="submit";
    })
}
