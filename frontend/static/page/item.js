import {component} from "../util/sigma.js";
import {h} from "../util/hyperscript.js";
import {api_url, fetch_checked} from "../util/http.js";

/** @type {ComponentInstance | null} */
let current_item_images_component = null;
/** @type {string | null} */
let current_item_images_id = null;

/**
 * @typedef {{
 *     id: String,
 *     item_id: String,
 *     uri: String,
 *     alt_text: String,
 *     priority: Number,
 *   }} ItemImage
 */

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
            h("a", {href: item_image.uri}, `${item_image.id}`),
            h("span", item_image.alt_text ? ` [${item_image.alt_text}]` : ""),
        );
        fragment.appendChild(element);
    })
    .build();

const item_image_upload_form = component()
    .properties({
        item_id: null,
    })
    .factory(({fragment, properties}) => {
        /** @type HTMLInputElement */
        const file_input = h("input", {type: "file", name: "file"});
        const error_placeholder = h("div");
        const form = h("div",
            h("h3", {style: {"margin-top": ".5rem"}}, "Upload"),
            file_input,
            h("button", {onclick: submit}, "Submit"),
            error_placeholder,
        );
        fragment.append(form);

        function submit() {
            const request = new Request(`${api_url}/item/${properties.item_id}/image`, {
                method: "POST",
                body: file_input.files.item(0),
            });
            fetch_checked(request, {
                error_target: error_placeholder,
                json: true,
            })
                .then(json => {
                    console.log(json);
                });
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
            item_id: properties.item_id,
        });
        upload_form.append_self(section);
        fragment.appendChild(section);

        add_callback("fetch", () => {
            const request = new Request(`${api_url}/item/${properties.item_id}/image`, {
                method: "GET",
            });
            fetch_checked(request, {
                error_target: content,
                json: true,
            })
                .then(json => {
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
        });
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
