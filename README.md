# Shop

## Setup instructions for MacOS

### Podman setup

Download and install [Podman](https://podman.io).

    brew install podman

Follow the [installation instructions](https://podman.io/docs/installation). You need to install and initialize the
Linux virtual machine.

    podman machine init
    podman machine start

Install podman-compose

    brew install podman-compose

### Application setup

Start the application database.

    # At the repository root
    podman compose up -d

Run the migrations via the `schema` application.

    cargo run -p schema

Start the inventory management server.

    cargo run -p inventory

Start the web server.

    cargo run -p storefront
