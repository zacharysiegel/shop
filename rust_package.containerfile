from rust:1.88-trixie as compile
arg package_name_arg

# The build context is expected to be the shop repository root
copy ./ /shop
workdir /shop

env PACKAGE_NAME=$package_name_arg
env SQLX_OFFLINE=true

run cargo build --package $package_name_arg --bins --release --locked --target aarch64-unknown-linux-gnu

entrypoint exec "/shop/target/aarch64-unknown-linux-gnu/release/${PACKAGE_NAME}"
