import {component} from "../../util/sigma.js";
import {api_url, base_url, fetch_checked} from "../../util/http.js";
import h from "../../util/hyperscript.js";
import {form_response_component} from "../../util/submit_form.js";

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
        refetch_images: null,
    })
    .factory(({fragment, properties}) => {
        const {
            /** @type ItemImage */
            item_image,
            /** @type {() => void} */
            refetch_images,
        } = properties;

        const x_button = h(
            "button",
            {
                type: "button",
                onclick: delete_image,
                style: {
                    "display": "inline",
                    "margin-left": "1rem",
                },
            },
            "X",
        );
        const error_target = h("div");
        const result_target = h("div");
        const element = h("li",
            h("a", {href: get_image_uri(item_image), target: "_blank"}, `${item_image.id}`),
            h("span", item_image.alt_text ? ` [${item_image.alt_text}]` : ""),
            x_button,
            error_target,
            result_target,
        );

        fragment.appendChild(element);

        function delete_image() {
            const form_response = form_response_component();
            form_response.append_self(result_target);

            const request = new Request(
                `${api_url}/item/${item_image.item_id}/image/${item_image.id}`,
                {method: "DELETE"},
            );
            fetch_checked(request, {
                error_target,
                response_handler: (response) => {
                    form_response.callbacks["set_status"](response);
                    return response;
                },
            })
                .then(refetch_images);
        }
    })
    .build();
