//! Validates that string identifiers are snake_case.

use crate::linter::{
    context::LintContext,
    finding::{Finding, Severity},
    rule::Rule,
};

/// Validates that string identifiers are snake_case.
pub struct StyleStringIdentifier;

impl Rule for StyleStringIdentifier {
    fn name(&self) -> &'static str {
        "Style/StringIdentifier"
    }

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
/// Checks if string passed is snake case. Returns true if, and false if not.
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

#[cfg(test)]
mod tests {
    use super::*;

    // is_snake_case - valid cases
    #[test]
    fn is_snake_case_accepts_lowercase_word() {
        assert!(is_snake_case("foo"));
    }

    #[test]
    fn is_snake_case_accepts_dollar_prefix() {
        assert!(is_snake_case("$foo"));
    }

    #[test]
    fn is_snake_case_accepts_underscore_prefix() {
        assert!(is_snake_case("_foo"));
    }

    #[test]
    fn is_snake_case_accepts_words_joined_by_underscore() {
        assert!(is_snake_case("foo_bar"));
    }

    #[test]
    fn is_snake_case_accepts_digits_in_identifier() {
        assert!(is_snake_case("foo_1"));
    }

    #[test]
    fn is_snake_case_accepts_dollar_prefix_with_underscores() {
        assert!(is_snake_case("$foo_bar"));
    }

    // is_snake_case - invalid cases
    #[test]
    fn is_snake_case_rejects_empty_string() {
        assert!(!is_snake_case(""));
    }

    #[test]
    fn is_snake_case_rejects_uppercase_start() {
        assert!(!is_snake_case("Foo"));
    }

    #[test]
    fn is_snake_case_rejects_camel_case() {
        assert!(!is_snake_case("fooBar"));
    }

    #[test]
    fn is_snake_case_rejects_dollar_prefix_followed_by_uppercase() {
        assert!(!is_snake_case("$Foo"));
    }

    #[test]
    fn is_snake_case_rejects_trailing_underscore() {
        assert!(!is_snake_case("foo_"));
    }

    #[test]
    fn is_snake_case_rejects_double_underscore() {
        assert!(!is_snake_case("foo__bar"));
    }

    #[test]
    fn is_snake_case_rejects_digit_start() {
        assert!(!is_snake_case("1foo"));
    }

    #[test]
    fn is_snake_case_rejects_underscore_only() {
        assert!(!is_snake_case("_"));
    }

    #[test]
    fn is_snake_case_rejects_dollar_only() {
        assert!(!is_snake_case("$"));
    }
}
