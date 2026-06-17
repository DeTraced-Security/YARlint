use yarlint::parser::lexer::tokenize;
use yarlint::parser::token::TokenType;

// --- happy path ---

#[test]
fn empty_input_returns_empty_token_list() {
    let result = tokenize("");

    assert_eq!(result, Ok(vec![]));
}

#[test]
fn whitespace_only_returns_empty_token_list() {
    let result = tokenize("   \t\n\r\n");

    assert_eq!(result, Ok(vec![]));
}

// --- punctuation and operators ---

#[test]
fn left_brace_produces_lbrace_token() {
    let tokens = tokenize("{").unwrap();

    assert_eq!(tokens[0].token_type, TokenType::LBrace);
}

#[test]
fn right_brace_produces_rbrace_token() {
    let tokens = tokenize("}").unwrap();

    assert_eq!(tokens[0].token_type, TokenType::RBrace);
}

#[test]
fn colon_produces_colon_token() {
    let tokens = tokenize(":").unwrap();

    assert_eq!(tokens[0].token_type, TokenType::Colon);
}

#[test]
fn left_paren_produces_lparen_token() {
    let tokens = tokenize("(").unwrap();

    assert_eq!(tokens[0].token_type, TokenType::LParen);
}

#[test]
fn right_paren_produces_rparen_token() {
    let tokens = tokenize(")").unwrap();

    assert_eq!(tokens[0].token_type, TokenType::RParen);
}

#[test]
fn star_produces_star_token() {
    let tokens = tokenize("*").unwrap();

    assert_eq!(tokens[0].token_type, TokenType::Star);
}

#[test]
fn dot_produces_dot_token() {
    let tokens = tokenize(".").unwrap();

    assert_eq!(tokens[0].token_type, TokenType::Dot);
}

#[test]
fn at_symbol_produces_at_token() {
    let tokens = tokenize("@").unwrap();

    assert_eq!(tokens[0].token_type, TokenType::AtSymbol);
}

#[test]
fn minus_produces_minus_token() {
    let tokens = tokenize("-").unwrap();

    assert_eq!(tokens[0].token_type, TokenType::Minus);
}

#[test]
fn plus_produces_plus_token() {
    let tokens = tokenize("+").unwrap();

    assert_eq!(tokens[0].token_type, TokenType::Plus);
}

#[test]
fn forward_slash_produces_fslash_token() {
    let tokens = tokenize("/").unwrap();

    assert_eq!(tokens[0].token_type, TokenType::FSlash);
}

// --- comparison operators ---

#[test]
fn single_equals_produces_equals_token() {
    let tokens = tokenize("=").unwrap();

    assert_eq!(tokens[0].token_type, TokenType::Equals);
}

#[test]
fn double_equals_produces_equals_equals_token() {
    let tokens = tokenize("==").unwrap();

    assert_eq!(tokens[0].token_type, TokenType::EqualsEquals);
}

#[test]
fn greater_than_produces_gthan_token() {
    let tokens = tokenize(">").unwrap();

    assert_eq!(tokens[0].token_type, TokenType::GThan);
}

#[test]
fn greater_than_or_equal_produces_gethan_token() {
    let tokens = tokenize(">=").unwrap();

    assert_eq!(tokens[0].token_type, TokenType::GEThan);
}

#[test]
fn less_than_produces_lthan_token() {
    let tokens = tokenize("<").unwrap();

    assert_eq!(tokens[0].token_type, TokenType::LThan);
}

#[test]
fn less_than_or_equal_produces_lethan_token() {
    let tokens = tokenize("<=").unwrap();

    assert_eq!(tokens[0].token_type, TokenType::LEThan);
}

// --- comments ---

#[test]
fn line_comment_is_ignored() {
    let tokens = tokenize("// this is a comment").unwrap();

    assert!(tokens.is_empty());
}

#[test]
fn line_comment_does_not_consume_next_line() {
    let tokens = tokenize("// comment\nrule").unwrap();

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Keyword("rule".to_string()));
}

#[test]
fn block_comment_is_ignored() {
    let tokens = tokenize("/* this is a block comment */").unwrap();

    assert!(tokens.is_empty());
}

#[test]
fn block_comment_does_not_consume_surrounding_tokens() {
    let tokens = tokenize("rule /* comment */ foo").unwrap();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Keyword("rule".to_string()));
    assert_eq!(
        tokens[1].token_type,
        TokenType::Identifier("foo".to_string())
    );
}

#[test]
fn multiline_block_comment_is_ignored() {
    let tokens = tokenize("/* line one\nline two\nline three */").unwrap();

    assert!(tokens.is_empty());
}

// --- string literals ---

#[test]
fn string_literal_produces_string_literal_token() {
    let tokens = tokenize(r#""hello""#).unwrap();

    assert_eq!(
        tokens[0].token_type,
        TokenType::StringLiteral("hello".to_string())
    );
}

#[test]
fn empty_string_literal_produces_empty_string_literal_token() {
    let tokens = tokenize(r#""""#).unwrap();

    assert_eq!(
        tokens[0].token_type,
        TokenType::StringLiteral("".to_string())
    );
}

#[test]
fn escaped_quote_in_string_literal_is_preserved() {
    let tokens = tokenize(r#""foo\"bar""#).unwrap();

    assert_eq!(
        tokens[0].token_type,
        TokenType::StringLiteral(r#"foo\"bar"#.to_string())
    );
}

// --- string identifiers ---

#[test]
fn dollar_prefix_produces_string_identifier_token() {
    let tokens = tokenize("$foo").unwrap();

    assert_eq!(
        tokens[0].token_type,
        TokenType::StringIdentifier("$foo".to_string())
    );
}

#[test]
fn string_identifier_with_underscore_is_tokenized() {
    let tokens = tokenize("$foo_bar").unwrap();

    assert_eq!(
        tokens[0].token_type,
        TokenType::StringIdentifier("$foo_bar".to_string())
    );
}

#[test]
fn string_identifier_with_digits_is_tokenized() {
    let tokens = tokenize("$foo1").unwrap();

    assert_eq!(
        tokens[0].token_type,
        TokenType::StringIdentifier("$foo1".to_string())
    );
}

#[test]
fn bare_dollar_produces_dollar_only_string_identifier() {
    // $ alone with no alphanumeric chars following is still a StringIdentifier
    let tokens = tokenize("$").unwrap();

    assert_eq!(
        tokens[0].token_type,
        TokenType::StringIdentifier("$".to_string())
    );
}

// --- keywords ---

#[test]
fn rule_keyword_produces_keyword_token() {
    let tokens = tokenize("rule").unwrap();

    assert_eq!(tokens[0].token_type, TokenType::Keyword("rule".to_string()));
}

#[test]
fn condition_keyword_produces_keyword_token() {
    let tokens = tokenize("condition").unwrap();

    assert_eq!(
        tokens[0].token_type,
        TokenType::Keyword("condition".to_string())
    );
}

#[test]
fn strings_keyword_produces_keyword_token() {
    let tokens = tokenize("strings").unwrap();

    assert_eq!(
        tokens[0].token_type,
        TokenType::Keyword("strings".to_string())
    );
}

#[test]
fn all_keywords_are_recognized() {
    use yarlint::parser::lexer::KEYWORDS;

    for keyword in KEYWORDS {
        let tokens = tokenize(keyword).unwrap();
        assert_eq!(
            tokens[0].token_type,
            TokenType::Keyword(keyword.to_string()),
            "keyword '{}' was not recognized",
            keyword
        );
    }
}

// --- identifiers ---

#[test]
fn unknown_word_produces_identifier_token() {
    let tokens = tokenize("myrule").unwrap();

    assert_eq!(
        tokens[0].token_type,
        TokenType::Identifier("myrule".to_string())
    );
}

#[test]
fn identifier_with_underscore_prefix_is_tokenized() {
    let tokens = tokenize("_foo").unwrap();

    assert_eq!(
        tokens[0].token_type,
        TokenType::Identifier("_foo".to_string())
    );
}

#[test]
fn all_identifiers_are_recognized() {
    use yarlint::parser::lexer::IDENTIFIERS;

    for identifier in IDENTIFIERS {
        let tokens = tokenize(identifier).unwrap();
        assert_eq!(
            tokens[0].token_type,
            TokenType::Identifier(identifier.to_string()),
            "identifier '{}' was not recognized",
            identifier
        );
    }
}

// --- numbers ---

#[test]
fn integer_produces_number_token() {
    let tokens = tokenize("42").unwrap();

    assert_eq!(tokens[0].token_type, TokenType::Number("42".to_string()));
}

#[test]
fn hex_number_produces_number_token() {
    // hex digits are alphanumeric so 0x1A is consumed as one number token
    let tokens = tokenize("0x1A").unwrap();

    assert_eq!(tokens[0].token_type, TokenType::Number("0x1A".to_string()));
}

// --- unknown characters ---

#[test]
fn unknown_character_produces_unknown_token() {
    let tokens = tokenize("~").unwrap();

    assert_eq!(tokens[0].token_type, TokenType::Unknown('~'));
}

// --- span tracking ---

#[test]
fn first_token_on_first_line_has_correct_span() {
    let tokens = tokenize("rule").unwrap();

    assert_eq!(tokens[0].span.line, 1);
    assert_eq!(tokens[0].span.column, 1);
}

#[test]
fn token_on_second_line_has_correct_line_number() {
    let tokens = tokenize("rule\nfoo").unwrap();

    assert_eq!(tokens[1].span.line, 2);
}

// --- error cases ---

#[test]
fn unterminated_string_literal_returns_err() {
    let result = tokenize(r#""unterminated"#);

    assert!(result.is_err());
}

#[test]
fn unterminated_block_comment_returns_err() {
    let result = tokenize("/* unterminated");

    assert!(result.is_err());
}

#[test]
fn error_message_includes_location_for_unterminated_string() {
    let result = tokenize(r#""unterminated"#);

    assert!(result.unwrap_err().contains("1:"));
}

#[test]
fn error_message_includes_location_for_unterminated_block_comment() {
    let result = tokenize("/* unterminated");

    assert!(result.unwrap_err().contains("1:"));
}

// --- full rule ---

#[test]
fn minimal_valid_rule_tokenizes_without_error() {
    let source = r#"rule foo { condition: true }"#;

    let result = tokenize(source);

    assert!(result.is_ok());
}

#[test]
fn minimal_valid_rule_produces_correct_token_count() {
    let source = r#"rule foo { condition: true }"#;
    let tokens = tokenize(source).unwrap();

    // rule, foo, {, condition, :, true, }
    assert_eq!(tokens.len(), 7);
}
