import {component} from "../../util/sigma.js";
import {base_url} from "../../util/http.js";
import h from "../../util/hyperscript.js";

/**
 * @param {ItemImage} item_image
 * @return String
 */
function get_image_uri(item_image) {
    return `${base_url}/volatile/images/${item_image.item_id}_${item_image.id}_${item_image.original_file_name}`;
}

export const item_image_list_item_component = component()
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
