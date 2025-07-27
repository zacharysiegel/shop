log_dir=/tmp/ro/zach/shop
release_dir=./target/aarch64-apple-darwin/release
env=RUNTIME_ENVIRONMENT=production

.PHONY: default
default:
	echo 'Use the `exec-release` target to launch `inventory` and `frontend` for deployed environments.'

.PHONY: compile
compile-release:
	SQLX_OFFLINE=true cargo build --package inventory --bins --release --locked --target aarch64-apple-darwin
	SQLX_OFFLINE=true cargo build --package frontend --bins --release --locked --target aarch64-apple-darwin

.PHONY: exec
exec-release: compile-release
	export RUNTIME_ENVIRONMENT=production
	mkdir -p $(log_dir)/inventory
	touch $(log_dir)/inventory/stdout.log
	touch $(log_dir)/inventory/stderr.log
	$(env) $(release_dir)/inventory 1> $(log_dir)/inventory/stdout.log 2> $(log_dir)/inventory/stderr.log &
	ps aux | grep '[i]nventory'

	mkdir -p $(log_dir)/frontend
	touch $(log_dir)/frontend/stdout.log
	touch $(log_dir)/frontend/stderr.log
	$(env) $(release_dir)/frontend 1> $(log_dir)/frontend/stdout.log 2> $(log_dir)/frontend/stderr.log &
	ps aux | grep '[f]rontend'
