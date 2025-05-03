use maud::{html, Markup, DOCTYPE};

pub type PageInfo<'a> = (&'a str, &'a str);

pub fn page(current_page_branch: &Vec<PageInfo>, head_content: Markup, body_content: Markup) -> Markup {
    html! {
		(DOCTYPE)
		html {
			head {
				meta charset="utf-8";
				title {"Shop | Administration"}
				link rel="icon" href="/favicon.svg";
				link rel="stylesheet" type="text/css" href="/reset.css";
				link rel="stylesheet" type="text/css" href="/base.css";
				link rel="stylesheet" type="text/css" href="/tree.css";
				script src="/submit_form.js" {};
				(head_content)
			}
			body style=(concat!(
				"min-height: calc(100vh - 2rem);",
				"margin: 1rem;",
				"font-family: monospace;",
				"display: flex; flex-direction: column;",
				"background-color: var(--color-base); color: var(--color-text);",
			)) {
				header {
					hgroup style="margin-bottom: 1rem;" {
						h1 { "Shop administration" }
						(breadcrumb(&current_page_branch))
					}
				}
				main style=("flex-grow: 1;") {
					(body_content)
				}
			}
		}
	}
}

fn breadcrumb(current_page_branch: &Vec<PageInfo>) -> Markup {
    html! {
		p {
			a href="/admin" { "Home" }
			@for (path, name) in current_page_branch {
				" / "
				a href=(path) { (name) }
			}
		}
	}
}
