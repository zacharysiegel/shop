# API Testing

These cURL configurations expect to be invoked from this directory.

Consult the cURL manual to learn about config files (`-K`) and variables (`--variable`, `--expand-...`).

## Tool

I would like to build a better tool to manage these configurations like Postman.

Desired features:

1. Easy variable overrides, easy default values
2. Write content bodies in YAML, convert to JSON where necessary
3. Print HTTP header responses, status codes, but still be able to pretty print output JSON

## Tips

The `base.curl` configuration adds the `--include` flag in order to show some basic HTTP metadata by default. Frequently, you may want to pipe the response body into another program (e.g. `jq`), but this additional output text may break integrations like this. You can override this flag with the `--no-include` flag as long as `--no-include` appears after `-K/--config` when you invoke `curl`.
