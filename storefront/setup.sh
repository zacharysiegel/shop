#!/bin/zsh

set -e -u -o pipefail

repo_dir=$(git rev-parse --show-toplevel)
cd "$repo_dir/storefront"

if ! npm --version 1>/dev/null 2>&1; then
	echo "The NPM program must be available on the PATH in order to build /lib/component-register"
fi;

pushd ../lib/component-register
npm install
popd

mkdir -p ./static/build
cp ../lib/component-register/dist/component-register.js ./static/build/
