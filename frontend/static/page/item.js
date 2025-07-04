import {component} from "../util/sigma.js";
import {h} from "../util/hyperscript.js";
import {api_url, base_url, fetch_checked} from "../util/http.js";
import {form_response_component} from "../util/submit_form.js";

/** @type {ComponentInstance | null} */
let current_item_images_component = null;
/** @type {string | null} */
let current_item_images_id = null;

/**
 * @typedef {{
 *     id: String,
 *     item_id: String,
 *     alt_text: String,
 *     priority: Number,
 *     original_file_name: String,
 *   }} ItemImage
 */

/**
 * @param {ItemImage} item_image
 * @return String
 */
function get_image_uri(item_image) {
    return `${base_url}/volatile/images/${item_image.item_id}_${item_image.id}_${item_image.original_file_name}`;
}

const item_image_element = component()
    .properties({
        item_image: null,
    })
    .factory(({fragment, properties}) => {
        const {
            /** @type ItemImage */
            item_image,
        } = properties;
        const element = h("li",
            h("a", {href: get_image_uri(item_image), target: "_blank"}, `${item_image.id}`),
            h("span", item_image.alt_text ? ` [${item_image.alt_text}]` : ""),
        );
        fragment.appendChild(element);
    })
    .build();

const item_image_upload_form = component()
    .properties({
        fetch: null,
        item_id: null,
    })
    .factory(({fragment, properties}) => {
        /** @type HTMLInputElement */
        const file_input = h("input", {type: "file", name: "file"});
        const alt_text_input = h("input", {type: "text", name: "alt_text"});
        const error_container = h("div");
        const result_container = h("div");
        const form = h("div",
            h("h3", {style: {"margin-top": ".5rem"}}, "Upload"),
            h("label", {htmlFor: "alt_text", style: {"margin-right": "1rem"}}, "Alt text"),
            alt_text_input,
            file_input,
            h("button", {onclick: submit}, "Submit"),
            error_container,
            result_container,
        );
        fragment.append(form);

        function submit() {
            if (!alt_text_input.value) {
                error_container.textContent = "Alt text required";
                return;
            } else if (!file_input.files.item(0)) {
                error_container.textContent = "File required";
                return;
            }
            error_container.replaceChildren();

            const query_string = new URLSearchParams([
                ["alt_text", alt_text_input.value],
                ["original_file_name", file_input.files.item(0).name],
            ]).toString();
            const request = new Request(`${api_url}/item/${properties.item_id}/image?${query_string}`, {
                method: "POST",
                body: file_input.files.item(0),
            });

            const form_response = form_response_component();
            form_response.append_self(result_container);
            // noinspection JSIgnoredPromiseFromCall
            fetch_checked(request, {
                error_target: error_container,
                response_handler: response => {
                    form_response.callbacks["set_status"](response);
                    return response;
                },
            })
                .then(_body => properties.refetch_images());
        }
    })
    .build();

const item_images_component = component()
    .properties({
        item_id: null,
    })
    .factory(({fragment, properties, add_callback}) => {
        const ol = h("ol");
        const content = h("div", ol);
        const section = h("div", [
            h("hr"),
            h("h2", "Item images"),
            content,
        ]);
        const upload_form = item_image_upload_form({
            refetch_images: fetch,
            item_id: properties.item_id,
        });
        upload_form.append_self(section);
        fragment.appendChild(section);

        add_callback("fetch", fetch);

        function fetch() {
            const request = new Request(`${api_url}/item/${properties.item_id}/image`, {
                method: "GET",
            });
            fetch_checked(request, {error_target: content})
                .then(body => {
                    const json = JSON.parse(body);
                    ol.replaceChildren();
                    if (json.length === 0) {
                        content.append("None");
                        return;
                    }
                    for (let item_image of json) {
                        const element = item_image_element({item_image});
                        element.append_self(ol);
                    }
                })
                .catch(() => null);
        }
    })
    .build();

component()
    .properties({
        text: null,
        item_id: null,
    })
    .factory(({fragment, properties}) => {
        const button = document.createElement("button");
        button.textContent = properties.text;
        button.onclick = () => {
            const right_panel = document.getElementsByClassName("right")
                .item(0);

            if (current_item_images_component !== null) {
                current_item_images_component.remove_self(right_panel);
                current_item_images_component = null;
            }

            if (current_item_images_id === properties.item_id) {
                current_item_images_id = null;
                return;
            }

            current_item_images_component = item_images_component({
                item_id: properties.item_id,
            });
            current_item_images_id = properties.item_id;
            current_item_images_component.append_self(right_panel);
            current_item_images_component.callbacks["fetch"]();
        };

        fragment.appendChild(button);
    })
    .define("x-item-images-button");
