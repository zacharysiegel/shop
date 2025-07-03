export const base_url = "https://127.0.0.1:1443";
export const api_url = `${base_url}/api`;

/**
 * Perform a "fetch" and siphon any errors into the {@link textContent} property of the given error target (if applicable).
 * @param {Request} request
 * @param {Object} options
 * @property {Node} [error_target = null]
 * @property {(Response) => Response} [response_handler = (response) => response] Optional hook for processing the raw {@link Response} object. (Do not consume the response body)
 * @return {Promise<any | string>}
 */
export const fetch_checked = (request, {
    error_target,
    response_handler,
} = {
    error_target: null,
    response_handler: response => response,
}) => {
    return fetch(request)
        .then(response_handler)
        .then(response => {
            if (!response.ok) {
                if (error_target) error_target.textContent = `Error: ${response.status} ${response.statusText}`;
                return Promise.reject(); // Will trigger the outer onRejected callback with error=undefined
            }

            return (response.text())
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
