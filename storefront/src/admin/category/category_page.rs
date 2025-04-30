use crate::admin::structure::{form, page, split};
use crate::registry::REGISTRY;
use actix_web::web;
use actix_web::web::ServiceConfig;
use inventory::category::CategorySerial;
use maud::{html, Markup};

pub const RELATIVE_PATH: &str = "/admin/category";

pub fn configurer(config: &mut ServiceConfig) {
    config.route("/category", web::get().to(render));
}

async fn render() -> Markup {
    page::page(
        Some("Category"),
        split::split(left().await, right()),
    )
}


async fn left() -> Markup {
    let elements: Vec<CategorySerial> = match get_all_categories().await {
        Ok(elements) => elements,
        Err(markup) => return markup,
    };

    html! {
        h2 { "All categories" }
        ol .tree {
            @if elements.is_empty() {
                p { "None" }
            }
            @for category in &elements {
                li {
                    (format!("Category: {:#?}", category))
                }
            }
        }
    }
}

fn right() -> Markup {
    form::form("Create category", "/category", html! {
        label {
            "Display name"
            input type="text" name="display_name";
        }
        label {
            "Internal name"
            input type="text" name="internal_name";
        }
        label {
            "Parent ID (optional)"
            input type="text" name="parent_id";
        }
        input type="submit";
    })
}

// todo: create a generic api call function once we flush out the pattern
async fn get_all_categories() -> Result<Vec<CategorySerial>, Markup> {
    let result = REGISTRY.http_client.get(format!("{}{}", REGISTRY.remote_url, "/category"))
        .send()
        .await;
    let response = match result {
        Ok(response) => response,
        Err(error) => {
            return Err(html!((format!("Error: {:#}", error))));
        }
    };
    let vec = match response.json::<Vec<CategorySerial>>().await {
        Ok(element) => element,
        Err(error) => {
            return Err(html!((format!("Error: {:#}", error))));
        }
    };
    Ok(vec)
}
