# Contributing to YARlint

First of all, thanks for contributing!

## How to Contribute (High Level)

1. **Identify a problem or improvement**
2. **Open or claim an issue**
3. **Fork the repo**
4. **Create a branch**
5. **Make your changes**
6. **Open a pull request**

If you do not open or claim an issue, your pull request will likely be rejected.

---

## Issues Come First

Before you open a PR, **there should be an issue**.

This applies if you are:
- Fixing a bug
- Adding a feature
- Refactoring existing behavior
- Changing styling, layout, or structure in a non-trivial way

### Why?
- It prevents duplicate work
- It lets us agree on direction before code is written
- It gives context to your PR
- It creates a paper trail for future contributors

### What to do:
- If an issue already exists comment on it and say you’re working on it
- If no issue exists open one
- If you’re unsure open an issue anyway and ask

> Small things like typo fixes can skip the issue step. Use your judgment.

### Exceptions

In rare cases, a pull request may not require a linked issue (for example,
automated release PRs or repository maintenance).

Maintainers may apply the `no-linked-issue-required` label to bypass the linked issue check. Contributors should not request or apply this label unless directed by a maintainer. Adding the label to a PR without authorization are grounds for the denial of merging, and further consequences as seen fit by maintainers.

---

## Fork & Branch

1. Fork the repository
2. Clone your fork locally
3. Create a branch from `main`

```bash
git checkout -b fix/my-short-description
```

Please use clear branch names. Since your pull request will be squashed upon 
merge, make sure the branch, and pull request title follow the conventional
commits standard.

As for commit messages themselves, we follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
standards. Each commit message should begin with a type, include an optional
scope for the changes committed, and conclude with a brief, but precise,
description of the changes. For example:

```
docs(linter): Added missing documentation
or
docs: Added missing documentation
```

Acceptable commit types include:
- docs - documentation
- fix - patch/bugfix
- feat - feature
- chore - simple maintenance
- test - testing
- style - style changes

Imprecise commit messages may result in increased difficulty of PR reviews and
will result in changes being requested and cause a PR to not pass CI/CD.

## Making Changes

A few expectations:

- Follow the existing code style
- Don’t reformat unrelated files
- Keep commits focused and meaningful
- If you change behavior, explain why in the commit or PR

## Testing

YARlint's tests live in a top-level `tests/` directory that mirrors the
structure of `src/`, rather than inline `#[cfg(test)] mod tests` blocks
inside individual source files. There's one integration test crate per
top-level `src/` module:

```
tests/
├── app.rs          # mirrors src/app.rs
├── cli.rs          # mirrors src/cli.rs
├── filesystem.rs   # mirrors src/filesystem.rs
│   └── filesystem/
├── linter.rs       # mirrors src/linter.rs
│   └── linter/
│       └── cops/
│           ├── lint/
│           ├── naming/
│           ├── performance/
│           └── style/
├── parser.rs       # mirrors src/parser.rs
│   └── parser/
├── validation.rs   # mirrors src/validation.rs
│   └── validation/
└── fixtures/       # real .yar files used across multiple test suites
```

Each top-level file (`tests/linter.rs`, `tests/parser.rs`, etc.) is picked
up automatically by Cargo as its own integration test binary — there's no
`[[test]]` section in `Cargo.toml` declaring these explicitly, since
Cargo's default test discovery already covers anything directly under
`tests/`.

### Adding a submodule

Submodules under each top-level file are declared with an explicit `#[path]`
attribute, not a bare `mod foo;`, since the file lives in a directory that
mirrors `src/`'s layout rather than sitting next to the file that declares
it:

```rust
#[path = "linter/cops/style/rule_name_case.rs"]
mod rule_name_case;
```

Follow the existing pattern in the relevant top-level file (e.g.
`tests/linter.rs`) when adding a test module for new code — don't introduce
`mod.rs` files or a different declaration style for just one submodule.

### Black-box testing

Tests import the crate the same way an external consumer would:

```rust
use yarlint::linter::{
    context::LintContext,
    cop::Category,
    cops::style::rule_name_case::{NameCase, StyleRuleNameCase},
    engine::LintEngine,
    finding::Severity,
};
```

This means tests can only exercise `pub` items.

### Testing non-public items

If what you need to test is `pub(crate)` or private, and there's no
existing `pub` entry point that exercises it, the external `tests/` tree
can't reach it — that's the one case where an inline `#[cfg(test)] mod
tests` block inside the source file itself is the right tool, rather than
loosening visibility just to make the black-box tree work.

Don't change a production item's visibility solely to make it testable
externally. If the item is genuinely part of the type's public contract,
making it `pub` is fine on its own merits — but if it's private
implementation detail, keep it private and test it in place instead.

### What to cover

For a new cop specifically, tests should cover:

- A rule that clearly violates the cop's condition produces the expected
  `Finding` (correct `rule`, `category`, `severity`, and a `message` that
  names the actual offending value, not just a generic description).
- A rule that clearly satisfies the condition produces no findings.
- Any boundary case specific to the cop's own logic — e.g. version-suffix
  handling, empty inputs, or whatever edge case the cop's own
  implementation singles out for special handling.

For parser or lexer changes, prefer a fixture file under `tests/fixtures/`
over an inline string literal when the input is realistic YARA source —
it keeps test files readable and lets the same fixture be reused by other
suites (including the benchmark workflow, which lints against
`tests/fixtures/` directly).

### Running tests

```
cargo test --all-features --locked
```

This runs every integration test crate under `tests/`, matching what CI
runs in the `test` job. `--locked` ensures you're testing against the
exact dependency versions in `Cargo.lock`, not whatever your local cache
happens to have resolved.


## Opening a Pull Request

When opening your PR:

- Reference the issue it fixes (e.g. Fixes #12)
- Explain what you changed
- Explain why you changed it
- Include screenshots if it’s visual
- Keep it scoped — one PR, one purpose

Please fill out the PR template when opening a pull request. That helps us with understanding what your PR is trying to do. PRs without context or without an associated issue may be closed or sent back for clarification.

For more information on how we handle PRs and PR reviews, check out our 
[release expectations](/docs/development/review-expectations) guide!


---

Thanks again for contributing 💜

---