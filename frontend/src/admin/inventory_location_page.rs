use crate::admin::api::wrapped_get;
use crate::admin::structure::page::Page;
use crate::admin::structure::{form, page, split};
use crate::unwrap_result_else_markup;
use actix_web::web;
use actix_web::web::ServiceConfig;
use inventory::inventory_location::InventoryLocationSerial;
use maud::{html, Markup};
use reqwest::Method;

pub const PAGE: Page = Page {
    name: "Inventory location",
    relative_path: "/admin/inventory_location",
    configurer,
};

fn configurer(config: &mut ServiceConfig) {
    config.route("/inventory_location", web::get().to(render));
}

async fn render() -> Markup {
    page::page(
        &vec!(PAGE.into()),
        Markup::default(),
        split::split(left().await, right()),
    )
}


async fn left() -> Markup {
    let elements: Vec<InventoryLocationSerial> = unwrap_result_else_markup!(
        wrapped_get("/inventory_location").await
    );

    html! {
        h2 { "Inventory locations" }
        @if elements.is_empty() {
            p { "None" }
        } @else {
            (table(elements))
        }
    }
}

fn right() -> Markup {
    form::form(Some("Create inventory location"), "/inventory_location", Method::POST, html! {
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

const HEADINGS: [&str; 3] = ["id", "display_name", "internal_name"];
fn table(elements: Vec<InventoryLocationSerial>) -> Markup {
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
                        td { (element.display_name) }
                        td { (element.internal_name) }
                    }
                }
            }
        }
    }
}
