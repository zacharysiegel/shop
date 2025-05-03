/**
 * @typedef {object} Category
 * @property {string} id
 * @property {string} display_name
 */

/**
 * @param {HTMLElement} element
 * @param {string} base_url
 * @param {string} product_id
 * @param {Array<Category>} categories
 */
function activate_categories(element, base_url, product_id, categories) {
    const section = element.getElementsByTagName("section")[0];
    section.replaceChildren();

    if (categories.length === 0) {
        section.innerText = "None";
        return;
    }

    for (const category of categories) {
        const category_entry = create_category_entry(base_url, product_id, category);
        section.appendChild(category_entry);
    }
}

/**
 * @param {string} base_url
 * @param {string} product_id
 * @param {Category} category
 * @return HTMLDivElement
 */
function create_category_entry(base_url, product_id, category) {
    const container = document.createElement("div");
    container.style.display = "flex";
    container.style.flexDirection = "row";
    container.style.justifyContent = "start";
    container.style.alignItems = "center";

    const form = document.createElement("form");
    form.action = `${base_url}/product/${product_id}/category/${category.id}`;
    form.dataset["method"] = "DELETE";

    const button = document.createElement("button");
    button.type = "submit";
    button.innerText = "x";
    button.style.marginRight = ".5rem";

    const p = document.createElement("p");
    p.innerText = category.display_name;

    form.appendChild(button);
    container.appendChild(form);
    container.appendChild(p);

    return container;
}
