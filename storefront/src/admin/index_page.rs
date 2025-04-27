use crate::admin::structure::page;
use crate::admin::{category, item, product};
use actix_web::web;
use actix_web::web::ServiceConfig;
use maud::{html, Markup};

pub async fn render() -> Markup {
    page::page(
        None,
        html! {
			div {
				ol {
					li { a href=(product::product_page::RELATIVE_PATH) { "Product" } }
					li { a href=(item::item_page::RELATIVE_PATH) { "Item" } }
					li { a href=(category::category_page::RELATIVE_PATH) { "Category" } }
				}
			}
		}
	)
}

pub fn configurer(config: &mut ServiceConfig) -> () {
    config
        .service(web::scope("/admin")
            .route("", web::get().to(render))
            .route("/index.html", web::get().to(render))
            .configure(item::item_page::configurer)
            .configure(product::product_page::configurer)
            .configure(category::category_page::configurer)
        )
    ;
}