//! Lint cop trait.

use crate::linter::{context::LintContext, finding::Finding};

/// Trait implemented by all lint cops.
pub trait Cop {
    /// Rule name.
    ///
    /// Should be implemented to return a string with the cop's name in the
    /// format [category]/[cop name]
    ///
    /// Examples:
    /// - "Naming/CopName" -> found in naming/cop_name.rs
    /// - "Lint/DuplicateString" -> found in lint/duplicate_string.rs
    ///
    fn name(&self) -> &'static str;

    /// Execute the cop
    ///
    /// Should be implemented such that the function checks for the cop's
    /// condition in the `context` and pushes a new Finding describing any
    /// non-compliance to `findings`
    ///
    /// # Arguments
    ///
    /// * `context` (`&LintContext`) - A `LintContext` containing the
    ///   `RuleNode`s to be checked
    /// * `findings` (`&mut Vec<Finding>`) - Vector to push Finding to
    fn check(&self, context: &LintContext, findings: &mut Vec<Finding>);
}
