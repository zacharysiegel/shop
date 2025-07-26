#!/bin/zsh

environments=("local" "stage" "production")

function prepare_env_file {
	if test -e .env; then
		echo "Environment file identity/.env already exists; Copying to identity/.env.bak;"
		cp .env .env.bak
		rm .env
	fi
}

function generate_secrets {
	properties="$(yq < ./secret/index.yaml '.[]')"

	while IFS= read -r property; do
		echo "Processing $property"

		environment_variable=$(
			sed <<< "$property" \
				-E -e 's/\./_/g; y/abcdefghijklmnopqrstuvwxyz/ABCDEFGHIJKLMNOPQRSTUVWXYZ/; s/^/AUTHELIA_/; s/$/_FILE/'
		)
		file_name="${property}.txt"

		echo "${environment_variable}=/secret/${file_name}" >> .env

		secret_name="authelia__${property}"
		secret=$(
			cargo run -p crypt -- decrypt --utf8 --key "$master_key" "$secret_name"
		)
		echo "$secret" > "./secret/${file_name}"
	done <<< "$properties"
}

function generate_configurations {
	configuration_dir="${repo_path}/identity/config/"
	configuration_path_template="${configuration_dir}/configuration.template.yaml"

	session_cookies_0_domain="session.cookies.0.domain"
	session_cookies_0_authelia_url="session.cookies.0.authelia_url"
	session_cookies_0_default_redirection_url="session.cookies.0.default_redirection_url"

	echo "Generating environment-specific configuration files from ${configuration_path_template}"
	for environment in "${environments[@]}"; do
		local configuration_path_env="${configuration_dir}/configuration.${environment}.yaml"
		cp "$configuration_path_template" "$configuration_path_env"

		sed -I "" -E \
			-e "s/ENVIRONMENT/${environment}/g" \
			"$configuration_path_env"
	done

	# production
	sed -I "" -E \
		-e "s/${session_cookies_0_domain}/shop.zach.ro/g" \
		-e "s/${session_cookies_0_authelia_url}/https:\/\/shop.zach.ro\/auth/g" \
		-e "s/${session_cookies_0_default_redirection_url}/https:\/\/shop.zach.ro\/admin/g" \
		"${configuration_dir}/configuration.production.yaml"

	# stage
	sed -I "" -E \
		-e "s/${session_cookies_0_domain}/shop-stage.zach.ro/g" \
		-e "s/${session_cookies_0_authelia_url}/https:\/\/shop-stage.zach.ro\/auth/g" \
		-e "s/${session_cookies_0_default_redirection_url}/https:\/\/shop-stage.zach.ro\/admin/g" \
		"${configuration_dir}/configuration.stage.yaml"

	# local
	sed -I "" -E \
		-e "s/${session_cookies_0_domain}/127.0.0.1/g" \
		-e "s/${session_cookies_0_authelia_url}/https:\/\/127.0.0.1:1443\/auth/g" \
		-e "s/${session_cookies_0_default_redirection_url}/https:\/\/127.0.0.1:1443\/admin/g" \
		"${configuration_dir}/configuration.local.yaml"
}

set -euo pipefail

master_key="${1?"Argument 1 required: master_key"}"

if ! which yq 1> /dev/null 2>&1; then
	echo 'The `yq` program is required'
	exit 1
fi

repo_path=$(git rev-parse --show-toplevel)
cd "${repo_path}/identity" || exit

prepare_env_file
generate_secrets
generate_configurations
