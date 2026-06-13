# CHANGELOG

## [0.1.2](https://github.com/DeTraced-Security/YARlint/compare/v0.1.1...v0.1.2) - 2026-06-13

### Bug Fixes

- default run bugs ([#56](https://github.com/DeTraced-Security/YARlint/pull/56))

## [0.1.1](https://github.com/DeTraced-Security/YARlint/compare/v0.1.0...v0.1.1) - 2026-06-12

### Bug Fixes

- *(ci)* regex bug in release plz config ([#33](https://github.com/DeTraced-Security/YARlint/pull/33))

### Documentation

- add readme ([#30](https://github.com/DeTraced-Security/YARlint/pull/30))

### Other

- add commitlint and release-plz configs ([#32](https://github.com/DeTraced-Security/YARlint/pull/32))
- *(workflows)* fix release-plz workflow ([#29](https://github.com/DeTraced-Security/YARlint/pull/29))
- add releases-plz workflow ([#26](https://github.com/DeTraced-Security/YARlint/pull/26))

### Refactored

- *(linter)* cops file structure ([#27](https://github.com/DeTraced-Security/YARlint/pull/27))

## [0.1.0] - 2026-06-10

### 🚀 Features

- Commitlint workflow
- Add initial framework
- Create dependabot.yml
- Add lexer
- New actions
- *(lexer)* Add column and row tracking
- *(ast)* Add yara syntax nodes and minor lints
- *(ast)* Add meta and strings parsing
- *(ast)* Condition parser can parse test rule
- *(lexer)* Add comment parsing in rewrite
- *(ast)* Nearly finish ast parsing, add rulefile nodes
- *(ast)* Fix module function parsing
- Add linting engine (#21)

### 🐛 Bug Fixes

- Security issues with commitlint
- Deny workflow by removing it lol
- Deny.toml workflow
- Gh pages build path
- Change license to reflect DeTraced core values
- Documentation site build (#9)
- Added private items tag to documentation build

### 🚜 Refactor

- Pass workflows
- *(lexer)* Move tokens to own file
- *(lexer)* Linting changes
- *(parser)* Refactor to pass ci tests
- Fix doc type and remove debugging output

### 📚 Documentation

- Add documentation to currently created code
- *(lexer)* Oops, forgor the documentation we already had
- *(lexer)* Add extended documentation to token and span
- *(lexer)* Document  the lexer struct

### 🎨 Styling

- Add ast parser blank files

### 🧪 Testing

- Add new github workflows
- Create codeql.yml

### ⚙️ Miscellaneous Tasks

- De-template the initial commit