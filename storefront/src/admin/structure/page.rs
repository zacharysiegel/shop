use maud::{html, Markup, DOCTYPE};

pub fn page(current_page_name: Option<&str>, content: Markup) -> Markup {
    html! {
		(DOCTYPE)
		html {
			head {
				meta charset="utf-8";
				title {"Shop | Administration"}
				link rel="stylesheet" type="text/css" href="/reset.css";
				link rel="stylesheet" type="text/css" href="/base.css";
				script src="/submit_form.js" {};
			}
			body style=(concat!(
				"min-height: calc(100vh - 2rem);",
				"margin: 1rem;",
				"font-family: monospace;",
				"display: flex; flex-direction: column;",
				"background-color: rgb(36, 39, 58); color: rgb(202, 211, 245);" // Catppuccin Macchiato: https://catppuccin.com/palette/
			)) {
				header {
					hgroup style="margin-bottom: 1rem;" {
						h1 { "Shop administration" }
						(breadcrumb(current_page_name))
					}
				}
				main style=("flex-grow: 1;") {
					(content)
				}
			}
		}
	}
}

fn breadcrumb(current_page_name: Option<&str>) -> Markup {
    html! {
		p {
			a href="/admin" { "Home" }
			@if let Some(current_page_name) = current_page_name {
			" / "
			a href="" { (current_page_name) }
			}
		}
	}
}
