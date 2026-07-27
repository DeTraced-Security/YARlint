//! Verifies no two strings are duplicates of each other
//!
//! This is both type sensitive ("foo" and "{66 6f 6f}" are different) and
//! modifier sensitive (`"foo" wide ascii` is different than `"foo" wide` and
//! `"foo" ascii`)

use std::collections::HashMap;

use crate::{
    linter::{
        context::LintContext,
        cop::Cop,
        finding::{Finding, Severity},
    },
    parser::syntax::{StringModifier, strings::StringType},
};

/// Verifies no two strings are duplicates of each other
pub struct LintDuplicateString;

impl Cop for LintDuplicateString {
    /// Returns the name of the cop
    ///
    /// # Returns
    ///
    /// Returns "Lint/DuplicateString"
    fn name(&self) -> &'static str {
        "Lint/DuplicateString"
    }

    /// Checks for compliance with the cop
    ///
    /// This cop check fails if a string is defined more than once in a rule
    ///
    /// # Arguments
    ///
    /// * `context` (`&LintContext`) - A `LintContext` containing the
    ///   `RuleNode`s to be checked
    /// * `findings` (`&mut Vec<Finding>`) - Vector to push Finding to
    fn check(&self, context: &LintContext, findings: &mut Vec<Finding>) {
        for rule in &context.file.rules {
            let mut string_map: HashMap<(StringType, Vec<StringModifier>), &str> = HashMap::new();

            for string in &rule.strings {
                let mut sorted_modifiers = string.modifiers.clone();
                sorted_modifiers.sort();

                let key = (string.value.clone(), sorted_modifiers);

                if let Some(existing_identifier) = string_map.get(&key) {
                    findings.push(Finding {
                        rule: self.name(),
                        message: format!(
                            "String {} in rule {} is a duplicate of {}",
                            string.identifier, rule.name, existing_identifier
                        ),
                        severity: Severity::Warning,
                    });
                } else {
                    string_map.insert(key, &string.identifier);
                }
            }
        }
    }
}
