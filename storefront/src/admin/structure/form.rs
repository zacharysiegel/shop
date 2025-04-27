use crate::registry::REGISTRY;
use maud::{html, Markup};

/// Generates an HTML form element. Will send a POST request to the path prefixed by the REGISTRY.remote_url.
pub fn form(heading: &str, path: &str, content: Markup) -> Markup {
    html! {
        h2 { (heading) }

        form
        action=(format!("{}{}", REGISTRY.remote_url, path))
        autocomplete="off"
        // This default encoding is overridden by submit_form.js
        enctype="application/x-www-form-urlencoded"
        method="POST"
        {
            (content)
        }
    }
}
