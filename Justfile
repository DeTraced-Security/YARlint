test:
    cargo test

lint:
    cargo clippy

fmt:
    cargo fmt

release:
    cargo build --release --target x86_64-pc-windows-gnu
    cargo build --release --target x86_64-unknown-linux-gnu
    cargo zigbuild --release --target x86_64-apple-darwin

all:
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo build --all-features --locked
    cargo test --all-features --locked
    cargo llvm-cov --fail-under-lines 95 --fail-under-functions 95