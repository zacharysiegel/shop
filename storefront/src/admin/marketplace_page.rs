use crate::admin::api::wrapped_get;
use crate::admin::structure::breadcrumb::BreadcrumbItem;
use crate::admin::structure::page::Page;
use crate::admin::structure::{form, page, split};
use crate::unwrap_result_else_markup;
use actix_web::web;
use actix_web::web::ServiceConfig;
use inventory::marketplace::MarketplaceSerial;
use maud::{html, Markup};
use reqwest::Method;

pub const PAGE: Page = Page {
    name: "Marketplace",
    relative_path: "/admin/marketplace",
    configurer,
};

const HEADINGS: [&str; 4] = ["id", "display_name", "internal_name", "uri"];

fn configurer(config: &mut ServiceConfig) {
    config.route("/marketplace", web::get().to(render))
    ;
}

async fn render() -> Markup {
    page::page(
        &vec!(BreadcrumbItem::from(PAGE)),
        Markup::default(),
        split::split(left().await, right()),
    )
}

async fn left() -> Markup {
    let marketplace_vec: Vec<MarketplaceSerial> = unwrap_result_else_markup!(
        wrapped_get::<Vec<MarketplaceSerial>>("/marketplace").await
    );

    html! {
        h2 { "Marketplaces" }
        @if marketplace_vec.is_empty() {
            p { "None" }
        } @else {
            (table(&marketplace_vec).await)
        }
    }
}

fn right() -> Markup {
    create_marketplace_form()
}

async fn table(marketplace_vec: &Vec<MarketplaceSerial>) -> Markup {
    html! {
        table {
            thead {
                tr {
                    @for heading in HEADINGS {
                        th { (heading) }
                    }
                }
            }
            tbody {
                @for marketplace in marketplace_vec {
                    tr {
                        td { (marketplace.id) }
                        td { (marketplace.display_name) }
                        td { (marketplace.internal_name) }
                        td { (format!("{:?}", marketplace.uri)) }
                    }
                }
            }
        }
    }
}

fn create_marketplace_form() -> Markup {
    form::form(Some("Create marketplace"), "/marketplace", Method::POST, html! {
        label {
            "Display name"
            input type="text" name="display_name";
        }
        label {
            "Internal name"
            input type="text" name="internal_name";
        }
        label {
            "URI (optional)"
            input type="text" name="uri";
        }
        input type="submit";
    })
}
