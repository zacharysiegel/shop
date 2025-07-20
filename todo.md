# TODO

## eBay webhook

[Platform Notifications](https://developer.ebay.com/api-docs/static/platform-notifications-landing.html)

## Public TLS certificate

* Write setup.sh for proxy
    1. [done] Create domain.crt and domain.key secrets
    2. [done] Create domain.crt and domain.key files
        * Stored at ~/cert/* on venus.
    3. [done] Create nginx.<env>.conf
    4. [done] Create profile-specific nginx docker container specifications in compose.template.yaml
