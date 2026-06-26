use yarlint::parser::{
    ast_parser::AstParser, lexer::yara::tokenize, syntax::rule_file::RuleFileNode,
};

fn parse(source: &str) -> Result<RuleFileNode, String> {
    let tokens = tokenize(source).map_err(|e| e.to_string())?;
    AstParser::parse_rule_file(AstParser::new(tokens))
}

#[test]
fn trailing_token_causes_err() {
    let source = r#"
        rule foo {
            meta:
                author = "DeTraced Security"
            strings:
                $s1 = {aa bb cc )}
            condition:
                all of them
        }
    "#;
    let result = parse(source);
    assert!(result.is_err())
}

#[test]
fn empty_hex_tokens_returns_0() {
    let source = r#"
        rule foo {
            meta:
                author = "DeTraced Security"
            strings:
                $s1 = {}
            condition:
                all of them
        }    
    "#;
    let result = parse(source);
    assert!(result.is_ok())
}