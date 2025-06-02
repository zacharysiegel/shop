import {component} from "../sliver.js";

component()
    .properties({action: undefined})
    .factory(({fragment, properties}) => {
        const inner_element = document.createElement("div");
        inner_element.innerHTML = `
            <form data-method="POST">
                <button type="submit" style="margin-right: .5rem;">Publish</button>
            </form>
        `;

        const form = inner_element.querySelector("form");
        form.action = properties.action;

        fragment.appendChild(inner_element);
    })
    .define("x-publish-listing");

component()
    .properties({action: undefined})
    .factory(({fragment, properties}) => {
        const div = document.createElement("div");
        div.append((() => {
            const form = document.createElement("form");
            form.setAttribute("data-method", "POST");
            form.action = properties.action;
            form.append((() => {
                const button = document.createElement("button");
                button.type = "submit";
                button.style.marginRight = ".5rem";
                button.textContent = "Cancel";
                return button;
            })())
            return form;
        })())

        fragment.appendChild(div);
    })
    .define("x-cancel-listing");
