//! Validates the string block in a YARA rule isn't empty
use crate::linter::{
    context::LintContext,
    finding::{Finding, Severity},
    rule::Rule,
};
/// Validates the string block in a YARA rule isn't empty
pub struct LintEmptyStringsBlock;

impl Rule for LintEmptyStringsBlock {
    /// Returns the name of the rule
    ///
    /// # Returns
    ///
    /// Returns "Lint/EmptyStringsBlock"
    fn name(&self) -> &'static str {
        "Lint/EmptyStringsBlock"
    }

    /// Checks for compliance with the rule
    ///
    /// This rule is violated if any `RuleNode`s have an empty strings block.
    ///
    /// # Arguments
    ///
    /// * `context` (`&LintContext`) - A `LintContext` containing the
    /// `RuleNode`s to be checked
    /// * `findings` (`&mut Vec<Finding>`) - Vector to push Finding to
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
