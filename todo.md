# TODO

## eBay webhook

[Platform Notifications](https://developer.ebay.com/api-docs/static/platform-notifications-landing.html)

## Public TLS certificate

* Write setup.sh for proxy
    1. Create domain.crt and domain.key files
        * Stored at ~/cert/* on venus.
    2. Create nginx.<env>.conf
    3. Create profile-specific nginx docker container specifications in compose.template.yaml
