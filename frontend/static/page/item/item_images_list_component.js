import {component} from "../../util/sigma.js";
import {api_url, fetch_checked} from "../../util/http.js";
import h from "../../util/hyperscript.js";
import {item_image_list_item_component} from "./item_image_list_item_component.js";
import {item_image_upload_form_component} from "./item_image_upload_form_component.js";

export const item_images_list_component = component()
    .properties({
        item_id: null,
    })
    .factory(({fragment, properties, add_callback}) => {
        const ol = h("ol");
        const content = h("div", ol);
        const upload_form = item_image_upload_form_component({
            refetch_images: fetch,
            item_id: properties.item_id,
        });
        const section = h("div", [
            h("hr"),
            h("h2", "Item images"),
            content,
            upload_form.elements.at(0),
        ]);
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
                        const element = item_image_list_item_component({item_image});
                        element.append_self(ol);
                    }
                })
                .catch(() => null);
        }
    })
    .build();
