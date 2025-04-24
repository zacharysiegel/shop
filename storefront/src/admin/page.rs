use maud::{html, Markup, DOCTYPE};

pub async fn page(content: Markup) -> Markup {
    html! {
		(DOCTYPE)
		html {
			head {
				meta charset="utf-8";
				title {"Shop | Administration"}
				link rel="stylesheet" href="/reset.css";
			}
			body style="margin: 1rem; font-family: monospace;" {
				header {
					hgroup style="margin-bottom: 1rem;" {
						h1 { "Shop administration" }
						a href="/admin" { "Home" }
					}
				}
				main {
					(content)
				}
			}
		}
	}
}
