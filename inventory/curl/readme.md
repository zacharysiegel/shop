# API Testing

These cURL configurations expect to be invoked from this directory.

Consult the cURL manual to learn about config files (`-K`) and variables (`--variable`, `--expand-...`).

## Tool

I would like to build a better tool to manage these configurations like Postman.

Desired features:

1. Easy variable overrides, easy default values
2. Write content bodies in YAML, convert to JSON where necessary
3. Print HTTP header responses, status codes, but still be able to pretty print output JSON
