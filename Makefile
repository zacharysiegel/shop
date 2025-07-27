.PHONY: default
default:
	echo 'Use the `exec-release` target to launch `inventory` and `frontend` for deployed environments.'

.PHONY: check-cargo
check-cargo:
	if ! which cargo 1> /dev/null 2>&1; then \
		echo 'The `cargo` program is required'; \
		exit 1; \
	fi

.PHONY: check
check: check-cargo

.PHONY: compile
compile-release:
	SQLX_OFFLINE=true cargo build --package inventory --bins --release --locked --target aarch64-apple-darwin
	SQLX_OFFLINE=true cargo build --package frontend --bins --release --locked --target aarch64-apple-darwin

.PHONY: exec
exec-release: check compile-release
	./target/aarch64-apple-darwin/release/inventory &
	disown %+
	./target/aarch64-apple-darwin/release/frontend &
	disown %+
