import {component} from "../util/sigma.js";
import {api_url, fetch_checked} from "../util/http.js";

component()
    .factory(({fragment}) => {
        const h2 = document.createElement("h2");
        h2.textContent = "Inventory locations";

        const refresh_button = document.createElement("button");
        refresh_button.textContent = "Refresh";
        refresh_button.onclick = () => {
            pre.textContent = "Pending.";
            get();
        };

        const pre = document.createElement("pre");
        pre.textContent = "Pending.";
        pre.style.margin = ".5rem 0"

        fragment.append(h2, refresh_button, pre);

        get();
        function get() {
            const request = new Request(`${api_url}/ebay/location`, {
                method: "GET",
            });
            fetch_checked(request, {
                error_target: pre,
            })
                .then(body => {
                    const json = JSON.parse(body);
                    let str = `n: ${json["total"]}`;
                    // Specific to eBay's inventory location JSON schema
                    for (let location of json["locations"]) {
                        str += "\n" + JSON.stringify(location);
                    }
                    pre.textContent = str;
                })
                .catch(() => null);
        }
    })
    .define("x-ebay-locations");
