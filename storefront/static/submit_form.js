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
    let to_delete = [];
    for (let entry of form_data) {
        const [key, value] = entry;
        if (!value) {
            to_delete.push(key);
        }
    }
    to_delete.forEach(key => {
        form_data.delete(key);
    })

    console.log(form_data); // todo: remove

    const request = new XMLHttpRequest();
    request.open("POST", form.action)
    // request.setRequestHeader("content-type", "multipart/form-data");
    request.setRequestHeader("content-type", "multipart/form-data");
    request.send(form_data);
}

document.addEventListener("submit", submit_form);
