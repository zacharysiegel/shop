# Shop

## Setup instructions for MacOS

### Podman setup

Download and install [Podman](https://podman.io).

    brew install podman

Follow the [installation instructions](https://podman.io/docs/installation). You need to install and initialize the Linux virtual machine.

    podman machine init
    podman machine start

Install podman-compose

    brew install podman-compose

### Application setup

Start the application database and the NGINX reverse proxy server.

    # At the repository root
    podman compose up --detach

Run the migrations via the `schema` application.

    cargo run -p schema

Start the inventory management server.

    cargo run -p inventory

Start the web server.

    cargo run -p storefront

### Proxy

The NGINX server manages TLS security concerns. It proxies HTTPS requests via HTTP to the internal services so each internal service is not burdened with SSL certification.

### Authelia

Authentication is proxied through the Authelia server. It uses a session cookie to persist a user's authorization across HTTP connections.
Since the cookie can only apply to a single domain, during local development, if you touch Authelia, only `127.0.0.1` will work (`localhost` will not).
