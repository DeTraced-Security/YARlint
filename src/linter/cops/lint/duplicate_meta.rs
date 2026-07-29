//! Verifies no two meta entries are duplicates of each other
//!
//! This does not check if two entries have the same value, rather if
//! two entries have the same key.

use std::collections::HashSet;

use crate::linter::{
    context::LintContext,
    cop::{Category, Cop},
    finding::{Finding, Severity},
};

/// Verifies no two meta entries are duplicates of each other
pub struct LintDuplicateMeta;

impl Cop for LintDuplicateMeta {
    /// Returns the category of the cop
    fn category(&self) -> Category {
        Category::Lint
    }

    /// Returns the name of the cop
    ///
    /// # Returns
    ///
    /// Returns "DuplicateMeta"
    fn name(&self) -> &'static str {
        "DuplicateMeta"
    }

    /// Checks for compliance with the cop
    ///
    /// This cop check fails if a meta entry is defined more than once
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
                        category: self.category(),
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
