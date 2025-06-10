use maud::{html, Markup};

pub fn split(
    left: Markup,
    right: Markup,
) -> Markup {
    html! {
        div style=(concat!(
            "display: flex; flex-direction: row;",
        )) {
            div .left style=(concat!(
                "flex-basis: 75%;",
                "flex-grow: 1;",
                "flex-shrink: 2;",
                "padding-right: .5rem;",
                "overflow-x: auto;"
            )) {
                (left)
            }

            div .right style=(concat!(
                "flex-basis: 25%;",
                "flex-grow: 0;",
                "flex-shrink: 1;",
                "padding-left: .5rem;",
            )) {
                (right)
            }
        }
    }
}