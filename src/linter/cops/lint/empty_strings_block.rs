//! Validates the string block in a YARA rule isn't empty
use crate::linter::{
    context::LintContext,
    cop::{Category, Cop},
    finding::{Finding, Severity},
};
/// Validates the string block in a YARA rule isn't empty
pub struct LintEmptyStringsBlock;

impl Cop for LintEmptyStringsBlock {
    /// Returns the category of the cop
    fn category(&self) -> Category {
        Category::Lint
    }
    /// Returns the name of the cop
    ///
    /// # Returns
    ///u
    /// Returns "EmptyStringsBlock"
    fn name(&self) -> &'static str {
        "EmptyStringsBlock"
    }

    /// Checks for compliance with the cop
    ///
    /// This cop is violated if any `RuleNode`s have an empty strings block.
    ///
    /// # Arguments
    ///
    /// * `context` (`&LintContext`) - A `LintContext` containing the
    ///   `RuleNode`s to be checked
    /// * `findings` (`&mut Vec<Finding>`) - Vector to push Finding to
    fn check(&self, context: &LintContext, findings: &mut Vec<Finding>) {
        for rule in &context.file.rules {
            if rule.strings.is_empty() {
                findings.push(Finding {
                    rule: self.name(),
                    category: self.category(),
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
