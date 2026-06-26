//! Verifies that a String is not empty

use crate::{
    linter::{
        context::LintContext,
        finding::{Finding, Severity},
        rule::Rule,
    },
    parser::syntax::strings::StringType,
};

/// Verifies that a String is not empty
pub struct LintEmptyString;

impl Rule for LintEmptyString {
    /// Returns the name of the rule
    ///
    /// # Returns
    ///
    /// Returns "Lint/EmptyString"
    fn name(&self) -> &'static str {
        "Lint/EmptyString"
    }

    /// Checks for compliance with the rule
    ///
    /// This rule is violated if a `StringNode` in the context being checked is
    /// empty.
    ///
    /// # Arguments
    ///
    /// * `context` (`&LintContext`) - A `LintContext` containing the
    ///   `RuleNode`s to be checked
    /// * `findings` (`&mut Vec<Finding>`) - Vector to push Finding to
    fn check(&self, context: &LintContext, findings: &mut Vec<Finding>) {
        for rule in &context.file.rules {
            for string in &rule.strings {
                match &string.value {
                    StringType::Text(text) => {
                        if text.is_empty() {
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

                    StringType::Hex(hex_node) => {
                        if hex_node.original_string.is_empty() {
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

                    StringType::RegEx(regex_string) => {
                        if regex_string.is_empty() {
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
    }
}
