const template = document.createElement("template");
template.innerHTML = `
    <form data-method="PUT">
        <button type="submit" style="margin-right: .5rem;">Publish</button>
    </form>
`;

// noinspection JSUnusedGlobalSymbols
class PublishListingComponent extends HTMLElement {

    /**
     * @type DocumentFragment
     */
    #content;

    connectedCallback() {
        if (this.#content) return;

        this.#content = template.content.cloneNode(true);

        const form = this.#content.querySelector("form");
        form.action = this.getAttribute("action");

        this.appendChild(this.#content);
    }

}

window.customElements.define("x-publish-listing", PublishListingComponent);
