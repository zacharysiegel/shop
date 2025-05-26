import {component} from "../sliver.js";

component()
    .properties({action: undefined})
    .factory(({fragment, properties}) => {
        const inner_element = document.createElement("div");
        inner_element.innerHTML = `
            <form data-method="PUT">
                <button type="submit" style="margin-right: .5rem;">Publish</button>
            </form>
        `;

        const form = inner_element.querySelector("form");
        form.action = properties.action;

        fragment.appendChild(inner_element);
    })
    .define("x-publish-listing")
    .build();
