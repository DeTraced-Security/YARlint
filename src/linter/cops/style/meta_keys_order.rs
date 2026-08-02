//! Enforces the order of expected meta keys.

use crate::linter::{
    context::LintContext,
    cop::{Category, Cop},
    finding::{Finding, Severity},
};

/// Enforces the order of expected meta keys.
pub struct StyleMetaKeysOrder;

/// The order of meta keys. This will be phased out when
/// <https://github.com/DeTraced-Security/YARlint/issues/74> is implemented.
const META_KEYS_ORDER: [&str; 4] = ["author", "description", "reference", "date"];

impl Cop for StyleMetaKeysOrder {
    /// Returns the category of the cop
    fn category(&self) -> Category {
        Category::Style
    }

    /// Returns the name of the cop
    ///
    /// # Returns
    ///
    /// Returns "MetaKeysOrder"
    fn name(&self) -> &'static str {
        "MetaKeysOrder"
    }

    /// Checks for compliance with the cop
    ///
    /// This cop is violated if any rules have meta values that are out of
    /// the specified order.
    ///
    /// # Arguments
    ///
    /// * `context` (`&LintContext`) - A `LintContext` containing the
    ///   `RuleNode`s to be checked
    /// * `findings` (`&mut Vec<Finding>`) - Vector to push Finding to
    fn check(&self, context: &LintContext, findings: &mut Vec<Finding>) {
        for rule in &context.file.rules {
            // (key, expected_index)
            let mut last_seen: Option<(&str, usize)> = None;

            for entry in &rule.meta {
                let Some(expected_index) = META_KEYS_ORDER.iter().position(|&k| k == entry.key)
                else {
                    continue;
                };

                if let Some((last_key, last_index)) = last_seen
                    && expected_index < last_index
                {
                    findings.push(Finding {
                        rule: self.name(),
                        category: self.category(),
                        message: format!(
                            "meta keys for rule {} are out of order: found '{}' after '{}', expected '{}' before '{}'",
                            rule.name, entry.key, last_key, entry.key, last_key
                        ),
                        severity: Severity::Info,
                    });
                }

                last_seen = Some((entry.key.as_str(), expected_index));
            }
        }
    }
}
