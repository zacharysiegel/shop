//! Generate JavaScript scripts to perform "reactive" updates to client-side application state and presentation.

use crate::registry::REGISTRY;
use serde::Serialize;
use std::fmt::Debug;

/// Produces the `element` variable which binds to the found element at the given identifier
pub fn activate_element_handler(element_id: &str) -> String {
    format!(r#"
        event.preventDefault();
        const element = document.getElementById("{}");
        element.style.display = "initial";
    "#,
            element_id,
    ).to_string()
}

/// Produces the `element` variable which binds to the found element at the given identifier
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
        r#"{{
        const forms = element.getElementsByTagName("form");
        if (forms.length === 0) {{
            console.err("Error: No form found within target element", element);
            return;
        }}

        const form = forms[forms.length - 1]
        form.action = "{}{}";

        const parameters = JSON.parse(`{}`); // If this string contains backticks, the script will probably fail
        for (let [key, value] of Object.entries(parameters)) {{
            const input = form[key];
            if (input === undefined) continue;
            
            if (input.type === "datetime-local") {{
                input.value = new Date(Date.parse(value)).toISOString().slice(0, 19);
            }} else {{
                input.value = value;
            }}
        }}
        }}"#,
        REGISTRY.inventory_external_path,
        path,
        json_parameters,
    )
}

#[allow(dead_code)]
pub fn update_form_action(path: &str) -> String {
    let json = "{}";
    update_form_from_json_string(path, json)
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
        r#"{{
        const parameters = JSON.parse('{}');
        for (let [key, value] of Object.entries(parameters)) {{
            const element = document.getElementById("{}" + key);
            element.innerText = value;
        }}
        }}"#,
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
