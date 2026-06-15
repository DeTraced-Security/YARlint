//! Verifies no two strings are duplicates of each other
//!
//! This is both type sensitive ("foo" and "{66 6f 6f}" are different) and
//! modifier sensitive (`"foo" wide ascii` is different than `"foo" wide` and
//! `"foo" ascii`)

use std::collections::HashMap;

use crate::{
    linter::{
        context::LintContext,
        finding::{Finding, Severity},
        rule::Rule,
    },
    parser::syntax::StringModifier,
};

/// Verifies no two strings are duplicates of each other
pub struct LintDuplicateString;

impl Rule for LintDuplicateString {
    fn name(&self) -> &'static str {
        "Lint/DuplicateString"
    }

    fn check(&self, context: &LintContext, findings: &mut Vec<Finding>) {
        for rule in &context.file.rules {
            let mut string_map: HashMap<(&str, Vec<StringModifier>), &str> = HashMap::new();

            for string in &rule.strings {
                let mut sorted_modifiers = string.modifiers.clone();
                sorted_modifiers.sort();

                let key = (string.value.as_str(), sorted_modifiers);

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
