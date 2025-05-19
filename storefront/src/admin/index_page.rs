use crate::admin::structure::page;
use crate::admin::{category_page, ebay_page, inventory_location_page, item_page, listing_page, marketplace_page, product_page};
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
					li { (product_page::PAGE.create_anchor()) }
					li { (category_page::PAGE.create_anchor()) }
					li { (inventory_location_page::PAGE.create_anchor()) }
                    li { (marketplace_page::PAGE.create_anchor()) }
                    li { (ebay_page::PAGE.create_anchor()) }
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
            .configure(item_page::PAGE.configurer)
            .configure(product_page::PAGE.configurer)
            .configure(category_page::PAGE.configurer)
            .configure(inventory_location_page::PAGE.configurer)
            .configure(listing_page::PAGE.configurer)
            .configure(marketplace_page::PAGE.configurer)
            .configure(ebay_page::PAGE.configurer)
        )
    ;
}
