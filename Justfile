test:
    cargo test

lint:
    cargo clippy

fmt:
    cargo fmt

release:
    cargo build --release

all:
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo build --all-features --locked
    cargo test --all-features --locked