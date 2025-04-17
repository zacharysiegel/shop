use maud::{DOCTYPE, Markup, html};

pub async fn render() -> Markup {
    html! {
        (DOCTYPE)
        body {
            p {
                "hello world!"
            }
        }
    }
}
