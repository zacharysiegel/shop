use crate::registry::REGISTRY;
use chrono::{DateTime, Utc};
use maud::{html, Markup};
use reqwest::Method;
use std::fmt::Debug;
use strum::VariantArray;

/// Generates an HTML form element. Will send a POST request to the path prefixed by the REGISTRY.remote_url.
pub fn form(heading: &str, path: &str, method: Method, content: Markup) -> Markup {
    html! {
        div {
            h2 { (heading) }

            form
            action=(format!("{}{}", REGISTRY.inventory_external_path, path))
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

pub fn get_current_datetime_string() -> String {
    let datetime: DateTime<Utc> = Utc::now();
    let string: String = datetime.to_rfc3339();
    let end_index: usize = string.find("T").unwrap_or(string.len() - 6) + 6;
    string[0..end_index].to_string()
}

pub fn enum_options<EnumT>() -> Markup
where
    EnumT: Debug + VariantArray,
{
    html! {
        @for variant in EnumT::VARIANTS {
            // This is a hack to get around our inability to require a primitive representation for the given enum type.
            @let value: String = format!("{:?}", std::mem::discriminant(variant))
                .replace("Discriminant(", "")
                .replace(")", "");
            option value=(value) {
                (format!("{:?}", variant))
            }
        }
    }
}
