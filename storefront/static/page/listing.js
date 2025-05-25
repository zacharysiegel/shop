import {register} from "../build/component-register.js";

register("x-publish-listing")((props, {element}) => {
    const inner_element = document.createElement("div");
    inner_element.innerHTML = `
        <form data-method="PUT">
            <button type="submit" style="margin-right: .5rem;">Publish</button>
        </form>
    `;

    const form = inner_element.querySelector("form");
    form.action = element.getAttribute("action");

    element.appendChild(inner_element);
});
