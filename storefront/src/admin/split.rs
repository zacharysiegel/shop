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
                "flex-basis: 65%;",
                "flex-grow: 1;",
                "flex-shrink: 2;",
                "padding-right: .5rem;",
            )) {
                (left)
            }

            div style=(concat!(
                "flex-basis: 35%;",
                "flex-grow: 0;",
                "flex-shrink: 1;",
                "padding-left: .5rem;",
            )) {
                (right)
            }
        }
    }
}