//! Rule name validation.

use crate::linter::{
    context::LintContext,
    finding::{Finding, Severity},
    rule::Rule,
};

/// Validates rule naming conventions.
pub struct NamingRuleName;

impl Rule for NamingRuleName {
    fn name(&self) -> &'static str {
        "Naming/RuleName"
    }

    fn check(&self, context: &LintContext, findings: &mut Vec<Finding>) {
        for rule in &context.file.rules {
            if rule.name.contains('-') {
                findings.push(Finding {
                    rule: self.name(),
                    message: format!("Rule '{}' contains '-'", rule.name),
                    severity: Severity::Warning,
                });
            }
        }
    }
}
