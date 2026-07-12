//! Verifies no two meta entries are duplicates of each other
//!
//! This does not check if two entries have the same value, rather if
//! two entries have the same key.

use std::collections::HashSet;

use crate::linter::{
    context::LintContext,
    finding::{Finding, Severity},
    rule::Rule,
};

/// Verifies no two meta entries are duplicates of each other
pub struct LintDuplicateMeta;

impl Rule for LintDuplicateMeta {
    /// Returns the name of the rule
    ///
    /// # Returns
    ///
    /// Returns "Lint/DuplicateMeta"
    fn name(&self) -> &'static str {
        "Lint/DuplicateMeta"
    }

    /// Checks for compliance with the rule
    ///
    /// This rule check fails if a meta entry is defined more than once
    /// in a rule
    ///
    /// # Arguments
    ///
    /// * `context` (`&LintContext`) - A `LintContext` containing the
    ///   `RuleNode`s to be checked
    /// * `findings` (`&mut Vec<Finding>`) - Vector to push Finding to
    fn check(&self, context: &LintContext, findings: &mut Vec<Finding>) {
        for rule in &context.file.rules {
            let mut meta_keys: HashSet<&str> = HashSet::new();
            for meta_entry in &rule.meta {
                if meta_keys.contains(meta_entry.key.as_str()) {
                    findings.push(Finding {
                        rule: self.name(),
                        message: format!(
                            "In rule {} there are two meta values named {}.",
                            rule.name, meta_entry.key
                        ),
                        severity: Severity::Warning,
                    });
                } else {
                    meta_keys.insert(&meta_entry.key);
                }
            }
        }
    }
}
