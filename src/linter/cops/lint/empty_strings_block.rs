//! Validates the string block in a YARA rule isn't empty
use crate::linter::{
    context::LintContext,
    finding::{Finding, Severity},
    rule::Rule,
};
/// Validates the string block in a YARA rule isn't empty
pub struct LintEmptyStringsBlock;

impl Rule for LintEmptyStringsBlock {
    fn name(&self) -> &'static str {
        "Lint/EmptyStringsBlock"
    }

    fn check(&self, context: &LintContext, findings: &mut Vec<Finding>) {
        for rule in &context.file.rules {
            if rule.strings.is_empty() {
                findings.push(Finding {
                    rule: self.name(),
                    message: format!(
                        "The strings block in rule {} is empty, consider removing it",
                        rule.name
                    ),
                    severity: Severity::Warning,
                })
            }
        }
    }
}
