//! Parsing of YARA string definitions.
//!
//! This module parses string declarations and their associated
//! modifiers, producing [`StringNode`] values suitable for AST
//! construction and later analysis.

use crate::parser::{
    ast_parser::AstParser,
    syntax::{StringModifier, StringNode},
    token::TokenType,
};

/// Parses the `strings` section of a YARA rule.
///
/// This function consumes string declarations until the beginning of
/// the `condition` section is encountered. Each string declaration
/// consists of a string identifier, an equals sign, a string literal,
/// and zero or more string modifiers.
///
/// Supported modifiers include:
/// - `wide`
/// - `ascii`
/// - `fullword`
/// - `nocase`
/// - `xor`
/// - `base64`
/// - `base64wide`
///
/// # Arguments
///
/// * `parser` - The active parser positioned at the first string
///   declaration following the `strings:` section header.
///
/// # Returns
///
/// Returns a vector containing all parsed [`StringNode`] values.
///
/// # Errors
///
/// Returns an error if:
/// - A string identifier is missing or malformed.
/// - An equals sign is missing.
/// - A string literal is missing or invalid.
/// - The input ends unexpectedly while parsing string declarations.
pub fn parse_strings(parser: &mut AstParser) -> Result<Vec<StringNode>, String> {
    let mut strings = Vec::new();

    while !parser.peek_keyword("condition") {
        let identifier = parser.expect_string_identifier()?;

        parser.expect(&TokenType::Equals)?;

        let value = parser.expect_string_literal()?;

        let mut modifiers = Vec::new();

        while let Some(token) = parser.peek() {
            match &token.token_type {
                TokenType::Keyword(k) if k == "wide" => {
                    parser.advance();
                    modifiers.push(StringModifier::Wide);
                }

                TokenType::Keyword(k) if k == "ascii" => {
                    parser.advance();
                    modifiers.push(StringModifier::Ascii);
                }

                TokenType::Keyword(k) if k == "fullword" => {
                    parser.advance();
                    modifiers.push(StringModifier::Fullword);
                }

                TokenType::Keyword(k) if k == "nocase" => {
                    parser.advance();
                    modifiers.push(StringModifier::Nocase);
                }

                TokenType::Keyword(k) if k == "xor" => {
                    parser.advance();
                    modifiers.push(StringModifier::Xor);
                }

                TokenType::Keyword(k) if k == "base64" => {
                    parser.advance();
                    modifiers.push(StringModifier::Base64);
                }

                TokenType::Keyword(k) if k == "base64wide" => {
                    parser.advance();
                    modifiers.push(StringModifier::Base64Wide);
                }

                _ => break,
            }
        }

        strings.push(StringNode {
            identifier,
            value,
            modifiers,
        });
    }

    Ok(strings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{ast_parser::AstParser, lexer::tokenize, syntax::StringModifier};

    fn make_parser_for_strings(source: &str) -> AstParser {
        // wrap with a "condition" keyword so the strings parser has a stop token
        let full = format!("{} condition", source);
        let tokens = tokenize(&full).unwrap();
        AstParser::new(tokens)
    }

    // --- happy path ---

    #[test]
    fn parse_strings_returns_empty_vec_when_no_entries() {
        let mut parser = make_parser_for_strings("");

        let result = parse_strings(&mut parser).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn parse_strings_parses_single_string() {
        let mut parser = make_parser_for_strings(r#"$s1 = "cmd.exe""#);

        let result = parse_strings(&mut parser).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].identifier, "$s1");
        assert_eq!(result[0].value, "cmd.exe");
    }

    #[test]
    fn parse_strings_parses_multiple_strings() {
        let mut parser = make_parser_for_strings(r#"$s1 = "cmd.exe" $s2 = "powershell""#);

        let result = parse_strings(&mut parser).unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].identifier, "$s1");
        assert_eq!(result[1].identifier, "$s2");
    }

    #[test]
    fn parse_strings_parses_empty_string_value() {
        let mut parser = make_parser_for_strings(r#"$s1 = """#);

        let result = parse_strings(&mut parser).unwrap();

        assert_eq!(result[0].value, "");
    }

    #[test]
    fn parse_strings_stops_at_condition_keyword() {
        let mut parser = make_parser_for_strings(r#"$s1 = "foo""#);

        let result = parse_strings(&mut parser).unwrap();

        // should not consume the "condition" keyword
        assert_eq!(result.len(), 1);
    }

    // --- modifiers ---

    #[test]
    fn parse_strings_parses_wide_modifier() {
        let mut parser = make_parser_for_strings(r#"$s1 = "foo" wide"#);

        let result = parse_strings(&mut parser).unwrap();

        assert!(result[0].modifiers.contains(&StringModifier::Wide));
    }

    #[test]
    fn parse_strings_parses_ascii_modifier() {
        let mut parser = make_parser_for_strings(r#"$s1 = "foo" ascii"#);

        let result = parse_strings(&mut parser).unwrap();

        assert!(result[0].modifiers.contains(&StringModifier::Ascii));
    }

    #[test]
    fn parse_strings_parses_fullword_modifier() {
        let mut parser = make_parser_for_strings(r#"$s1 = "foo" fullword"#);

        let result = parse_strings(&mut parser).unwrap();

        assert!(result[0].modifiers.contains(&StringModifier::Fullword));
    }

    #[test]
    fn parse_strings_parses_nocase_modifier() {
        let mut parser = make_parser_for_strings(r#"$s1 = "foo" nocase"#);

        let result = parse_strings(&mut parser).unwrap();

        assert!(result[0].modifiers.contains(&StringModifier::Nocase));
    }

    #[test]
    fn parse_strings_parses_xor_modifier() {
        let mut parser = make_parser_for_strings(r#"$s1 = "foo" xor"#);

        let result = parse_strings(&mut parser).unwrap();

        assert!(result[0].modifiers.contains(&StringModifier::Xor));
    }

    #[test]
    fn parse_strings_parses_base64_modifier() {
        let mut parser = make_parser_for_strings(r#"$s1 = "foo" base64"#);

        let result = parse_strings(&mut parser).unwrap();

        assert!(result[0].modifiers.contains(&StringModifier::Base64));
    }

    #[test]
    fn parse_strings_parses_base64wide_modifier() {
        let mut parser = make_parser_for_strings(r#"$s1 = "foo" base64wide"#);

        let result = parse_strings(&mut parser).unwrap();

        assert!(result[0].modifiers.contains(&StringModifier::Base64Wide));
    }

    #[test]
    fn parse_strings_parses_multiple_modifiers() {
        let mut parser = make_parser_for_strings(r#"$s1 = "foo" wide ascii nocase"#);

        let result = parse_strings(&mut parser).unwrap();

        assert!(result[0].modifiers.contains(&StringModifier::Wide));
        assert!(result[0].modifiers.contains(&StringModifier::Ascii));
        assert!(result[0].modifiers.contains(&StringModifier::Nocase));
    }

    #[test]
    fn parse_strings_string_with_no_modifiers_has_empty_modifiers() {
        let mut parser = make_parser_for_strings(r#"$s1 = "foo""#);

        let result = parse_strings(&mut parser).unwrap();

        assert!(result[0].modifiers.is_empty());
    }

    #[test]
    fn parse_strings_modifiers_do_not_bleed_across_strings() {
        let mut parser = make_parser_for_strings(r#"$s1 = "foo" wide $s2 = "bar""#);

        let result = parse_strings(&mut parser).unwrap();

        assert!(result[0].modifiers.contains(&StringModifier::Wide));
        assert!(result[1].modifiers.is_empty());
    }

    // --- error cases ---

    #[test]
    fn parse_strings_returns_err_on_missing_equals() {
        let mut parser = make_parser_for_strings(r#"$s1 "foo""#);

        assert!(parse_strings(&mut parser).is_err());
    }

    #[test]
    fn parse_strings_returns_err_on_missing_string_value() {
        let mut parser = make_parser_for_strings("$s1 =");

        assert!(parse_strings(&mut parser).is_err());
    }

    #[test]
    fn parse_strings_returns_err_on_non_string_identifier() {
        // plain identifier instead of $ prefixed one
        let mut parser = make_parser_for_strings(r#"s1 = "foo""#);

        assert!(parse_strings(&mut parser).is_err());
    }

    #[test]
    fn parse_strings_returns_err_when_value_is_not_string_literal() {
        // number instead of string literal
        let mut parser = make_parser_for_strings("$s1 = 42");

        assert!(parse_strings(&mut parser).is_err());
    }
}
