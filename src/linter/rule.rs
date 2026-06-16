//! Lint rule trait.

use crate::linter::{context::LintContext, finding::Finding};

/// Trait implemented by all lint rules.
pub trait Rule {
    /// Rule name.
    /// 
    /// Should be implemented to returna string with the rule's name in the
    /// format [category]/[rule name]
    /// 
    /// Examples:
    /// - "Naming/RuleName" -> found in naming/rule_name.rs
    /// - "Lint/DuplicateString" -> found in lint/duplicate_string.rs
    /// 
    fn name(&self) -> &'static str;

    /// Execute the rule
    /// 
    /// Should be implemented such that the function checks for the rule's
    /// condition in the `context` and pushes a new Finding describing any
    /// non-complicance to `findings`
    /// 
    /// # Arguments
    /// 
    /// * `context` (`&LintContext`) - A `LintContext` containing the
    /// `RuleNode`s to be checked
    /// * `findings` (`&mut Vec<Finding>`) - Vector to push Finding to
    fn check(&self, context: &LintContext, findings: &mut Vec<Finding>);
}
