
format:
	cargo +nightly fmt

format-checks:
	cargo +nightly fmt --check

install:
	cargo install --path crates/cli

clippy:
	cargo clippy -- -D warnings

clippy-strict:
	cargo clippy --all-targets --all-features -- -D warnings -D clippy::all -D clippy::pedantic -D clippy::nursery

test:
	cargo test

refresh-crates:
	cargo update

release-workspace:
	cargo workspaces publish --token $(TOKEN) --publish-as-is