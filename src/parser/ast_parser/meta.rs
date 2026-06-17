//! Parsing of YARA meta sections.
//!
//! The `meta` section contains key-value pairs that provide descriptive
//! information about a rule. This module converts metadata tokens into
//! collections of [`MetaEntryNode`] values.

use crate::parser::{
    ast_parser::AstParser,
    syntax::{MetaEntryNode, MetaValue},
    token::{Token, TokenType},
};

/// Parses the `meta` section of a YARA rule.
///
/// This function consumes metadata entries until the beginning of the
/// `strings` section is encountered. Each entry consists of an
/// identifier key, an equals sign, and a value.
///
/// Supported metadata value types include:
/// - String literals
/// - Numeric literals
/// - Boolean values (`true` and `false`)
///
/// # Arguments
///
/// * `parser` - The active parser positioned at the first metadata
///   entry following the `meta:` section header.
///
/// # Returns
///
/// Returns a vector containing all parsed [`MetaEntryNode`] values.
///
/// # Errors
///
/// Returns an error if:
/// - A metadata key is not a valid identifier.
/// - An equals sign is missing.
/// - A metadata value is not a supported type.
/// - The input ends unexpectedly while parsing metadata.
pub fn parse_meta(parser: &mut AstParser) -> Result<Vec<MetaEntryNode>, String> {
    let mut meta_entries: Vec<MetaEntryNode> = Vec::new();

    while !parser.check(&crate::parser::token::TokenType::Keyword(
        "strings".to_string(),
    )) {
        let key: String = parser.expect_identifier()?;
        parser.expect(&TokenType::Equals)?;

        let value: MetaValue = match parser.peek() {
            Some(Token {
                token_type: TokenType::StringLiteral(_),
                ..
            }) => MetaValue::String(parser.expect_string_literal()?),

            Some(Token {
                token_type: TokenType::Number(_),
                ..
            }) => MetaValue::Number(parser.expect_number()?),

            Some(Token {
                token_type: TokenType::Keyword(k),
                ..
            }) if k == "true" || k == "false" => MetaValue::Boolean(parser.expect_boolean()?),

            _ => return Err("Expected meta value".into()),
        };

        meta_entries.push(MetaEntryNode { key, value });
    }

    Ok(meta_entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{ast_parser::AstParser, lexer::tokenize, syntax::MetaValue};

    fn make_parser_for_meta(source: &str) -> AstParser {
        // wrap in a fake rule context so the meta parser has a "strings" keyword to stop at
        let full = format!("{} strings:", source);
        let tokens = tokenize(&full).unwrap();
        AstParser::new(tokens)
    }

    // --- happy path ---

    #[test]
    fn parse_meta_returns_empty_vec_when_no_entries() {
        let mut parser = make_parser_for_meta("");

        let result = parse_meta(&mut parser).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn parse_meta_parses_string_value() {
        let mut parser = make_parser_for_meta(r#"author = "DeTraced Security""#);

        let result = parse_meta(&mut parser).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].key, "author");
        assert!(matches!(&result[0].value, MetaValue::String(s) if s == "DeTraced Security"));
    }

    #[test]
    fn parse_meta_parses_number_value() {
        let mut parser = make_parser_for_meta("version = 1");

        let result = parse_meta(&mut parser).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].key, "version");
        assert!(matches!(&result[0].value, MetaValue::Number(n) if n == "1"));
    }

    #[test]
    fn parse_meta_parses_true_boolean_value() {
        let mut parser = make_parser_for_meta("is_test = true");

        let result = parse_meta(&mut parser).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].key, "is_test");
        assert!(matches!(&result[0].value, MetaValue::Boolean(true)));
    }

    #[test]
    fn parse_meta_parses_false_boolean_value() {
        let mut parser = make_parser_for_meta("is_test = false");

        let result = parse_meta(&mut parser).unwrap();

        assert_eq!(result.len(), 1);
        assert!(matches!(&result[0].value, MetaValue::Boolean(false)));
    }

    #[test]
    fn parse_meta_parses_multiple_entries() {
        let mut parser = make_parser_for_meta(r#"author = "DeTraced Security" version = 1 is_test = true"#);

        let result = parse_meta(&mut parser).unwrap();

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].key, "author");
        assert_eq!(result[1].key, "version");
        assert_eq!(result[2].key, "is_test");
    }

    #[test]
    fn parse_meta_stops_at_strings_keyword() {
        let mut parser = make_parser_for_meta(r#"author = "DeTraced Security""#);

        let result = parse_meta(&mut parser).unwrap();

        // should not consume the "strings" keyword — it belongs to the parent parser
        assert_eq!(result.len(), 1);
    }

    // --- error cases ---

    #[test]
    fn parse_meta_returns_err_on_missing_equals() {
        let mut parser = make_parser_for_meta(r#"author "DeTraced Security""#);

        assert!(parse_meta(&mut parser).is_err());
    }

    #[test]
    fn parse_meta_returns_err_on_invalid_key() {
        // key must be an identifier, not a string literal
        let mut parser = make_parser_for_meta(r#""author" = "DeTraced Security""#);

        assert!(parse_meta(&mut parser).is_err());
    }

    #[test]
    fn parse_meta_returns_err_on_unsupported_value_type() {
        // LBrace is not a valid meta value
        let mut parser = make_parser_for_meta("author = {");

        assert!(parse_meta(&mut parser).is_err());
    }

    #[test]
    fn parse_meta_returns_err_on_non_boolean_keyword_value() {
        // "rule" is a keyword but not a valid meta value
        let mut parser = make_parser_for_meta("author = rule");

        assert!(parse_meta(&mut parser).is_err());
    }
}
