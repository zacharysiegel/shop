use inventory::pagination::{Direction, KeysetPaginationOptionsForString, KeysetPaginationResultForString};
use maud::{html, Markup};

macro_rules! url_encoded_pagination_options_else_err {
    ($options:expr$(,)?) => {
        match ::serde_urlencoded::to_string($options) {
            ::std::result::Result::Ok(val) => val,
            ::std::result::Result::Err(error) => return $crate::admin::structure::error_text::error_markup(error),
        }
    };
}

pub fn pagination_control(
    relative_path: &str,
    pagination_options: &KeysetPaginationOptionsForString,
    pagination_result: &KeysetPaginationResultForString,
) -> Markup {
    let next_page_params = url_encoded_pagination_options_else_err!(
        pagination_result.create(&Direction::Ascending, &pagination_options.max_page_size)
    );
    let previous_page_params = url_encoded_pagination_options_else_err!(
        pagination_result.create(&Direction::Descending, &pagination_options.max_page_size)
    );

    html! {
        div style=(concat!(
            "display: flex; flex-direction: row; justify-content: center; align-items: center;",
            "margin: 1rem 0;",
        )) {
            a href=(format!("{}/?{}", relative_path, previous_page_params)) {
                button disabled[!pagination_result.has_lesser_value] { "<--" }
            }
            span style=(concat!("margin: 0 1rem;")) {
                (format!("Showing {} entries", pagination_result.page_size))
            }
            a href=(format!("{}/?{}", relative_path, next_page_params)) {
                button disabled[!pagination_result.has_greater_value] { "-->" }
            }
        }
    }
}
