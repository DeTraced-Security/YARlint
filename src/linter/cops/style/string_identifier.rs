//! Validates that string identifiers are snake_case.

use crate::linter::{
    context::LintContext,
    cop::{Category, Cop},
    cops::style::is_snake_case,
    finding::{Finding, Severity},
};

/// Validates that string identifiers are snake_case.
pub struct StyleStringIdentifier;

impl Cop for StyleStringIdentifier {
    /// Returns the category of the cop
    fn category(&self) -> Category {
        Category::Style
    }

    /// Returns the name of the cop
    ///
    /// # Returns
    ///
    /// Returns "StringIdentifier"
    fn name(&self) -> &'static str {
        "StringIdentifier"
    }

    /// Checks for compliance with the cop
    ///
    /// This cop is violated if any rules contain `StringNode`s not in snake
    /// case
    ///
    /// # Arguments
    ///
    /// * `context` (`&LintContext`) - A `LintContext` containing the
    ///   `RuleNode`s to be checked
    /// * `findings` (`&mut Vec<Finding>`) - Vector to push Finding to
    fn check(&self, context: &LintContext, findings: &mut Vec<Finding>) {
        for rule in &context.file.rules {
            for string in &rule.strings {
                if !is_snake_case(&string.identifier) {
                    findings.push(Finding {
                        rule: self.name(),
                        category: self.category(),
                        message: format!(
                            "String identifier {} in rule {} should be snake_case",
                            string.identifier, rule.name
                        ),
                        severity: Severity::Warning,
                    })
                }
            }
        }
    }
}
