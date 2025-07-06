#!/bin/zsh

set -e -u -o pipefail

master_key="${1?"Argument 1 required: master_key"}"

repo_dir=$(git rev-parse --show-toplevel)
cd "${repo_dir}"

git submodule init
git submodule update

echo "Generating compose.yaml"
postgres__user_shop_password_key="postgres__user.shop.password"
postgres__user_shop_password=$(
	cargo run -p crypt -- decrypt --key "$master_key" "$postgres__user_shop_password_key" \
		2>/dev/null |
		sed -E -e 's/[[:space:]]//g; 1d; 3,$d' # Remove whitespace and select line 2 (1-indexed) (utf-8)
);
sed -e "s/${postgres__user_shop_password_key}/${postgres__user_shop_password}/g" ./compose.yaml.template > ./compose.yaml

# All setup scripts should be idempotent and callable from the repo root directory
zsh ./identity/setup.sh "$master_key"
