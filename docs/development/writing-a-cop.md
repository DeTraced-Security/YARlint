# Writing a Cop

This guide walks through adding a new lint cop to YARlint, using the real `Cop` 
trait and an existing cop as a reference.

## 1. Pick a category

Every cop belongs to exactly one `Category`:

- `Lint`
  - Likely bugs or dead code (e.g. duplicate strings, empty blocks)
- `Logic`
  - Conditions that are always true/false or otherwise suspicious
- `Naming`
  - Naming conventions for rules, meta keys, etc.
- `Performance`
  - Patterns that make YARA scanning slower than necessary
- `Style`
  - Formatting and stylistic conventions

The category you pick determines the file's location: a cop with 
`category() == Category::Style` lives under `src/linter/cops/style/`, and so on.
 This is a convention reviewers will check by hand (see the review expectations guide).

## 2. Implement the `Cop` trait

The trait itself, from `src/linter/cop.rs`:

```rust
pub trait Cop {
    fn name(&self) -> &'static str;
    fn category(&self) -> Category;
    fn qualified_name(&self) -> String {
        format!("{}/{}", self.category(), self.name())
    }
    fn check(&self, context: &LintContext, findings: &mut Vec<Finding>);
}
```

`qualified_name()` has a default implementation, please don't override it. It's 
what `--only`/`--except` match against (e.g. `Style/RuleNameCase`).

### Construction pattern

If your cop needs configuration, take it as a constructor parameter, not by 
reading config inside `check()`. This is a unit-struct-plus-`new()` pattern 
already used throughout the cop library:

```rust
pub struct StyleRuleNameCase {
    case: NameCase,
}

impl StyleRuleNameCase {
    pub fn new(case: NameCase) -> Self {
        Self { case }
    }
}
```

If your cop has no configuration, it can be a plain unit struct with no
`new()` at all that is registered directly by name (see step 4).

### A full example

`src/linter/cops/style/rule_name_case.rs`, trimmed:

```rust
use crate::linter::cops::style::{is_pascal_case, is_snake_case};
use crate::linter::{
    context::LintContext,
    cop::{Category, Cop},
    finding::{Finding, Severity},
};

pub struct StyleRuleNameCase {
    case: NameCase,
}

impl StyleRuleNameCase {
    pub fn new(case: NameCase) -> Self {
        Self { case }
    }
}

impl Cop for StyleRuleNameCase {
    fn name(&self) -> &'static str {
        "RuleNameCase"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn check(&self, context: &LintContext, findings: &mut Vec<Finding>) {
        for rule in &context.file.rules {
            let conforms = match self.case {
                NameCase::PascalCase => is_pascal_case(&rule.name),
                NameCase::SnakeCase => is_snake_case(&rule.name),
            };

            if !conforms {
                findings.push(Finding {
                    rule: self.name(),
                    category: self.category(),
                    message: format!(
                        "Rule name '{}' does not conform to the configured {:?} naming convention",
                        rule.name, self.case
                    ),
                    severity: Severity::Warning,
                });
            }
        }
    }
}
```

## 3. Push a `Finding` for every violation

`Finding`, from `src/linter/finding.rs`:

```rust
pub struct Finding {
    pub rule: &'static str,      // self.name()
    pub category: Category,      // self.category()
    pub message: String,         // human-readable, specific to the violation
    pub severity: Severity,      // Info / Warning / Error
}
```

`rule` and `category` should always come from `self.name()` and
`self.category()`. Do not hardcode them separately, or they can drift out of
sync with the trait methods above.

`message` should name the specific thing that's wrong (the actual rule name,
the actual value found, etc.), and not just restate the cop's name. The
`StyleRuleNameCase` example above includes the offending rule name and the
expected case for exactly this reason.

## 4. Register the cop

New cops are wired up in `src/linter.rs`, in the function that builds the
default `LintEngine`:

```rust
engine.register(StyleRuleNameCase::new(config::rule_name_case()));
engine.register(StyleMetaKeysOrder);  // no config needed, plain unit struct
```

A cop that isn't registered here will never run, regardless of whether it's
correctly implemented. Please4 remember this, as it is an easy step to forget.

## 5. Write tests

Tests for cops currently live in a separate `tests/` tree that mirrors
`src/linter/`'s structure via `#[path = "..."]` module declarations (see
`tests/linter.rs`), rather than inline `#[cfg(test)] mod tests` blocks inside
the cop's own source file. Tests exercise the cop as an external consumer of
the crate (`use yarlint::linter::...`), the same way a real caller would.

At minimum, a new cop's tests should cover:

- A rule that clearly violates the cop's condition produces exactly the
  `Finding` you expect (right `rule`, `category`, `severity`, and a
  `message` that mentions the actual offending value).
- A rule that clearly satisfies the condition produces no findings.
- Any boundary/edge case specific to the cop's logic (e.g.
  `StyleRuleNameCase`'s version-suffix stripping needs a test for names like
  `SomeRule_v2`, not just a plain PascalCase/snake_case pair).

## 6. Confirm CI passes

Once the cop is implemented, registered, and tested, the usual PR checks
apply. See [Review Expectations](./review-expectations.md) for what's
automated versus what a human reviewer will look for specifically on a new
cop (family placement, construction pattern, test quality beyond the raw
coverage number).