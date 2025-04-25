use crate::admin::registry::REGISTRY;
use crate::admin::{page, split};
use actix_web::web;
use actix_web::web::ServiceConfig;
use inventory::product::ProductSerial;
use maud::{html, Markup};

pub const RELATIVE_PATH: &str = "/admin/product";

pub fn configurer(config: &mut ServiceConfig) {
    config.route("/product", web::get().to(render));
}

async fn render() -> Markup {
    html! {
        (page::page(
            split::split(left().await, right())
        ))
    }
}

async fn left() -> Markup {
    let result = REGISTRY.http_client.get(REGISTRY.remote_url.clone() + "/product")
        .send()
        .await;
    let response = match result {
        Ok(response) => response,
        Err(error) => {
            return html!((format!("Error: {:#}", error)));
        }
    };
    let product_vec = match response.json::<Vec<ProductSerial>>().await {
        Ok(product) => product,
        Err(error) => {
            return html!((format!("Error: {:#}", error)));
        }
    };
    html! {
        h2 {"All products"}
        ol {
            @for product in &product_vec {
                li {
                    (format!("Product: {:#?}", product))
                }
            }
        }
    }
}

fn right() -> Markup {
    html! {
        h2 {"Create product"}
        "right side"
    }
}
