# Security Policy

## Supported Versions


| Version | Supported |
|---------|-----------|
| `main` (pre-release) | ✅ Yes |
| Any tagged release | ✅ Yes (latest only) |
| Older tagged releases | ❌ No |

We strongly recommend always running the latest commit on `main` or the most recent tagged release until a long-term support policy is established.

---

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues, pull requests, or discussions.**

### Private Disclosure

Report vulnerabilities privately via GitHub's built-in security advisory system:

1. Navigate to the [YARlint repository](https://github.com/DeTraced-Security/YARlint)
2. Click **Security** → **Advisories** → **Report a vulnerability**
3. Fill in the details described below

If you are unable to use GitHub's advisory system, contact the maintainers directly via the email address listed in the repository's `CODEOWNERS` file.

### What to Include

A useful report includes as much of the following as possible:

- A clear description of the vulnerability and its potential impact
- The affected component
- Steps to reproduce, including any proof-of-concept code or packet captures
- The version or commit hash where the issue was observed
- Any suggested mitigations or fixes

The more detail you provide, the faster we can triage and respond.

### Response Timeline

| Milestone | Target |
|-----------|--------|
| Acknowledgement of report | Within 48 hours |
| Initial triage and severity assessment | Within 7 days |
| Fix developed and reviewed | Within 30 days for critical/high; 90 days for medium/low |
| Public disclosure | Coordinated with reporter; typically after fix is released |

We follow a coordinated disclosure model. We ask that reporters allow us reasonable time to develop and release a fix before publishing details publicly. We will credit reporters in the security advisory unless anonymity is requested.

---

## Scope

The following are considered in scope for security reports:

- **File Parsing** binary injection, RCE, DOS via malformed input

The following are currently out of scope:

- Vulnerabilities in third-party dependencies -> report these upstream
- Issues requiring physical access
- Social engineering of maintainers
- Vulnerabilities in GitHub Actions CI configuration that do not affect the project itself

---


## Acknowledgements

We are grateful to all researchers who responsibly disclose vulnerabilities in YARlint. Contributors who report valid security issues will be credited in the relevant GitHub Security Advisory unless they request otherwise.

---

*This policy is versioned alongside the YARlint codebase. For the current version, see [`SECURITY.md`](/.github/SECURITY.md) on the `main` branch.*