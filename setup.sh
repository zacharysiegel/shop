#!/bin/zsh

set -e -u -o pipefail

master_key="${1?"Argument 1 required: master_key"}" # todo: pull master key from .env file
environments=("local" "stage" "production")
package_names=("inventory" "frontend")
repo_dir=$(git rev-parse --show-toplevel)

cd "${repo_dir}"

git submodule init
git submodule update

if ! which cargo 1> /dev/null 2>&1; then
	echo 'The `cargo` program is required'
	exit 1
fi

function sqlx_setup {
	echo 'Installing the SQLx CLI onto the system (used for caching database state for query validations)'
	echo 'You can run this command to check the status of ./.sqlx: `cargo sqlx prepare --workspace --check -- --all-targets --all-features`'
	echo '...or to regenerate ./.sqlx: `cargo sqlx prepare --workspace -- --all-targets --all-features`'
	cargo install sqlx-cli
}
sqlx_setup

function generate_env_from_template {
	echo "Generating .env"
	local postgres__user_shop_password_local_key="postgres__user.shop.password.local"
	local postgres__user_shop_password_local=$(
		cargo run -p crypt -- decrypt --utf8 --key "$master_key" "$postgres__user_shop_password_local_key"
	)

	sed > ./.env \
		-E \
		-e "s/^(MASTER_SECRET=).*$/\1${master_key}/g" \
		-e "s/${postgres__user_shop_password_local_key}/${postgres__user_shop_password_local}/g" \
		./template.env
}
generate_env_from_template

function generate_compose_from_template {
	echo "Generating compose.yaml"
	cp ./compose.template.yaml ./compose.yaml

	for environment in "${environments[@]}"; do
		local postgres__user_shop_password_env_key="postgres__user.shop.password.${environment}"
		local postgres__user_shop_password_env=$(
			cargo run -p crypt -- decrypt --utf8 --key "$master_key" "$postgres__user_shop_password_env_key"
		)
		echo "s/${postgres__user_shop_password_env_key}/${postgres__user_shop_password_env}/g"
		sed -E -I "" \
			-e "s/${postgres__user_shop_password_env_key}/${postgres__user_shop_password_env}/g" \
			./compose.yaml
	done
}
generate_compose_from_template

# All setup scripts should be idempotent and callable from the repo root directory
zsh ./identity/setup.sh "$master_key"
zsh ./proxy/setup.sh "$master_key"
