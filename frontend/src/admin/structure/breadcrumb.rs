use crate::admin::structure::page::Page;
use maud::{html, Markup};

pub struct BreadcrumbItem<'a> {
    name: &'a str,
    relative_path: &'a str,
}

impl BreadcrumbItem<'_> {
    pub fn new<'a>(name: &'a str, relative_path: &'a str) -> BreadcrumbItem<'a> {
        BreadcrumbItem { name, relative_path }
    }
}

impl<'a> From<Page<'a>> for BreadcrumbItem<'a> {
    fn from(value: Page<'a>) -> Self {
        BreadcrumbItem {
            name: value.name,
            relative_path: value.relative_path,
        }
    }
}

pub fn breadcrumb(current_page_branch: &Vec<BreadcrumbItem>) -> Markup {
    html! {
		p {
			a href="/admin" { "Home" }
			@for breadcrumb_item in current_page_branch {
				" / "
				a href=(breadcrumb_item.relative_path) { (breadcrumb_item.name) }
			}
		}
	}
}
