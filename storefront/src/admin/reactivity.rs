//! Generate JavaScript scripts to perform "reactive" updates to client-side application state and presentation.

use crate::registry::REGISTRY;
use serde::Serialize;
use std::fmt::Debug;

pub fn activate_element_handler(element_id: &str) -> String {
    format!(r#"
        event.preventDefault();
        const element = document.getElementById("{}");
        element.style.display = "initial";
    "#,
            element_id,
    ).to_string()
}

pub fn hide_element_handler(element_id: &str) -> String {
    format!(r#"
        event.preventDefault();
        const element = document.getElementById("{}");
        element.style.display = "none";
    "#,
            element_id,
    ).to_string()
}

/// The generated script expects an "element" object to exist already which itself contains a <form> element.
/// This element object can be created by prepending the `activate_element_handler` snippet or a similar script.
pub fn update_form_from_json_string(path: &str, json_parameters: &str) -> String {
    format!(
        r#"
        const form = element.getElementsByTagName("form")[0];
        form.action = "{}{}";

        const parameters = JSON.parse('{}');
        for (let [key, value] of Object.entries(parameters)) {{
            const input = form[key];
            if (input === undefined) continue;
            input.value = value;
        }}
        "#,
        REGISTRY.remote_url,
        path,
        json_parameters,
    )
}

/// Serializes the given object to a JSON string and forwards to `update_form_script_from_json_string`.
pub fn update_form_from_serialize<T: Serialize + Debug>(path: &str, parameter_object: &T) -> String {
    let json: String = to_json_else_console_err(parameter_object);
    update_form_from_json_string(path, &json)
}

/// Set the inner text of a set of HTML elements which share a common prefix and whose suffixes match the keys of a given struct.
pub fn set_content_by_prefix_from_serialize<T: Serialize + Debug>(id_prefix: &str, object: &T) -> String {
    let json: String = to_json_else_console_err(object);
    format!(
        r#"
        const parameters = JSON.parse('{}');
        for (let [key, value] of Object.entries(parameters)) {{
            const element = document.getElementById("{}" + key);
            element.innerText = value;
        }}
        "#,
        json,
        id_prefix,
    )
}

fn to_json_else_console_err<T: Serialize + Debug>(object: &T) -> String {
    match serde_json::to_string(object) {
        Ok(value) => value,
        Err(error) => return format!(
            r#"console.err("Error serializing value", {:?}, {:#});"#,
            object,
            error,
        ).to_string(),
    }
}
