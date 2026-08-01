## YARlint Pull Request

<!-- Thanks for contributing. Fill this out so future-you doesn't hate past-you. Yes its a bit long, but hopefully this lets us catch any issues before review starts -->

<!-- 
Not finished yet?

Feel free to open this as a Draft Pull Request to receive early feedback before requesting a full review.
-->

- [ ] I have read and understand the [Contributing Guidelines](https://github.com/DeTraced-Security/YARlint/blob/main/.github/CONTRIBUTING.MD) and the [Code of Conduct](https://github.com/DeTraced-Security/YARlint/blob/main/.github/CODE_OF_CONDUCT.md)
- [ ] I have read and understand the [codebase, project infrastructure, and documentation guidelines](https://github.com/DeTraced-Security/YARlint/tree/main/docs).

## Summary

<!-- One sentence describing the purpose of this Pull Request. -->


## Motivation

<!-- What problem does this solve? Why is this approach appropriate? -->


## Link to the relevant issue

<!-- All changes must be discussed in an issue first. Failure to do so may result in your Pull Request being rejected -->
closes #<!--issue ID here-->


## Summary of Changes

<!--
Briefly list the most significant changes.

Example:

- Added parser support for ...
- Fixed incorrect span calculation.
- Added regression tests.
-->


## Type of Change

- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Refactor / Cleanup
- [ ] Docs only

## Scope

- [ ] Lexer
- [ ] AST Parser
- [ ] Cops
- [ ] CLI
- [ ] Documentation
- [ ] CI / GitHub Actions
- [ ] Filesystem
- [ ] Configuration
- [ ] Other (Please specify below)

## Testing

- [ ] Added unit tests
- [ ] Added integration tests
- [ ] Existing tests sufficiently cover this change
- [ ] Documentation only

### Manual Testing

<!--
Describe how you verified this change manually.

Examples:

- Ran against sample.yar
- Parsed MalwareBazaar rules
- Verified CLI output
- Compared diagnostics before and after

If not applicable, write "N/A".
-->


## Example Output (if applicable)

<!--
Paste any relevant output here.

Examples:

- CLI diagnostics
- Error messages
- Parser output
- Documentation screenshots

Otherwise write "N/A".
-->


## Checklist

Run these commands before requesting review.

*Local Validation Suite*
```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo build --all-features --locked
cargo test --all-features --locked
cargo llvm-cov --fail-under-lines 95 --fail-under-functions 95
```

- [ ] All commits are GPG signed
- [ ] Appropriate tests have been added or updated
- [ ] Documentation has been updated where necessary
- [ ] I ran the full local validation suite
- [ ] I verified this change locally
- [ ] No unnecessary files committed


## Breaking Changes

- [ ] This PR contains no breaking changes.
- [ ] This PR introduces breaking changes (describe below).

### Details

<!-- Otherwise write N/A -->

## Reviewer Guidance

<!-- Something you'd like reviewers to focus on -->


## Additional Notes

<!-- Anything to know/note? Edge cases, follow-ups, known issues. -->





<!-- 

Thank you for taking the time to contribute to YARlint!

The more information you provide, the faster maintainers can review your contribution. If anything in this template is unclear, please let us know—we're always looking to improve the contributor experience.

-->