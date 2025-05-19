use std::ops::Deref;
use crate::admin::structure::breadcrumb::BreadcrumbItem;
use crate::admin::structure::page::Page;
use crate::admin::structure::{form, page};
use actix_web::web;
use actix_web::web::ServiceConfig;
use maud::{html, Markup};
use reqwest::Method;
use inventory::environment::RuntimeEnvironment;

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
        content(),
    )
}


fn content() -> Markup {
    let runtime_environment: RuntimeEnvironment = RuntimeEnvironment::default();
    html! {
        h2 { "eBay integration" }

        @if runtime_environment == RuntimeEnvironment::Local {
            (auth_local())
        }
    }
}

fn auth_local() -> Markup {
    let oauth_url: &&str = inventory::marketplace::ebay::ebay_api::EBAY_OAUTH_AUTHORIZATION_URL.deref();
    html! {
        p {
            a href=(oauth_url) target="_blank" rel="noopener noreferrer" { "Authorization code redirect" }
        }
        p {
            r#"Authenticate with your eBay "TESTUSER_*" account. After logging in, you will be redirected to a blank landing page. The landing page URL contains a "code" query parameter. Copy that code and enter it below."#
        }
        br;
        (form::form("Fetch user tokens", "/ebay/auth/user/token", Method::PUT, html! {
            label {
                "Authorization code"
                input type="text" name="code";
            }
            input type="submit";
        }))
    }
}
