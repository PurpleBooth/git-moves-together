# This help screen
show-help:
	just --list

# Test it was built ok
test:
	RUST_BACKTRACE=1 cargo test

# Test the markdown in the docs directory
specdown: build
    ./test-harness/specdown ./README.md

# Build release version
build:
	cargo build --release

# Build docker image
docker-build:
	docker build -t purplebooth/git-moves-together:latest .

# Lint it
lint:
	cargo fmt --all -- --check
	cargo clippy --all-features
	cargo check
	cargo audit

# Format what can be formatted
fmt:
	cargo fix --allow-dirty --allow-staged
	cargo clippy --allow-dirty --allow-staged --fix --all-features
	cargo fmt --all

# Clean the build directory
clean:
	cargo clean
