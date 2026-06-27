//! Enforces a minimum and maximum length on YARA rule names. THis is so the
//! names are not too short to be useful but also not so long that its hard to
//! understand them

use crate::linter::{
    context::LintContext,
    finding::{Finding, Severity},
    rule::Rule,
};

/// Enforces a minimum and maximum length on YARA rule names.
pub struct NamingRuleNameLength;

/// Minimum length of a name in chars. Will be swapped out for a configuration
/// file eventually
const MIN_NAME_LENGTH: usize = 4;

/// Maximum length of a name in chars. Will be swapped out for a configuration
/// file eventually
const MAX_NAME_LENGTH: usize = 80;

impl Rule for NamingRuleNameLength {
    fn name(&self) -> &'static str {
        "Naming/RuleNameLength"
    }

    fn check(&self, context: &LintContext, findings: &mut Vec<Finding>) {
        for rule in &context.file.rules {
            let name = rule.name.clone();

            if name.chars().count() > MAX_NAME_LENGTH {
                findings.push(Finding {
                    rule: self.name(),
                    message: format!(
                        "The name {} is longer than the maximum character length of {}",
                        rule.name, MAX_NAME_LENGTH
                    ),
                    severity: Severity::Info,
                });
            } else if name.chars().count() < MIN_NAME_LENGTH {
                findings.push(Finding {
                    rule: self.name(),
                    message: format!(
                        "The name {} is shorter than the minimum character length of {}",
                        rule.name, MIN_NAME_LENGTH
                    ),
                    severity: Severity::Info,
                });
            }
        }
    }
}
