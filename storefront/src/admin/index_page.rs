use crate::admin::{item, page, product};
use actix_web::web;
use actix_web::web::ServiceConfig;
use maud::{html, Markup};

pub async fn render() -> Markup {
    page::page(html! {
		div {
			ol {
				li { a href={(product::product_page::RELATIVE_PATH)} { "Product" } }
				li { a href={(item::item_page::RELATIVE_PATH)} { "Item" } }
			}
		}
	})
}

pub fn configurer(config: &mut ServiceConfig) -> () {
    config
        .service(web::scope("/admin")
            .route("", web::get().to(render))
            .route("/index.html", web::get().to(render))
            .configure(item::item_page::configurer)
            .configure(product::product_page::configurer)
        )
    ;
}