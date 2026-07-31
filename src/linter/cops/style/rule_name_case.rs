//! Check if a rule's name is in the permitted case format

use crate::linter::cops::style::{is_pascal_case, is_snake_case};
use crate::linter::{
    context::LintContext,
    cop::{Category, Cop},
    finding::{Finding, Severity},
};

/// The naming convention a `StyleRuleNameCase` cop enforces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameCase {
    /// Represents PascalCase
    PascalCase,
    /// Represents snake_case
    SnakeCase,
}

/// Enforces a single, configured casing convention across all rule
/// names in a file.
pub struct StyleRuleNameCase {
    /// The rule case
    case: NameCase,
}

impl StyleRuleNameCase {
    /// Create a cop enforcing `case` on all rule names.
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
                NameCase::PascalCase => is_pascal_case(strip_version_suffix(&rule.name)),
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

/// Strips off version suffixes from a string
///
/// Since version suffixes are permitted by the YARlint for PascalCase names,
/// this method strips them off before being passed to the string checker.
fn strip_version_suffix(s: &str) -> &str {
    if let Some(pos) = s.rfind("_v") {
        let suffix = &s[pos + 2..];
        if !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit()) {
            return &s[..pos];
        }
    }
    s
}
