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
pub fn update_form_script_from_json_string(path: &str, json_parameters: &str) -> String {
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

pub fn update_form_script_from_serialize<T: Serialize + Debug>(path: &str, parameter_object: &T) -> String {
    let json: String = match serde_json::to_string(parameter_object) {
        Ok(value) => value,
        Err(error) => return format!(
            r#"console.err("Error serializing value", {:?}, {:#});"#,
            parameter_object,
            error,
        ).to_string(),
    };
    update_form_script_from_json_string(path, &json)
}
