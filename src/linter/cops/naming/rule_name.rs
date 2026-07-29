//! Rule name validation.

use crate::linter::{
    context::LintContext,
    cop::{Category, Cop},
    finding::{Finding, Severity},
};

/// Validates rule naming conventions.
pub struct NamingRuleName;

impl Cop for NamingRuleName {
    /// Returns the category of the cop
    fn category(&self) -> Category {
        Category::Naming
    }
    /// Returns the name of the cop
    ///
    /// # Returns
    ///
    /// Returns "RuleName"
    fn name(&self) -> &'static str {
        "RuleName"
    }

    /// Checks for compliance with the cop
    ///
    /// This rule is violated if a `RuleNode`'s name includes any '-'s
    ///
    /// # Arguments
    ///
    /// * `context` (`&LintContext`) - A `LintContext` containing the
    ///   `RuleNode`s to be checked
    /// * `findings` (`&mut Vec<Finding>`) - Vector to push Finding to
    fn check(&self, context: &LintContext, findings: &mut Vec<Finding>) {
        for rule in &context.file.rules {
            if rule.name.contains('-') {
                findings.push(Finding {
                    rule: self.name(),
                    category: self.category(),
                    message: format!("Rule '{}' contains '-'", rule.name),
                    severity: Severity::Warning,
                });
            }
        }
    }
}
