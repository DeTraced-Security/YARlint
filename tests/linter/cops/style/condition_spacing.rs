use yarlint::{
    linter::{
        context::LintContext,
        cop::{Category, Cop},
        cops::style::condition_spacing::StyleConditionSpacing,
        default_engine_with_tokens,
        engine::LintEngine,
        finding::{Finding, Severity},
    },
    parser::{
        span::Span,
        syntax::{ConditionNode, ExprNode, RuleNode, rule_file::RuleFileNode},
        token::{Token, TokenType},
    },
};

fn token(token_type: TokenType, line: usize, column: usize) -> Token {
    Token {
        token_type,
        span: Span { line, column },
    }
}

fn keyword(value: &str, line: usize, column: usize) -> Token {
    token(TokenType::Keyword(value.to_string()), line, column)
}

fn condition_tokens(mut expression: Vec<Token>) -> Vec<Token> {
    let mut tokens = vec![keyword("condition", 1, 1), token(TokenType::Colon, 1, 10)];
    tokens.append(&mut expression);
    tokens.push(token(TokenType::RBrace, 4, 1));
    tokens
}

fn rule_file() -> RuleFileNode {
    RuleFileNode {
        imports: Vec::new(),
        rules: vec![RuleNode {
            name: "Example".to_string(),
            is_global: false,
            is_private: false,
            tags: Vec::new(),
            meta: Vec::new(),
            strings: Vec::new(),
            condition: ConditionNode {
                expression: ExprNode::AllOfThem,
            },
        }],
    }
}

fn lint(tokens: Vec<Token>) -> Vec<Finding> {
    let file = rule_file();
    let context = LintContext { file: &file };
    let mut engine = LintEngine::new();
    engine.register(StyleConditionSpacing::new(tokens));
    engine.run(&context)
}

#[test]
fn single_spaces_are_accepted() {
    let tokens = condition_tokens(vec![
        token(TokenType::StringIdentifier("$a".to_string()), 2, 1),
        keyword("and", 2, 4),
        keyword("not", 2, 8),
        token(TokenType::StringIdentifier("$b".to_string()), 2, 12),
        keyword("or", 2, 15),
        token(TokenType::Identifier("filesize".to_string()), 2, 18),
        token(TokenType::GEThan, 2, 27),
        token(TokenType::Number("10".to_string()), 2, 30),
    ]);

    assert!(lint(tokens).is_empty());
}

#[test]
fn missing_space_after_word_operator_is_flagged() {
    let tokens = condition_tokens(vec![
        token(TokenType::StringIdentifier("$a".to_string()), 2, 1),
        keyword("and", 2, 4),
        token(TokenType::StringIdentifier("$b".to_string()), 2, 7),
    ]);
    let findings = lint(tokens);

    assert_eq!(findings.len(), 1);
    assert!(findings[0].message.contains("'and'"));
}

#[test]
fn extra_spaces_around_word_operator_are_flagged_once() {
    let tokens = condition_tokens(vec![
        token(TokenType::StringIdentifier("$a".to_string()), 2, 1),
        keyword("and", 2, 5),
        token(TokenType::StringIdentifier("$b".to_string()), 2, 10),
    ]);

    assert_eq!(lint(tokens).len(), 1);
}

#[test]
fn line_break_around_operator_is_flagged() {
    let tokens = condition_tokens(vec![
        token(TokenType::StringIdentifier("$a".to_string()), 2, 1),
        keyword("and", 3, 1),
        token(TokenType::StringIdentifier("$b".to_string()), 3, 5),
    ]);

    assert_eq!(lint(tokens).len(), 1);
}

#[test]
fn skipped_text_between_operator_and_operand_is_flagged() {
    let tokens = condition_tokens(vec![
        token(TokenType::StringIdentifier("$a".to_string()), 2, 1),
        keyword("and", 2, 4),
        token(TokenType::StringIdentifier("$b".to_string()), 2, 20),
    ]);

    assert_eq!(lint(tokens).len(), 1);
}

#[test]
fn missing_space_before_symbol_operator_is_flagged() {
    let tokens = condition_tokens(vec![
        token(TokenType::Identifier("filesize".to_string()), 2, 1),
        token(TokenType::GEThan, 2, 9),
        token(TokenType::Number("10".to_string()), 2, 12),
    ]);
    let findings = lint(tokens);

    assert_eq!(findings.len(), 1);
    assert!(findings[0].message.contains("'>='"));
}

#[test]
fn missing_space_after_not_is_flagged() {
    let tokens = condition_tokens(vec![
        keyword("not", 2, 1),
        token(TokenType::StringIdentifier("$a".to_string()), 2, 4),
    ]);
    let findings = lint(tokens);

    assert_eq!(findings.len(), 1);
    assert!(findings[0].message.contains("'not'"));
}

#[test]
fn all_supported_word_operators_are_checked() {
    let binary = [
        "and",
        "or",
        "at",
        "in",
        "of",
        "contains",
        "icontains",
        "startswith",
        "istartswith",
        "endswith",
        "iendswith",
        "iequals",
        "matches",
    ];

    for operator in binary {
        let tokens = condition_tokens(vec![
            token(TokenType::Identifier("a".to_string()), 2, 1),
            keyword(operator, 2, 3),
            token(
                TokenType::Identifier("b".to_string()),
                2,
                operator.len() + 5,
            ),
        ]);
        assert_eq!(lint(tokens).len(), 1, "{operator}");
    }

    for operator in ["not", "defined"] {
        let tokens = condition_tokens(vec![
            keyword(operator, 2, 1),
            token(
                TokenType::Identifier("a".to_string()),
                2,
                operator.len() + 3,
            ),
        ]);
        assert_eq!(lint(tokens).len(), 1, "{operator}");
    }
}

#[test]
fn all_supported_symbol_operators_are_checked() {
    let operators = [
        TokenType::EqualsEquals,
        TokenType::NotEquals,
        TokenType::GThan,
        TokenType::GEThan,
        TokenType::LThan,
        TokenType::LEThan,
    ];

    for operator in operators {
        let width = match operator {
            TokenType::EqualsEquals
            | TokenType::NotEquals
            | TokenType::GEThan
            | TokenType::LEThan => 2,
            _ => 1,
        };
        let tokens = condition_tokens(vec![
            token(TokenType::Identifier("a".to_string()), 2, 1),
            token(operator, 2, 3),
            token(TokenType::Identifier("b".to_string()), 2, width + 5),
        ]);
        assert_eq!(lint(tokens).len(), 1);
    }
}

#[test]
fn operators_outside_condition_are_ignored() {
    let tokens = vec![
        keyword("meta", 1, 1),
        token(TokenType::Colon, 1, 5),
        token(TokenType::Identifier("a".to_string()), 1, 7),
        keyword("and", 1, 9),
        token(TokenType::Identifier("b".to_string()), 1, 12),
        keyword("condition", 2, 1),
        token(TokenType::Colon, 2, 10),
        token(TokenType::StringIdentifier("$a".to_string()), 3, 1),
        keyword("and", 3, 4),
        token(TokenType::StringIdentifier("$b".to_string()), 3, 8),
        token(TokenType::RBrace, 4, 1),
    ];

    assert!(lint(tokens).is_empty());
}

#[test]
fn multiple_offenses_produce_multiple_findings() {
    let tokens = condition_tokens(vec![
        token(TokenType::StringIdentifier("$a".to_string()), 2, 1),
        keyword("and", 2, 4),
        token(TokenType::StringIdentifier("$b".to_string()), 2, 7),
        keyword("or", 2, 10),
        token(TokenType::StringIdentifier("$c".to_string()), 2, 14),
    ]);

    assert_eq!(lint(tokens).len(), 2);
}

#[test]
fn finding_uses_style_category_and_warning_severity() {
    let tokens = condition_tokens(vec![
        token(TokenType::StringIdentifier("$a".to_string()), 2, 1),
        keyword("or", 2, 4),
        token(TokenType::StringIdentifier("$b".to_string()), 2, 6),
    ]);
    let findings = lint(tokens);

    assert_eq!(findings[0].rule, "ConditionSpacing");
    assert_eq!(findings[0].category, Category::Style);
    assert_eq!(findings[0].severity, Severity::Warning);
}

#[test]
fn cop_has_expected_qualified_name() {
    assert_eq!(
        StyleConditionSpacing::new(Vec::new()).qualified_name(),
        "Style/ConditionSpacing"
    );
}

#[test]
fn token_aware_default_engine_reports_condition_spacing() {
    let tokens = condition_tokens(vec![
        token(TokenType::Identifier("filesize".to_string()), 2, 1),
        token(TokenType::GEThan, 2, 9),
        token(TokenType::Number("10".to_string()), 2, 12),
    ]);
    let file = rule_file();
    let context = LintContext { file: &file };
    let findings = default_engine_with_tokens(tokens).run(&context);

    assert!(
        findings
            .iter()
            .any(|finding| finding.rule == "ConditionSpacing")
    );
}
