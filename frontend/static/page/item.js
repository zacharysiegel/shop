import {component} from "../util/sigma.js";

const item_images_component = component()
    .factory(({fragment}) => {
        const inner = document.createElement("div");
        inner.textContent = "_test_";

        fragment.appendChild(inner);
    })
    .build();

component()
    .factory(({fragment, properties, add_callback}) => {
        let is_open = false;
        let item_images = item_images_component();

        const button = document.createElement("button");
        button.textContent = properties.text;
        button.onclick = () => {
            const right_panel = document.getElementsByClassName("right")
                .item(0);

            if (is_open) {
                item_images.remove_self(right_panel);
                is_open = false;
            } else {
                item_images.append_self(right_panel);
                is_open = true;
            }
        };

        console.log(properties.item_id);

        fragment.appendChild(button);
    })
    .properties({
        text: null,
        item_id: null,
    })
    .define("x-item-images-button");
