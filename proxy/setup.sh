#!/bin/zsh

set -e -u -o pipefail

master_key="${1?"Argument 1 required: master_key"}"

repo_dir=$(git rev-parse --show-toplevel)
cd "${repo_dir}/proxy"

function generate_secret_certificates {
	production_path="$(realpath ./cert)/production"
	crt_file_name="domain.crt"
	key_file_name="domain.key"

	echo "Decrypting production TLS certificates at ${production_path}"
	mkdir -p "$production_path"
	cargo run -p crypt -- decrypt --utf8 --key "$master_key" "proxy__cert.shop.certificate" \
		> "${production_path}/${crt_file_name}"
	cargo run -p crypt -- decrypt --utf8 --key "$master_key" "proxy__cert.shop.key" \
		> "${production_path}/${key_file_name}"
}
generate_secret_certificates
