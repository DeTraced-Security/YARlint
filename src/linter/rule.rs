//! Lint rule trait.

use crate::linter::{context::LintContext, finding::Finding};

/// Trait implemented by all lint rules.
pub trait Rule {
    /// Rule name.
    fn name(&self) -> &'static str;

    /// Execute the rule.
    fn check(&self, context: &LintContext, findings: &mut Vec<Finding>);
}
