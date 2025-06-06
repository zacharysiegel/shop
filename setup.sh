#!/bin/zsh

set -e -u -o pipefail

master_key="${1?"Argument 1 required: master_key"}"

repo_dir=$(git rev-parse --show-toplevel)
cd "${repo_dir}"

git submodule init
git submodule update

# All setup scripts should be idempotent and callable from the repo root directory
zsh ./identity/setup.sh "$master_key"
