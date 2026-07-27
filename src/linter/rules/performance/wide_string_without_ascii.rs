//! Checks if a Text or RegEx string is using the `wide` modifier without
//! using the `ascii` modifier
use crate::{
    linter::{
        context::LintContext,
        finding::{Finding, Severity},
        rule::Rule,
    },
    parser::syntax::{
        StringModifier::{Ascii, Wide},
        strings::StringType,
    },
};

/// Checks if a Text or RegEx string is using the `wide` modifier without
/// using the `ascii` modifier
pub struct PerformanceWideStringWithoutAscii;

impl Rule for PerformanceWideStringWithoutAscii {
    fn name(&self) -> &'static str {
        "Performance/WideStringWithoutAscii"
    }

    fn check(&self, context: &LintContext, findings: &mut Vec<Finding>) {
        for rule in &context.file.rules {
            for string in &rule.strings {
                if match string.value {
                    StringType::Text(_) => true,
                    StringType::Hex(_) => false,
                    StringType::RegEx(_) => true,
                } && string.modifiers.contains(&Wide)
                    && !string.modifiers.contains(&Ascii)
                {
                    findings.push(Finding {
                        rule:self.name(),
                        message: format!("The string {} in rule {} uses the modifier 'Wide' without the modifier 'Ascii'", string.identifier, rule.name),
                        severity: Severity::Info,
                    });
                }
            }
        }
    }
}
