//! Enforces a minimum level of substantiality to the description tag in the
//! meta section of a YARA rule. Values that are too short, generic, or
//! placeholder-like (e.g. "`malware`", "`test`", "`TODO`") should be flagged.

use crate::{
    linter::{
        context::LintContext,
        finding::{Finding, Severity},
        rule::Rule,
    },
    parser::syntax::MetaValue,
};

/// Enforces a minimum level of substantiality to the description tag
pub struct NamingDescriptiveMeta;

/// Defines the minimum character length of the description. Will be made 
/// configurable in the future.
const MIN_CHAR_LENGTH: usize = 20;

/// Defines the minimum word count of the description. Will be made 
/// configurable in the future.
const MIN_WORD_COUNT: usize = 5;

/// Defines placeholder values to search for in the description. Will be made 
/// configurable in the future.
const PLACEHOLDER_VALUES: &[&str] = &[
    "malware",
    "test",
    "testing",
    "todo",
    "tbd",
    "placeholder",
    "fixme",
    "unknown",
    "sample",
    "detection",
    "rule",
    "generic",
    "description",
    "temp",
    "wip",
    "n/a",
    "none",
    "...",
    "xxx",
    "foo",
    "bar",
];

impl Rule for NamingDescriptiveMeta {
    
    /// Returns the name of the rule
    ///
    /// # Returns
    ///
    /// Returns "Naming/DescriptiveMeta"
    fn name(&self) -> &'static str {
        "Naming/DescriptiveMeta"
    }

    /// Checks the description value for descriptiveness.
    /// 
    /// It checks the description value in the meta tag for three things. If
    /// any of the things are violated, it creates a finding. Since a 
    /// description can have multiple issues with it, it alerts if all three are
    /// met. The three things are:
    /// - Not a placeholder value
    /// - Above a minimum character length
    /// - Above a minimum word count
    /// 
    /// # Arguments
    ///
    /// * `context` (`&LintContext`) - A `LintContext` containing the
    ///   `RuleNode`s to be checked
    /// * `findings` (`&mut Vec<Finding>`) - Vector to push Finding to
    fn check(&self, context: &LintContext, findings: &mut Vec<Finding>) {
        for rule in &context.file.rules {
            for meta_entry_node in rule.meta.clone() {
                if meta_entry_node.key == "description" {
                    match meta_entry_node.value.clone() {
                        MetaValue::String(description_string) => {
                            let normalized_description_string =
                                description_string.trim().to_lowercase();
                            let words: Vec<&str> =
                                normalized_description_string.split_whitespace().collect();

                            
                            if PLACEHOLDER_VALUES.contains(&normalized_description_string.as_str()) {
                                findings.push(
                                    Finding { 
                                        rule: self.name(), 
                                        message: format!(
                                            "Rule {}'s description tag is a generic placeholder value \"{}\"", 
                                            rule.name, normalized_description_string
                                        ), 
                                        severity: Severity::Info
                                    }
                                );
                            }
                            
                            if normalized_description_string.chars().count() < MIN_CHAR_LENGTH {
                                findings.push(
                                    Finding { 
                                        rule: self.name(), 
                                        message: format!(
                                            "Description value in {} is shorter than minimum character length ({})", 
                                            rule.name, MIN_CHAR_LENGTH
                                        ), 
                                        severity: Severity::Info 
                                    }
                                );
                            }

                            if words.len() < MIN_WORD_COUNT {
                                findings.push(
                                    Finding { 
                                        rule: self.name(), 
                                        message: format!(
                                            "Description value in {} is shorter than minimum word count ({})", 
                                            rule.name, MIN_WORD_COUNT
                                        ), 
                                        severity: Severity::Info 
                                    }
                                );
                            }
                        }
                        MetaValue::Number(_) => continue,
                        MetaValue::Boolean(_) => continue,
                    }
                }
            }
        }
    }
}
