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
