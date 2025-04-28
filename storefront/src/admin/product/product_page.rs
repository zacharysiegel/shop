use crate::admin::structure::form;
use crate::admin::structure::{page, split};
use crate::registry::REGISTRY;
use actix_web::web;
use actix_web::web::ServiceConfig;
use inventory::product::ProductSerial;
use maud::{html, Markup};
use inventory::pagination;
use inventory::pagination::{KeysetPaginationOptionsForStr, SortOrder};
use crate::admin::structure::error_text::error_text;

pub const RELATIVE_PATH: &str = "/admin/product";

pub fn configurer(config: &mut ServiceConfig) {
    config.route("/product", web::get().to(render));
}

async fn render() -> Markup {
    page::page(
        Some("Product"),
        split::split(left().await, right()),
    )
}

async fn left() -> Markup {
    let product_vec: Vec<ProductSerial> = match get_all_products_paged_display_name().await {
        Ok(vec) => vec,
        Err(markup) => return markup,
    };

    html! {
        h2 { "All products" }
        ol {
            @if product_vec.is_empty() {
                p { "None" }
            }
            @for product in &product_vec {
                li {
                    (format!("Product: {:#?}", product))
                }
            }
        }
    }
}

fn right() -> Markup {
    form::form("Create product", "/product", html! {
        label {
            "Display name"
            input type="text" name="display_name";
        }
        label {
            "Internal name"
            input type="text" name="internal_name";
        }
        label {
            "Universal product code (optional)"
            input type="text" name="upc";
        }
        label {
            "Release date (optional)"
            input type="date" name="release_date";
        }
        input type="submit";
    })
}

async fn get_all_products_paged_display_name() -> Result<Vec<ProductSerial>, Markup> {
    let pagination_options = {
        let mut pagination_options = KeysetPaginationOptionsForStr::default();
        pagination_options.page_size = 3;
        let pagination_options = match serde_urlencoded::to_string(pagination_options) {
            Ok(pagination_options) => pagination_options,
            Err(error) => return Err(error_text(error)),
        };
        pagination_options
    };

    let result = REGISTRY.http_client.get(format!("{}{}?{}", REGISTRY.remote_url, "/product", pagination_options))
        .send()
        .await;
    let response = match result {
        Ok(response) => response,
        Err(error) => {
            return Err(html!((format!("Error: {:#}", error))));
        }
    };
    let product_vec = match response.json::<Vec<ProductSerial>>().await {
        Ok(product) => product,
        Err(error) => {
            return Err(error_text(error));
        }
    };
    Ok(product_vec)
}
