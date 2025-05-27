import {component} from "../sliver.js";
import {api_url, fetch_checked} from "../http.js";

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
        pre.style.margin = "1rem 0"

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
                    let str = "";
                    // Specific to eBay's inventory location JSON schema
                    for (let location of json["locations"]) {
                        str += JSON.stringify(location) + "\n";
                    }
                    pre.textContent = str.slice(0, -1);
                })
                .catch(() => null);
        }
    })
    .define("x-ebay-locations");
