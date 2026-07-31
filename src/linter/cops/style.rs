//! The parent module for all cops pertaining to style of YARA rules

pub mod meta_keys_order;
pub mod missing_required_meta;
pub mod rule_name_case;
pub mod string_identifier;

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
        // The $ is for checking identifiers. Is does not impact the ability
        // to pass rule names into this function
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

/// Checks if string passed is pascal case
///
/// # Arguments
///
/// * `s` (`&str`) - the string to be evaluated
///
/// # Returns
///
/// Returns `true` if string is in pascal case
/// Returns `false` if string is not in pascal case
/// Returns `false` if string is empty
fn is_pascal_case(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let mut chars = s.chars();

    match chars.next() {
        Some(c) if c.is_ascii_uppercase() => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric())
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
