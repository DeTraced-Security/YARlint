//! Verifies that a String is not empty

use crate::linter::{
    context::LintContext,
    finding::{Finding, Severity},
    rule::Rule,
};

/// Verifies that a String is not empty
pub struct LintEmptyString;

impl Rule for LintEmptyString {
    fn name(&self) -> &'static str {
        "Lint/EmptyString"
    }

    fn check(&self, context: &LintContext, findings: &mut Vec<Finding>) {
        for rule in &context.file.rules {
            for string in &rule.strings {
                if string.value.is_empty() {
                    findings.push(Finding {
                        rule: self.name(),
                        message: format!(
                            "String '{}' in rule '{}' is empty, consider removing it",
                            string.identifier, rule.name
                        ),
                        severity: Severity::Warning,
                    });
                }
            }
        }
    }
}
