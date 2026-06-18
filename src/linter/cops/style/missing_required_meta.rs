//! Enforces the presence of expected meta keys.

use crate::linter::{
    context::LintContext,
    finding::{Finding, Severity},
    rule::Rule,
};

/// Enforces the presence of expected meta keys.
pub struct StyleMissingRequiredMeta;

/// The required meta keys. This will be phased out when
/// https://github.com/DeTraced-Security/YARlint/issues/74 is implemented.
const REQUIRED_META_KEYS: [&str; 4] = ["author", "description", "reference", "date"];

impl Rule for StyleMissingRequiredMeta {
    /// Returns the name of the rule
    ///
    /// # Returns
    ///
    /// Returns "Style/MissingRequiredMeta"
    fn name(&self) -> &'static str {
        "Style/MissingRequiredMeta"
    }

    /// Checks for compliance with the rule
    ///
    /// This rule is violated if any rules are missing required meta values
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
