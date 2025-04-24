use crate::admin::{item, page, product};
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
        .await
}
