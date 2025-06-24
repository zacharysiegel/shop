export const api_url = "https://127.0.0.1:1443/api";

/**
 * Perform a "fetch" and siphon any errors into the {@link textContent} property of the given error target (if applicable).
 * @param {Request} request
 * @param {Object} options
 * @property {Node} [error_target = null]
 * @property {boolean} [json = false] Iff true, the response body will be parsed as JSON
 * @return {Promise<any | string>}
 */
export const fetch_checked = (request, {
    error_target,
    json,
} = {
    error_target: null,
    json: false,
}) => {
    return fetch(request)
        .then(response => {
            if (!response.ok) {
                if (error_target) error_target.textContent = `Error: ${response.status} ${response.statusText}`;
                return Promise.reject(); // Will trigger the outer onRejected callback with error=undefined
            }

            return (json ? response.json() : response.text())
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
