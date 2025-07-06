# Shop

## Setup instructions

_This setup has only been tested on macOS and may not "just work" in a different environment._

### Podman setup

Download and install [Podman](https://podman.io).

    brew install podman

Follow the [installation instructions](https://podman.io/docs/installation). You need to install and initialize the Linux virtual machine.

    podman machine init
    podman machine start

Install podman-compose

    brew install podman-compose

### Rust setup

Follow the official [Rust installation guide](https://www.rust-lang.org/tools/install).
Ensure the `cargo` program is accessible in your shell.

### Application setup

Start the application database and the NGINX reverse proxy server.

    # At the repository root
    podman compose up --detach


Execute `./schema/migrations/manual.sql` manually, then run the automatic migrations via the `schema` application.

    cargo run -p schema

Start the inventory management server.

    cargo run -p inventory

Start the web server.

    cargo run -p frontend

### Proxy

The NGINX server manages TLS security concerns. It proxies HTTPS requests via HTTP to the internal services so each internal service is not burdened with SSL certification.

Even during local development, you should access web pages through the proxy. Otherwise the browser will punish you with CORS errors.

### Authelia

Authentication is proxied through the [Authelia](https://www.authelia.com/overview/prologue/introduction/) server. It uses a session cookie to persist a user's authorization across HTTP connections.
Since the cookie can only apply to a single domain, during local development, if you touch Authelia, only `127.0.0.1` will work (`localhost` will not).

### Environment variables

Environment variables are specified in `.env`. If you need to add an environment variable, add it to both `.env` and `.env.template`. If the value is secret, still add the variable declaration to the template file, but leave it unassigned. (e.g. `TWITTER_API_TOKEN=`)

### Secrets

The `crypt` application/library manages nearly all secrets within the application. The ciphertext for application secrets (e.g. the eBay client secret) are stored within `crypt`. The secrets can be manually decrypted using the CLI provided by `crypt`. They are decrypted within the other applications via the `MASTER_SECRET` environment variable.

You must be given the master secret and save it into your local `.env` file (not committed to Git) before you can run the server properly.

To add a new secret, use the `crypt` CLI:

	cargo run -p crypt -- encrypt "$plaintext" --key "$master_key"

The command produces literal Rust code which should be copied and pasted into `crypt/src/data.rs`.

### Venus

_"Venus" is the name of the machine (or cluster of machines) which hosts the production environment_

### IntellIJ

For convenience, you may want to add the following entries to the "Editor" > "File Types" > "Recognized File Types":
    * "DockerIgnore"
        * `*.containerignore`
    * "YAML"
        * `*.yaml.template`
        * `*.yml.template`
    * ".env file"
        * `.env.template`

todo: talk about cloudflare tunnel and ssh connection via `cloudflared`

scratch:
cloudflared tunnel login
    select "venus" tunnel in browser
    clourflared creates ~/.cloudflared/cert.pem
cloudflared tunnel route ip add 127.0.0.1/32 venus

