//! Enforces the presence of expected meta keys.

use crate::linter::{
    context::LintContext,
    cop::{Category, Cop},
    finding::{Finding, Severity},
};

/// Enforces the presence of expected meta keys.
pub struct StyleMissingRequiredMeta;

/// The required meta keys. This will be phased out when
/// https://github.com/DeTraced-Security/YARlint/issues/74 is implemented.
const REQUIRED_META_KEYS: [&str; 4] = ["author", "description", "reference", "date"];

impl Cop for StyleMissingRequiredMeta {
    /// Returns the category of the cop
    fn category(&self) -> Category {
        Category::Style
    }

    /// Returns the name of the cop
    ///
    /// # Returns
    ///
    /// Returns "MissingRequiredMeta"
    fn name(&self) -> &'static str {
        "MissingRequiredMeta"
    }

    /// Checks for compliance with the cop
    ///
    /// This cop is violated if any rules are missing required meta values
    ///
    /// # Arguments
    ///
    /// * `context` (`&LintContext`) - A `LintContext` containing the
    ///   `RuleNode`s to be checked
    /// * `findings` (`&mut Vec<Finding>`) - Vector to push Finding to
    fn check(&self, context: &LintContext, findings: &mut Vec<Finding>) {
        for rule in &context.file.rules {
            for &required_key in REQUIRED_META_KEYS.iter() {
                let present = rule.meta.iter().any(|entry| entry.key == required_key);
                if !present {
                    findings.push(Finding {
                        rule: self.name(),
                        category: self.category(),
                        message: format!(
                            "Rule {} is missing the meta key '{}'",
                            rule.name, required_key
                        ),
                        severity: Severity::Info,
                    })
                }
            }
        }
    }
}
