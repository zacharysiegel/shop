use crate::admin::api::wrapped_get;
use crate::admin::structure::page::Page;
use crate::admin::structure::{form, page, split};
use crate::unwrap_result_else_markup;
use actix_web::web;
use actix_web::web::ServiceConfig;
use inventory::category::CategorySerial;
use maud::{html, Markup};
use reqwest::Method;

pub const PAGE: Page = Page {
    name: "Category",
    relative_path: "/admin/category",
    configurer,
};

fn configurer(config: &mut ServiceConfig) {
    config.route("/category", web::get().to(render));
}

async fn render() -> Markup {
    page::page(
        &vec!(PAGE.into()),
        Markup::default(),
        split::split(left().await, right()),
    )
}


async fn left() -> Markup {
    let elements: Vec<CategorySerial> = unwrap_result_else_markup!(
        wrapped_get("/category").await
    );

    html! {
        h2 { "Categories" }
        @if elements.is_empty() {
            p { "None" }
        } @else {
            (table(elements))
        }
    }
}

fn right() -> Markup {
    form::form(Some("Create category"), "/category", Method::POST, html! {
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

const HEADINGS: [&str; 4] = ["id", "display_name", "internal_name", "parent_id"];
fn table(elements: Vec<CategorySerial>) -> Markup {
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
                        td { (format!("{:?}", element.parent_id)) }
                    }
                }
            }
        }
    }
}
