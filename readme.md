# Shop

## Setup

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

Run the top-level setup script (requires zsh). This script will invoke any requisite nested setup scripts.

    # At the repository root
    zsh ./setup.sh

Start the application database and the NGINX reverse proxy server.

    # At the repository root
    podman compose up --detach

Execute `./schema/migrations/manual.sql` manually, then run the automatic migrations via the `schema` application.

    cargo run -p schema

Start the inventory management server.

    cargo run -p inventory

Start the web server.

    cargo run -p frontend

## Deployment

### Deploy to Venus

"Venus" is the name of the Mac Mini (M1) currently used to host the servers for the production (and eventually stage) environments.

To deploy the application to this computer, first SSH into the machine:

    ssh zachary@ssh.zach.ro

Launch the Docker containers:

    podman compose --profile production up -d

Start the Rust servers:

    make exec-release

## Other information

### Proxy

The NGINX server manages TLS security concerns. It proxies HTTPS requests via HTTP to the internal services so each internal service is not burdened with SSL certification.

Even during local development, you should access web pages through the proxy. Otherwise the browser will punish you with CORS errors.

### Authelia

Authentication is proxied through the [Authelia](https://www.authelia.com/overview/prologue/introduction/) server. It uses a session cookie to persist a user's authorization across HTTP connections.
Since the cookie can only apply to a single domain, during local development, if you touch Authelia, only `127.0.0.1` will work (`localhost` will not).

Validate the Authelia configuration using the `authelia-validate` container specified in the compose file. (It points to "local" by default.)

### Environment variables

Environment variables are specified to the applications by `.env`. Since this file may include secrets, it is generated dynamically by the setup script.
If you need to add an environment variable, add it to `template.env`.
If your variable's value is a secret, encrypt it using `cargo run -p crypt -- encrypt ...` and add the encrypted entry to `crypt`'s data store. Decrypt the value in the setup script following the pattern used for `postgres__user.shop.password.local`.

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

<table>
<tr>
    <td>".env file"</td>
    <td><code>*.env</code></td>
</tr>
</table>

### Cloudflare Tunnel

Our server does not have access to a static IP address, so we use a Cloudflare tunnel to expose our services to the public Internet. The tunnel operates by using the `cloudflared` program (running on our own server) establishing a long-lived TCP connection with a Cloudflare server which acts as an application-level router. Rather than directing a DNS record to the NGINX reverse proxy server, our DNS records point to this Cloudflare server which is configured to forward traffic to the "local" `cloudflared` process. The `cloudflared` program itself may be configured to foward traffic to the NGINX process at a local IP address.

We use the Cloudflare tunnel for both public application routing and internal SSH access.

#### Cloudflare tunnel setup

A tunnel called "venus" has already been created in the Cloudflare ZeroTrust console.

1. `cloudflared tunnel login`
    * select "venus" tunnel in browser
    * `clourflared` creates ~/.cloudflared/cert.pem
2. `cloudflared tunnel route ip add 127.0.0.1/32 venus`

### SQLx

We use [the SQLx library](https://github.com/launchbadge/sqlx) to construct and statically analyze SQL queries to the database.

During development, SQLx can connect to a live database server during compilation in order to verify syntactical validity of SQL queries written within strings within Rust macros. When building container images for the Rust applications, the compiler does not have access to the database server (nor should it), so we use the `SQLX_OFFLINE=true` environment variable to direct SQLx to read a cached version of database state instead. This cache is checked in to version control at `/.sqlx`.

*Important:* Any modification to the database schema must be followed by manual regeneration of the SQLx cache. Fail to do so, and compilations during container image builds will fail.

    cargo sqlx prepare --workspace -- --all-targets --all-features
