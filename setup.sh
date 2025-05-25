#!/bin/zsh

set -e -u -o pipefail

repo_dir=$(git rev-parse --show-toplevel)
cd "${repo_dir}"

git submodule init
git submodule update

# All setup scripts should be idempotent and callable from the repo root directory
zsh ./identity/setup.sh # todo: master_secret missing
zsh ./storefront/setup.sh
