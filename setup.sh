#!/bin/zsh

set -e -u -o pipefail

master_key="${1?"Argument 1 required: master_key"}"

repo_dir=$(git rev-parse --show-toplevel)
cd "${repo_dir}"

git submodule init
git submodule update

if ! which cargo 1> /dev/null 2>&1; then
	echo 'The `cargo` program is required'
	exit 1
fi

postgres__user_shop_password_key="postgres__user.shop.password"
postgres__user_shop_password=$(
	cargo run -p crypt -- decrypt --utf8 --key "$master_key" "$postgres__user_shop_password_key"
)

echo "Generating .env"
sed > ./.env \
	-E \
	-e "s/^(MASTER_SECRET=).*$/\1${master_key}/g" \
	-e "s/${postgres__user_shop_password_key}/${postgres__user_shop_password}/g" \
	./.env.template

echo "Generating compose.yaml"
sed -e "s/${postgres__user_shop_password_key}/${postgres__user_shop_password}/g" ./compose.template.yaml > ./compose.yaml

# All setup scripts should be idempotent and callable from the repo root directory
zsh ./identity/setup.sh "$master_key"
zsh ./proxy/setup.sh "$master_key"
