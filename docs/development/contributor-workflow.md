# Contributor Workflow Guide

This doc walks through how an idea becomes a merged, released change in YARlint.

## The pipeline

```
Issue → Maintainer Approval → Contributor Claims → Implementation → Pull Request → Review → Merge → Release
```

## 1. Issue

Everything starts as a GitHub issue. Bug reports, feature requests, and performance regressions each have a template. Use the one that fits. Be specific: for a new cop, that means the pattern it detects, false-positive risks, and ideally a couple of example rules (one that should trigger, one that shouldn't). Vague issues sit longer because a maintainer has to go back and forth to even scope them.

## 2. Maintainer Approval

A maintainer triages the issue before anyone starts writing code. This is the "is this actually a good idea, and does it fit the project" gate. For cops specifically, this is where category placement (Lint/Logic/Naming/Performance/Style) and severity get sanity-checked against existing conventions, since retrofitting that later is way more annoying than agreeing on it up front. An approved issue gets labeled accordingly. Don't start implementation before this happens, since unapproved work risks getting rejected outright regardless of code quality.

## 3. Contributor Claims

Once approved, comment on the issue to claim it. This avoids two people duplicating work. If an issue's been claimed but gone quiet for a while, it's fair game to ask the maintainers if it's still being worked or if you can pick it up.

## 4. Implementation

Build it against the conventions documented in the repo. Review is checked against these directly:

- No `mod.rs`; sibling-file pattern (`folder.rs` next to `folder/`)
- SHA-pin any third-party GitHub Action, never float a tag
- One concern per workflow file
- Tests live in the external `tests/` tree mirroring source structure; inline `#[cfg(test)]` only when private visibility forces it
- Workflow files are path-filtered to what they actually care about, including themselves

If you're implementing a cop, check the writing-a-cop guide and the contributor testing guide before you start. They cover the expected shape of a cop implementation and what test coverage looks like. Commits should be logically scoped so review isn't reading one giant diff blind.

## 5. Pull Request

Open the PR against the issue. Reference the issue number so they're linked. Fill out the PR template with what changed and why. `pr-size-labeler` will tag the PR by diff size automatically.

## 6. Review

This is checked against the review expectations guide, but broadly:

- CI has to be green: `fmt`, `clippy`, `build`, `test`, `coverage` all run in parallel, plus `cargo-deny` on the PR check
- MSRV verification runs against whatever's declared in `Cargo.toml`
- A maintainer reviews for correctness, convention adherence, and test quality
- Expect iteration. Review comments are part of the normal path to merge.

## 7. Merge

Once approved and CI's green, a maintainer merges. Semantic versioning applies from here. Since we're pre-1.0.0, any new feature (like a new cop) bumps the minor version, per `features_always_increment_minor`.

## 8. Release

Releases are handled by `cargo-dist` + `release-plz`. Merged changes accumulate until a release is cut, which produces cross-platform binaries (including macOS via `zigbuild`) named `yarlint-{target-triple}[.exe]`. You don't need to do anything here as a contributor. This is just so you know: a merged PR ships on the next cut release.

---

One meta note: scope deferral (config file support, autofix, etc.) is a deliberate choice. If your issue or PR touches one of those deferred areas, expect a quick "not yet" from a maintainer. These are settled decisions.