//! Validates that string identifiers are snake_case.

use crate::linter::{
    context::LintContext,
    finding::{Finding, Severity},
    rule::Rule,
};

/// Validates that string identifiers are snake_case.
pub struct StyleStringIdentifier;

impl Rule for StyleStringIdentifier {
    /// Returns the name of the rule
    ///
    /// # Returns
    ///
    /// Returns "Style/StringIdenifier"
    fn name(&self) -> &'static str {
        "Style/StringIdentifier"
    }

    /// Checks for compliance with the rule
    ///
    /// This rule is violated if any rules contain `StringNode`s not in snake
    /// case
    ///
    /// # Arguments
    ///
    /// * `context` (`&LintContext`) - A `LintContext` containing the
    /// `RuleNode`s to be checked
    /// * `findings` (`&mut Vec<Finding>`) - Vector to push Finding to
    fn check(&self, context: &LintContext, findings: &mut Vec<Finding>) {
        for rule in &context.file.rules {
            for string in &rule.strings {
                if !is_snake_case(&string.identifier) {
                    findings.push(Finding {
                        rule: self.name(),
                        message: format!(
                            "String identifier {} in rule {} should be snake_case",
                            string.identifier, rule.name
                        ),
                        severity: Severity::Warning,
                    })
                }
            }
        }
    }
}

/// Checks if string passed is snake case
///
/// # Arguments
///
/// * `s` (`&str`) - the string to be evaluated
///
/// # Returns
///
/// Returns `true` if string is in snake case
/// Returns `false` if string is not in snake case
/// Returns `false` if string is empty
fn is_snake_case(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let mut chars = s.chars().peekable();

    match chars.next() {
        Some(c) if c.is_ascii_lowercase() => {}
        Some('_') | Some('$') => match chars.peek() {
            Some(next) if next.is_ascii_lowercase() => {}
            _ => return false,
        },
        _ => return false,
    }

    while let Some(c) = chars.next() {
        if c.is_ascii_lowercase() || c.is_ascii_digit() {
            continue;
        }
        if c == '_' {
            match chars.peek() {
                Some(next) if next.is_ascii_lowercase() || next.is_ascii_digit() => {}
                _ => return false,
            }
        } else {
            return false;
        }
    }

    true
}
