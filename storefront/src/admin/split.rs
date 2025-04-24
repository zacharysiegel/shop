use maud::{html, Markup};

pub fn split(
    left: Markup,
    right: Markup,
) -> Markup {
    html! {
        div style=(concat!(
            "display: flex; flex-direction: row;",
        )) {
            div style=(concat!(
                "flex-grow: 2;"
            )) {
                (left)
            }

            div style=(concat!(
                "flex-grow: 1;"
            )) {
                (right)
            }
        }
    }
}