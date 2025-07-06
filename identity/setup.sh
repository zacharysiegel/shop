#!/bin/zsh

set -euo pipefail

master_key="${1?"Argument 1 required: master_key"}"

if ! which yq 1>/dev/null 2>&1; then
	echo 'The `yq` program is required'
	exit 1
fi

if ! which cargo 1>/dev/null 2>&1; then
	echo 'The `cargo` program is required'
	exit 1
fi

repo_path=$(git rev-parse --show-toplevel)
cd "${repo_path}/identity" || exit

properties="$(< ./secret/index.yaml yq '.[]')"

if test -e .env; then
	echo "Environment file identity/.env already exists; Copying to identity/.env.bak;"
	cp .env .env.bak
	rm .env
fi

while IFS= read -r property; do
    echo "Processing $property"

    environment_variable=$(\
        <<< "$property" \
        sed -E -e 's/\./_/g; y/abcdefghijklmnopqrstuvwxyz/ABCDEFGHIJKLMNOPQRSTUVWXYZ/; s/^/AUTHELIA_/; s/$/_FILE/' \
    )
    file_name="${property}.txt"

    echo "${environment_variable}=/secret/${file_name}" >> .env

	secret_name="authelia__${property}"
	secret=$(\
		cargo run -p crypt -- decrypt --key "$master_key" "$secret_name" \
			2> /dev/null \
			| sed -E -e 's/[[:space:]]//g; 1d; 3,$d' # Remove whitespace and select line 2 (1-indexed) (utf-8)
	)
	echo "$secret" > "./secret/${file_name}"
done <<< "$properties"
