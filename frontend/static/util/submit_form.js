import {component} from "./sigma.js";

document.addEventListener("submit", submit_form);

/**
 * @param {SubmitEvent} submit_event
 */
function submit_form(submit_event) {
    submit_event.preventDefault();
    submit_event.stopPropagation();
    /**
     * @type {HTMLFormElement}
     */
    const form = submit_event.target;

    const form_data = new FormData(form, submit_event.submitter);
    const form_data_as_object = {};
    for (let entry of form_data.entries()) {
        let [key, value] = entry;

        const number = as_number(form, key, value);
        if (number !== undefined) {
            form_data_as_object[key] = number;
            continue;
        }

        const iso_date_or_datetime = as_iso_date_or_datetime(form, key, value);
        if (iso_date_or_datetime !== undefined) {
            form_data_as_object[key] = iso_date_or_datetime;
            continue;
        }

        const enum_representation = as_enum_representation(form, key, value);
        if (enum_representation !== undefined) {
            form_data_as_object[key] = enum_representation;
            continue;
        }

        form_data_as_object[key] = value;
    }

    const response_component = form_response_component();
    form.getElementsByClassName(response_component.elements[0].classList.item(0))?.item(0)?.remove();
    response_component.append_self(form);

    /* By default, FormData is converted to the format "multipart/form-data". This representation
        adds significant bloat when each field's data is encoded in UTF-8 and is generally small.
        We convert to JSON instead. */
    const request = new Request(form.action, {
        method: form.dataset["method"],
        headers: new Headers([
            ["Content-Type", "application/json"],
        ]),
        body: JSON.stringify(form_data_as_object),
    });
    fetch(request)
        .then(response => {
            response_component.callbacks.set_status(response);
        });
}

/**
 * Dates and datetimes should be serialized into ISO format.
 * @param form {HTMLFormElement}
 * @param key {string}
 * @param value {string}
 * @return {(string | undefined)}
 */
function as_iso_date_or_datetime(form, key, value) {
    if (!value || typeof value !== "string") {
        return undefined;
    }

    const dateValue = Date.parse(value);
    if (isNaN(dateValue)) {
        return undefined;
    }

    const form_control = form.elements.namedItem(key);
    if (!form_control) {
        return undefined;
    }

    if (form_control.getAttribute("type") === "date") {
        return new Date(dateValue).toISOString().slice(0, -14); // Remove the time
    } else if (form_control.getAttribute("type") === "datetime-local") {
        return new Date(dateValue).toISOString();
    }

    return undefined;
}

/**
 * Numerical values of select elements should be coerced into integers in order to deserialize into enumeration representations.
 * @param form {HTMLFormElement}
 * @param key {string}
 * @param value {string}
 * @return {(number | undefined)}
 */
function as_enum_representation(form, key, value) {
    if (!value || typeof value !== "string") {
        return undefined;
    }

    const form_control = form.elements.namedItem(key);
    if (!form_control || form_control.tagName !== "SELECT") {
        return undefined;
    }

    const int_value = Number(value);
    if (isNaN(int_value)) {
        return undefined;
    }

    return int_value;
}

/**
 * Number inputs should be converted back to numbers.
 * @param form {HTMLFormElement}
 * @param key {string}
 * @param value {string}
 * @return {(number | undefined)}
 */
function as_number(form, key, value) {
    const form_control = form.elements.namedItem(key);
    if (!form_control || form_control.type !== "number") {
        return undefined;
    }

    const int_value = Number(value);
    if (isNaN(int_value)) {
        return undefined;
    }

    return int_value;
}

/**
 * @type {ComponentFactory}
 * @param parameters
 * @param {{
 *     response: Response,
 * }} parameters.properties
 */
const form_response_component_factory = ({fragment, properties, add_callback}) => {
    let text;
    fragment.append((() => {
        const root = document.createElement("div");
        root.classList.add("form_response_component");
        root.append((() => {
            text = document.createElement("span");
            set_status(null);
            return text;
        })());
        root.append((() => {
            const x_button = document.createElement("button");
            x_button.innerText = "X";
            x_button.type = "button";
            x_button.style.display = "inline";
            x_button.style.marginLeft = "1rem";
            x_button.onclick = () => root.parentElement.removeChild(root);
            return x_button;
        })());
        return root;
    })());

    /**
     * @param response {Response}
     */
    function set_status(response) {
        text.innerText = response
            ? `Response status: ${response.status} ${response.statusText}`
            : "pending";
    }

    add_callback("set_status", set_status);
};

export const form_response_component = component()
    .factory(form_response_component_factory)
    .properties({response: null})
    .build();
