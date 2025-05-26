import {component} from "../sliver.js";
import {api_url, fetch_checked} from "../http.js";

component()
    .factory(({fragment}) => {
        const h2 = document.createElement("h2");
        h2.textContent = "Inventory locations";

        const pre = document.createElement("pre");
        pre.textContent = "Pending.";

        fragment.append(h2, pre);

        const request = new Request(`${api_url}/ebay/location`, {
            method: "GET",
        });
        fetch_checked(request, {
            error_target: pre,
        })
            .then(body => pre.textContent = String(body))
            .catch(() => null);
    })
    .define("x-ebay-locations");
