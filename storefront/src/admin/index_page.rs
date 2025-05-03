use crate::admin::structure::page;
use crate::admin::{category_page, inventory_location_page, item_page, listing_page, product_page};
use actix_web::web;
use actix_web::web::ServiceConfig;
use maud::{html, Markup};

pub async fn render() -> Markup {
    page::page(
        &Vec::default(),
		Markup::default(),
        html! {
			div {
				ol {
					li { a href=(product_page::RELATIVE_PATH) { "Product" } }
					li { a href=(category_page::RELATIVE_PATH) { "Category" } }
					li { a href=(inventory_location_page::RELATIVE_PATH) { "Inventory location" } }
				}
			}
		},
    )
}

pub fn configurer(config: &mut ServiceConfig) -> () {
    config
        .service(web::scope("/admin")
            .route("", web::get().to(render))
            .route("/index.html", web::get().to(render))
            .configure(item_page::configurer)
            .configure(product_page::configurer)
            .configure(category_page::configurer)
            .configure(inventory_location_page::configurer)
            .configure(listing_page::configurer)
        )
    ;
}
