# Development Setup

This guide walks through setting up a local development environment for YARlint. By the end, you should be able to build the project, run the test suite, and verify your changes before opening a pull request.

## Prerequisites

Before cloning the repository, install the following tools.

### Git

Git is required to clone the repository and submit pull requests.

Verify your installation:

```bash
git --version
```

### Rust

YARlint is written in Rust and follows the latest stable toolchain.

Install Rust using `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify the installation:

```bash
rustc --version
cargo --version
```

If Rust is already installed, update to the latest stable release:

```bash
rustup update stable
```

---

## Clone the Repository

Fork the repository on GitHub, then clone your fork.

```bash
git clone https://github.com/<your-username>/YARlint.git
cd YARlint
```

Add the upstream repository so your fork stays synchronized.

```bash
git remote add upstream https://github.com/DeTraced-Security/YARlint.git
```

Verify your remotes:

```bash
git remote -v
```

Example output:

```text
origin    https://github.com/<your-username>/YARlint.git
upstream  https://github.com/DeTraced-Security/YARlint.git
```

---

## Build the Project

Compile YARlint.

```bash
cargo build
```

A successful build should finish without warnings or errors.

---

## Run the Test Suite

Execute every unit and integration test.

```bash
cargo test
```

Every test must pass before opening a pull request.

---

## Run the Formatter

YARlint uses the standard Rust formatter.

Check formatting:

```bash
cargo fmt --all -- --check
```

Automatically format files:

```bash
cargo fmt --all
```

---

## Run Clippy

All pull requests must pass Clippy with warnings treated as errors.

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

---

## Check Test Coverage

YARlint maintains high code coverage.

Install `cargo-llvm-cov` once:

```bash
cargo install cargo-llvm-cov
```

Run coverage locally:

```bash
cargo llvm-cov
```

To verify against CI thresholds:

```bash
cargo llvm-cov \
    --fail-under-lines 95 \
    --fail-under-functions 95
```

---

## Generate Documentation

Build the project documentation.

```bash
cargo doc --no-deps --document-private-items
```

Open it locally:

```bash
cargo doc --open
```

---

## Recommended Development Workflow

A typical contribution looks like this:

```text
Create Issue
      │
      ▼
Fork Repository
      │
      ▼
Create Branch
      │
      ▼
Implement Change
      │
      ▼
Run fmt
      │
      ▼
Run clippy
      │
      ▼
Run tests
      │
      ▼
Run coverage
      │
      ▼
Open Pull Request
```

---

## Keeping Your Fork Updated

Fetch the latest changes from the upstream repository.

```bash
git fetch upstream
```

Switch to your local main branch.

```bash
git checkout main
```

Merge the latest upstream changes.

```bash
git merge upstream/main
```

Push the updated branch to your fork.

```bash
git push origin main
```

---

## Creating a Feature Branch

Never develop directly on `main`.

Create a new branch for every issue.

```bash
git checkout -b fix/parser-span-bug
```

Examples:

```text
fix/parser-span-bug
fix/duplicate-meta
feature/unanchored-regex
docs/contributing-guide
test/parser-edge-cases
```

---

## Before Opening a Pull Request

Every pull request should successfully complete the following commands:

```bash
cargo fmt --all -- --check

cargo clippy --all-targets --all-features -- -D warnings

cargo test --all-features --locked

cargo build --all-features --locked

cargo llvm-cov \
    --fail-under-lines 95 \
    --fail-under-functions 95
```

> Note: these commands can be run with `just all` from the Justfile

If all commands complete successfully, your pull request should also pass GitHub Actions.

---

## Troubleshooting

### Cargo.lock has changed

If your change does not intentionally modify dependencies, avoid committing changes to `Cargo.lock`.

---

### Clippy reports warnings

CI treats Clippy warnings as errors. Resolve every warning before submitting your pull request.

---

### Coverage fails

If the coverage threshold is not met, add or improve tests for the affected code instead of lowering the threshold. Lowering the threshold will result in your PR being denied.

---

### Need Help?

If you're unsure how to implement a change, open a draft pull request or start a discussion on the associated issue before investing significant development time. We'd much rather answer questions early than ask you to rework a large contribution later.
