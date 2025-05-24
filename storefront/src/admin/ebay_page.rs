use crate::admin::api::wrapped_get;
use crate::admin::structure::breadcrumb::BreadcrumbItem;
use crate::admin::structure::page::Page;
use crate::admin::structure::{form, page};
use crate::unwrap_result_else_markup;
use actix_web::web;
use actix_web::web::ServiceConfig;
use inventory::environment::RuntimeEnvironment;
use inventory::item::ItemSerial;
use maud::{html, Markup};
use reqwest::Method;
use std::ops::Deref;

pub const PAGE: Page = Page {
    name: "eBay",
    relative_path: "/admin/ebay",
    configurer,
};

fn configurer(config: &mut ServiceConfig) {
    config.route("/ebay", web::get().to(render));
}

async fn render() -> Markup {
    page::page(
        &vec!(BreadcrumbItem::from(PAGE)),
        Markup::default(),
        content().await,
    )
}


async fn content() -> Markup {
    let runtime_environment: RuntimeEnvironment = RuntimeEnvironment::default();

    html! {
        h1 { "eBay integration" }

        @if runtime_environment == RuntimeEnvironment::Local {
            (auth_local())
            (refresh())
        }
        
        (inventory_locations().await)
    }
}

fn auth_local() -> Markup {
    let oauth_url: &&str = inventory::marketplace::ebay::ebay_api::EBAY_OAUTH_AUTHORIZATION_URL.deref();
    html! {
        (form::form("Authenticate", "/ebay/auth/user/token", Method::PUT, html! {
            h3 { "Fetch authorization code"}
            p {
                a href=(oauth_url) target="_blank" rel="noopener noreferrer" { "Authorization code redirect" }
            }
            p {
                r#"Authenticate with your eBay "TESTUSER_*" account. After logging in, you will be redirected to a blank landing page. The landing page URL contains a "code" query parameter. Copy that code and enter it below."#
            }
            br;
            h3 { "Generate user access and refresh tokens"}
            label {
                "Authorization code"
                input type="text" name="code";
            }
            input type="submit";
        }))
    }
}

async fn inventory_locations() -> Markup {
    let inventory_locations: serde_json::Value = unwrap_result_else_markup!(
        wrapped_get("/ebay/location").await
    );

    html! {
        h2 { "Inventory locations" }
        pre { (format!("{}", inventory_locations)) }
    }
}

fn refresh() -> Markup {
    html! {
        hr;
        (form::form("Refresh user access token", "/ebay/auth/user/refresh", Method::PUT, html! {
            button type="submit" { "Refresh" }
        }))
    }
}
