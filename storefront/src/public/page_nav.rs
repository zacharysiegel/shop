use maud::{DOCTYPE, Markup, html};

const NAV_WIDTH: &str = "17rem";
const NAV_PADDING: &str = "8px";

/// Deprecated
pub async fn page_nav(content: Markup) -> Markup {
	html! {
		(DOCTYPE)
		html {
			head {
				meta charset="utf-8";
				title {"Collect"}
			}
			body {
				nav style=(format!("\
				display: block;\
				position: fixed;\
				left: 0;\
				top: 0;\
				width: {};\
				height: 100%;\
				z-index: 1;\
				background-color: rgb(48, 52, 70);\
				color: white;",
				NAV_WIDTH
				)) {
					div style=(format!("\
					display: flex;\
					flex-direction: column;\
					justify-content: space-between;\
					height: calc(100% - {0} * 2);\
					padding: {0};\
					",
					NAV_PADDING
					)) {
						(nav_top())
						(nav_bottom())
					}
				}
				main style=(format!("\
				position: relative;\
				left: {0};\
				width: calc(100% - {0});\
				",
				NAV_WIDTH
				)) {
					(content)
				}
			}
		}
	}
}

fn nav_top() -> Markup {
	html! {
		div style="\
		display: flex;\
		justify-content: space-between;\
		align-items: center;\
		" {
			div style="width: 4rem; height: 4rem; background-color: #DDD25B;" {}
			div style="width: 4rem; height: 4rem; background-color: #5FC6DD;" {}
		}
	}
}

fn nav_bottom() -> Markup {
	html! {
		div style="\
		display: flex;\
		flex-direction: row;\
		justify-content: space-between;\
		align-items: center;\
		"
		{
			div { button { "About Us" } }
			div { button { "Contact" } }
		}
	}
}
