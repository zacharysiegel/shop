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

function generate_nginx_env_configurations {
	template_file_name="nginx.template.conf"
	local_file_name="nginx.local.conf"
	stage_file_name="nginx.stage.conf"
	production_file_name="nginx.production.conf"

	sed > "./${local_file_name}" \
		-e "s/environment.name/local/g" \
		"$template_file_name"
	sed > "./${stage_file_name}" \
		-e "s/environment.name/stage/g" \
		"$template_file_name"
	sed > "./${production_file_name}" \
		-e "s/environment.name/production/g" \
		"$template_file_name"
}
generate_nginx_env_configurations
