use crate::admin::page;
use maud::{html, Markup};

pub async fn render() -> Markup {
    page::page(html! {
		div {
			ol {
				li {
					a href={"/admin/item"} { "Item" }
				}
			}
		}
	})
        .await
}
