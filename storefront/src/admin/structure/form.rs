use crate::registry::REGISTRY;
use maud::{html, Markup};
use reqwest::Method;

/// Generates an HTML form element. Will send a POST request to the path prefixed by the REGISTRY.remote_url.
pub fn form(heading: &str, path: &str, method: Method, content: Markup) -> Markup {
    html! {
        div {
            h2 { (heading) }

            form
            action=(format!("{}{}", REGISTRY.remote_url, path))
            autocomplete="off"
            /* The standard method attribute only allows "post", "get", and "dialogue" values.
                This value is extracted in submit_form.js. */
            data-method=(method.as_str())
            {
                (content)
            }
        }
    }
}
