use crate::admin::page;
use maud::{html, Markup};

pub async fn render() -> Markup {
    page::page(html! {
		div {
			"test content"
		}
	})
        .await
}
