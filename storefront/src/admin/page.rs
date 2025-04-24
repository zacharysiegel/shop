use maud::{html, Markup, DOCTYPE};

pub fn page(content: Markup) -> Markup {
    html! {
		(DOCTYPE)
		html {
			head {
				meta charset="utf-8";
				title {"Shop | Administration"}
				link rel="stylesheet" href="/reset.css";
			}
			body style=(concat!(
				"min-height: calc(100vh - 2rem);",
				"margin: 1rem;",
				"font-family: monospace;",
				"display: flex; flex-direction: column;",
			)) {
				header {
					hgroup style="margin-bottom: 1rem;" {
						h1 { "Shop administration" }
						a href="/admin" { "Home" }
					}
				}
				main style=("flex-grow: 1;") {
					(content)
				}
			}
		}
	}
}
