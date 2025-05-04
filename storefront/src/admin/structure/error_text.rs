use maud::{html, Markup};
use std::error;

pub fn error_markup(error: impl error::Error) -> Markup {
    html!{
        (format!("Error: {:#}", error))
    }
}

pub fn error_text(error: impl error::Error) -> String {
    format!("Error: {:#}", error)
}
