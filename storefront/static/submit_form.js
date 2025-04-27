/**
 * @param {SubmitEvent} submit_event
 */
function submit_form(submit_event) {
    submit_event.preventDefault();
    submit_event.stopPropagation();
    /**
     * @type {HTMLFormElement}
     */
    const form = submit_event.target;

    const form_data = new FormData(form, submit_event.submitter);
    const form_data_as_object = {};
    for (let entry of form_data) {
        const [key, value] = entry;
        if (value) {
            form_data_as_object[key] = value;
        }
    }

    /* By default, FormData is converted to the format "multipart/form-data". This representation
        adds significant bloat when each field's data is encoded in UTF-8 and is generally small.
        We convert to JSON instead. */
    const request = new XMLHttpRequest();
    request.open("POST", form.action)
    request.setRequestHeader("content-type", "application/json");
    request.send(JSON.stringify(form_data_as_object));
}

document.addEventListener("submit", submit_form);
