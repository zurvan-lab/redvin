#  Developer tools
devtools:
	rustup component add clippy

# Testing
test:
	cargo test --all

# Format and linting
fmt:
	cargo fmt

check:
	cargo check
	cargo clippy --all-targets --all-features
	cargo fmt --check
