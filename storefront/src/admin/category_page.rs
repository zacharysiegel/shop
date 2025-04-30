use crate::admin::api::wrapped_get;
use crate::admin::structure::{form, page, split};
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
    let elements: Vec<CategorySerial> = match wrapped_get("/category").await {
        Ok(elements) => elements,
        Err(markup) => return markup,
    };

    html! {
        h2 { "All categories" }
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
