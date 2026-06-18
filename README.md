# YARlint

> **Early WIP / Pre-Alpha**: expect breaking changes, missing features, and rough edges.

A YARA rule linter written in Rust. Catches syntax errors, style issues, logic problems, and performance pitfalls before they end up in production rulesets. Inspired by [Rubocop](https://rubocop.org/)

---

## Features

- **Syntax checking** — parse errors in rule syntax
- **Style & formatting** — consistent rule structure across your team
- **Logic analysis** — flags conditions that are always true/false or otherwise suspect
- **Performance warnings** — wide regexes, missing anchors, expensive string patterns

---

## Installation

### From source

```bash
git clone https://github.com/DeTraced-Security/YARlint.git
cd YARlint
cargo build --release
```

The binary lands at `target/release/yarlint`.

### Cargo

```bash
cargo install yarlint
```

---

## Usage

### CLI

```
A modern YARA linter written in Rust

Usage: yarlint [OPTIONS] --path <PATH>

Options:
  -p, --path <PATH>    File or directory path to scan
  -r, --recursive      Recursively traverse subdirectories when scanning a directory
  -d, --depth <DEPTH>  Maximum traversal depth when recursive scanning is enabled
  -v, --verbose        Enables verbose output
  -h, --help           Print help (see more with '--help')
  -V, --version        Print version
```

**Examples:**

```bash
# Lint a single file
yarlint --path rule.yar

# Lint a directory
yarlint --path rules/

# Lint a directory recursively
yarlint --path rules/ --recursive

# Limit recursion depth
yarlint --path rules/ --recursive --depth 2
```

## Contributing

This project is in early development. If you want to contribute, please check out [CONTRIBUTING.MD](.github/CONTRIBUTING.MD)

---

## License

[GNU GPL 3.0](LICENSE) (c) DeTraced Security
