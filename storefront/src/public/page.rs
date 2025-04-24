use maud::{html, Markup, DOCTYPE};

pub async fn page(content: Markup) -> Markup {
    html! {
		(DOCTYPE)
		html {
			head {
				meta charset="utf-8";
				title {"Collect"}
			}
			body {
				(content)
			}
		}
	}
}
