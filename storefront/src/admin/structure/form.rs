use crate::registry::REGISTRY;
use maud::{html, Markup};

/// Generates an HTML form element. Will send a POST request to the path prefixed by the REGISTRY.remote_url.
pub fn form(path: &str, content: Markup) -> Markup {
    html! {
        form
        action=(format!("{}{}", REGISTRY.remote_url, path))
        autocomplete="off"
        enctype="application/x-www-form-urlencoded"
        method="POST"
        {
            (content)
        }
    }
}
