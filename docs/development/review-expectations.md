# Review Expectations

This document explains what happens to a pull request after you open it: what's checked automatically, what a maintainer looks for by hand, and how a PR gets merged.

## CI Checks
In order for a PR to be merged, it must pass ***ALL*** ci/cd checks. Human review happens regardless, and you can update your PR to pass the checks after initially opening it, but it is a requirement in order to be merged.

The current CI checks for pull requests are:
# CI Checks That Run on Pull Requests

- **Lint GitHub Actions workflows**
  - Statically lints the workflow YAML files themselves for syntax errors and common mistakes.
  - Downloads and runs `actionlint` via its official install script.
  - Command: `${{ steps.get_actionlint.outputs.executable }} -color`

- **Cargo Deny**
  - Enforces the license allowlist, checks for yanked/vulnerable dependencies, and restricts dependency sources.
  - Filtered to PRs touching `Cargo.toml`, `Cargo.lock`, `deny.toml`, or the workflow file itself.
  - Runs the pinned `cargo-deny-action` (SHA-pinned to v2.1.1).
  - Command: `cargo deny check` (via the action's `command: check` input)

- **MSRV Audit**
  - Verifies the crate builds and passes tests on the minimum Rust version declared in `Cargo.toml`.
  - Filtered to PRs touching `Cargo.toml`, `Cargo.lock`, `src/**`, or the workflow file itself.
  - Extracts the declared MSRV directly from `Cargo.toml` rather than hardcoding it.
  - Command: `MSRV=$(grep -m1 '^rust-version' Cargo.toml | sed -E 's/.*"([^"]+)".*/\1/')`, then installs that exact toolchain and runs `cargo build --all-features --locked` and `cargo test --all-features --locked` against it.

- **Rust CI**
  - The core correctness/quality gate, split into independent parallel jobs.
  - **fmt**: `cargo fmt --all -- --check`
  - **clippy**: `cargo clippy --all-targets --all-features -- -D warnings`
  - **build**: `cargo build --all-features --locked`
  - **test**: `cargo test --all-features --locked` (depends on `build` passing first)
  - **coverage**: `cargo llvm-cov --fail-under-lines 95 --fail-under-functions 95`
  - **docs**: `cargo doc --no-deps --document-private-items` with `RUSTDOCFLAGS: "-D warnings"`, so broken intra-doc links or missing docs fail the build.

- **Cross Platform**
  - Confirms the test suite passes on all three shipped target platforms.
  - Matrix across `ubuntu-latest`, `windows-latest`, `macos-latest`.
  - Command: `cargo test --all-features` on each OS.

- **CodeQL**
  - Static security/quality analysis of the Rust source.
  - Runs GitHub's CodeQL init → autobuild → analyze pipeline for the `rust` language.

- **Lint Commit Messages**
  - Enforces conventional-commit-style formatting on commit messages.
  - Runs `wagoid/commitlint-github-action` (SHA-pinned) against the commits in the PR.

- **Typos**
  - Catches spelling mistakes across the repo.
  - Runs `crate-ci/typos` against the full checkout.

- **Benchmark** 
  - Measures whether the PR makes linting meaningfully slower, by building and timing both the PR's base and head commits against the same fixed input.
  - Runs with a read-only token: never touches secrets, safe to build/execute PR code from forks.
  - Checks out both `base` and `head` refs, builds each in release mode.
  - Command: `hyperfine --warmup 3 --export-markdown bench-result.md` comparing `./base/target/release/yarlint -p base/tests/fixtures -r` against `./pr/target/release/yarlint -p base/tests/fixtures -r`.
  - Uploads the result as an artifact for **Benchmark Comment** (a separate `workflow_run`-triggered workflow, not itself a PR trigger) to post.

- **Check Linked Issues**
  - Requires the PR to reference an approved GitHub issue.
  - `pull_request_target`, fires on open/edit/reopen/synchronize.
  - Posts an automatic reminder comment if no linked issue is found
  - Bypassable via `no-linked-issue-required` label (only maintainers should apply this label)
  - Excludes `release-plz-**`/`dependabot/**` branches.

- **PR Size Labeler**
  - Labels the PR by diff size.
  - `pull_request_target` (needed for fork PRs to get a write-capable token for label mutation).
  - Runs `cbrgm/pr-size-labeler-action`.

- **Labeler**
  - Labels the PR based on which file paths were touched, per `.github/labeler.yml`.
  - `pull_request_target`.
  - Runs `actions/labeler`.

If a check fails, fix it before requesting review. A clean PR lets us know that you're ready for a review.

## What a human reviewer looks for

This is the part CI can't check, and where most review time actually goes:

- Architectural fit
    - Does a new cop belong in the family it claims (Lint / Logic / Naming / Performance / Style)? Does it follow the established construction pattern (parameters passed at construction, e.g. StyleRuleNameCase::new(case), not read from config at check() time)?
- Test quality
  - 95% line coverage doesn't guarantee the right edge cases are covered. A reviewer will check whether tests actually exercise the tricky paths (empty inputs, boundary conditions, malformed input a real user might hit), not just whatever lines the happy path touches.
- Public API surface
  - Should this actually be pub? Does it need a doc comment? Does it match existing naming conventions?
- Scope
  - One PR, one concern. A PR that bundles an unrelated refactor alongside its stated change will likely be asked to split.
- Respecting prior deferrals
  - This project deliberately defers complexity (e.g. the config file system, full autofix, certain regex edge cases) as explicit decisions, not oversights. A reviewer won't relitigate "why doesn't this handle X yet" if X was already consciously deferred elsewhere. If you think a deferral should be revisited, open an issue to discuss it rather than reopening the debate inside an unrelated PR.

## Review Timeline
- Initial feedback will come from a reviewer when your PR has all CI checks pass
- After any changes are made and the first reviewer is satisfied, they will request a second review.
- Once the second reviewer is satisfied, the PR will be squashed and merged.
- Larger PRs (new cop families, parser changes, anything touching the autofix engine once it exists) may take longer given the review depth above. Please expect more back-and-forth, and not a slower first response.


## Merge Process
- PRs are merged via squashes. You PR title becomes the squash commit message, so keep it accurate and following the same convention `commitlint` enforces on individual commits.
- Once merged, `release-plz` will pick up the change on its next run against main and include it in the next version bump / changelog entry automatically. Don't update the version yourself.

## Questions
If anything here is unclear, open a discussion or ask directly on your PR. This document will get revised as the project's process evolves.