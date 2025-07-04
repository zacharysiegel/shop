import {component} from "../../util/sigma.js";
import {item_images_list_component} from "./item_images_list_component.js";

/** @type {ComponentInstance | null} */
let current_item_images_component = null;
/** @type {string | null} */
let current_item_images_id = null;

export const item_images_button_component = component()
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

            current_item_images_component = item_images_list_component({
                item_id: properties.item_id,
            });
            current_item_images_id = properties.item_id;
            current_item_images_component.append_self(right_panel);
            current_item_images_component.callbacks["fetch"]();
        };

        fragment.appendChild(button);
    })
    .define("x-item-images-button");
