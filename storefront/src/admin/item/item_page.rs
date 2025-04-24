use crate::admin;
use crate::admin::item::create_item;
use admin::page;
use maud::{html, Markup};

pub const RELATIVE_PATH: &str = "/admin/item";

pub async fn render() -> Markup {
    page::page(
        html!(
            div {
                "<item page>"
            }
            (create_item::create_item().await)
        )
    ).await
}