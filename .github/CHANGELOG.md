# CHANGELOG

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