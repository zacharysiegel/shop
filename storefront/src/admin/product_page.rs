use crate::admin;
use crate::admin::api::wrapped_get;
use crate::admin::structure::error_text::error_text;
use crate::admin::structure::form;
use crate::admin::structure::{page, split};
use actix_web::web::ServiceConfig;
use actix_web::{guard, web};
use admin::structure::pagination_control::pagination_control;
use inventory::pagination::{pagination_guard, KeysetPaginationOptionsForString, KeysetPaginationResultForString};
use inventory::product::ProductSerial;
use maud::{html, Markup};

pub const RELATIVE_PATH: &str = "/admin/product";

pub fn configurer(config: &mut ServiceConfig) {
    config
        .route("/product",
               web::get()
                   .guard(guard::fn_guard(pagination_guard))
                   .to(handle_paginated))
        .route("/product", web::get().to(handle_unpaginated))
    ;
}

async fn handle_unpaginated() -> Markup {
    render(None).await
}

async fn handle_paginated(
    query: web::Query<KeysetPaginationOptionsForString>,
) -> Markup {
    render(Some(query.into_inner())).await
}

async fn render(pagination_options: Option<KeysetPaginationOptionsForString>) -> Markup {
    page::page(
        Some("Product"),
        split::split(left(pagination_options).await, right()),
    )
}

async fn left(pagination_options: Option<KeysetPaginationOptionsForString>) -> Markup {
    let pagination_options = pagination_options.unwrap_or_default();
    let query_params = match serde_urlencoded::to_string(&pagination_options) {
        Ok(pagination_options) => pagination_options,
        Err(error) => return error_text(error),
    };

    let (product_vec, pagination_result) = match wrapped_get::<(Vec<ProductSerial>, KeysetPaginationResultForString)>(
        format!("/product?{}", query_params).as_str()
    ).await {
        Ok(response) => response,
        Err(markup) => return markup,
    };

    html! {
        h2 { "Products" }
        (pagination_control(RELATIVE_PATH, &pagination_options, &pagination_result))
        @if product_vec.is_empty() {
            p { "None" }
        } @else {
            (table(product_vec))
        }
        (pagination_control(RELATIVE_PATH, &pagination_options, &pagination_result))
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

const HEADINGS: [&str; 5] = ["id", "display_name", "internal_name", "upc", "release_date"];

fn table(elements: Vec<ProductSerial>) -> Markup {
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
                        td { (format!("{:?}", element.upc)) }
                        td { (format!("{:?}", element.release_date)) }
                    }
                }
            }
        }
    }
}
