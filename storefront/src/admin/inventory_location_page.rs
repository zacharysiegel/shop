use crate::admin::structure::{form, page, split};
use crate::registry::REGISTRY;
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
    let elements: Vec<InventoryLocationSerial> = match get_all_inventory_locations().await {
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

// todo: create a generic api call function once we flush out the pattern
async fn get_all_inventory_locations() -> Result<Vec<InventoryLocationSerial>, Markup> {
    let result = REGISTRY.http_client
        .get(format!("{}{}", REGISTRY.remote_url, "/inventory_location"))
        .send()
        .await;
    let response = match result {
        Ok(response) => response,
        Err(error) => {
            return Err(html!((format!("Error: {:#}", error))));
        }
    };
    let vec = match response.json::<Vec<InventoryLocationSerial>>().await {
        Ok(element) => element,
        Err(error) => {
            return Err(html!((format!("Error: {:#}", error))));
        }
    };
    Ok(vec)
}
