export const api_url = "https://127.0.0.1:1443/api";

/**
 * @param {Request} request
 * @param {Object} options
 * @property {Node} [error_target = null]
 * @property {boolean} [parse = false]
 */
export const fetch_checked = (request, {
    error_target,
    parse,
} = {
    error_target: null,
    parse: false,
}) => {
    return fetch(request)
        .then(response => {
            if (!response.ok) {
                if (error_target) error_target.textContent = `Error: ${response.status} ${response.statusText}`;
                return Promise.reject(); // Will trigger the outer onRejected callback with error=undefined
            }

            return (parse ? response.json() : response.text())
                .catch(error => {
                    if (error_target) error_target.textContent = String(error);
                    return Promise.reject(error);
                });
        })
        .catch(error => {
            if (error && error_target) error_target.textContent = String(error);
            if (error) console.error(error);
            return Promise.reject(error);
        });
};
