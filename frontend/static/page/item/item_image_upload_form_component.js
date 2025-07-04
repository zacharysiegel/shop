import {component} from "../../util/sigma.js";
import {api_url, fetch_checked} from "../../util/http.js";
import h from "../../util/hyperscript.js";
import {form_response_component} from "../../util/submit_form.js";

export const item_image_upload_form_component = component()
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
