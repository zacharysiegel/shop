use maud::{html, Markup};

pub async fn create_item() -> Markup {
    html! {
        div {
            "<create item>"
        }
    }
}