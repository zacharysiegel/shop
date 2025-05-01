use crate::admin;
use crate::admin::api::wrapped_get;
use crate::admin::structure::error_text::error_text;
use crate::admin::structure::form;
use crate::admin::structure::{page, split};
use crate::registry::REGISTRY;
use actix_web::web::ServiceConfig;
use actix_web::{guard, web};
use admin::structure::pagination_control::pagination_control;
use inventory::pagination::{pagination_guard, KeysetPaginationOptionsForString, KeysetPaginationResultForString};
use inventory::product::ProductSerial;
use maud::{html, Markup};
use reqwest::Method;
use uuid::Uuid;

pub const RELATIVE_PATH: &str = "/admin/product";
pub const DELETE_FORM_CONTAINER_ID: &str = "delete_form_container";

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
    html! {
        (create_form())
        (delete_form())
    }
}

fn create_form() -> Markup {
    form::form("Create product", "/product", Method::POST, html! {
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

fn delete_form() -> Markup {
    html! {
        div #(DELETE_FORM_CONTAINER_ID) style=(concat!("display: none;")) {
            hr style=("margin: 1rem 0") {}
            (form::form("Delete product", "undefined", Method::DELETE, html! {
                label {
                    "ID"
                    input type="text" name="id" disabled[true];
                }
                input type="submit";
                button onclick=(cancel_form_script()) { "Cancel" }
            }))
        }
    }
}

const HEADINGS: [&str; 6] = ["id", "display_name", "internal_name", "upc", "release_date", "actions"];

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
                        td {
                            button style="margin: .2rem;" onclick=(activate_form_script(&element.id)) { "Delete" }
                        }
                    }
                }
            }
        }
    }
}

fn activate_form_script(id: &Uuid) -> String {
    format!(r#"
        const form_container = document.getElementById("{0}");
        form_container.style.display = "block";
        const form = form_container.lastChild.lastChild;
        form.action = "{1}{2}{3}";
        form.id.value = "{3}";
    "#,
            DELETE_FORM_CONTAINER_ID,
            REGISTRY.remote_url,
            "/product/",
            id.to_string(),
    ).to_string()
}

fn cancel_form_script() -> String {
    format!(r#"
        event.preventDefault();
        const form_container = document.getElementById("{}");
        form_container.style.display = "none";
    "#,
            DELETE_FORM_CONTAINER_ID,
    ).to_string()
}
