use yarlint::parser::{
    ast_parser::AstParser,
    lexer::yara::tokenize,
    syntax::{expr::ExprNode, rule_file::RuleFileNode, strings::StringType},
};

fn parse(source: &str) -> Result<RuleFileNode, String> {
    let tokens = tokenize(source).map_err(|e| e.to_string())?;
    AstParser::parse_rule_file(AstParser::new(tokens))
}

// --- empty input ---

#[test]
fn empty_input_returns_empty_rule_file() {
    let result = parse("").unwrap();

    assert!(result.rules.is_empty());
    assert!(result.imports.is_empty());
}

// --- imports ---

#[test]
fn single_import_is_parsed() {
    let result = parse(r#"import "pe""#).unwrap();

    assert_eq!(result.imports, vec!["pe".to_string()]);
}

#[test]
fn multiple_imports_are_parsed() {
    let result = parse(r#"import "pe" import "math""#).unwrap();

    assert_eq!(result.imports, vec!["pe".to_string(), "math".to_string()]);
}

// --- minimal rule ---

#[test]
fn minimal_rule_parses_successfully() {
    let result = parse("rule foo { condition: all of them }");

    assert!(result.is_ok());
}

#[test]
fn rule_name_is_parsed_correctly() {
    let result = parse("rule my_rule { condition: all of them }").unwrap();

    assert_eq!(result.rules[0].name, "my_rule");
}

#[test]
fn rule_name_with_dash_is_parsed_correctly() {
    // dashes are valid in YARA rule names even though they trigger NamingRuleName
    let result = parse("rule malware-one { condition: all of them }").unwrap();

    assert_eq!(result.rules[0].name, "malware-one");
}

#[test]
fn rule_defaults_to_not_global() {
    let result = parse("rule foo { condition: all of them }").unwrap();

    assert!(!result.rules[0].is_global);
}

#[test]
fn rule_defaults_to_not_private() {
    let result = parse("rule foo { condition: all of them }").unwrap();

    assert!(!result.rules[0].is_private);
}

// --- multiple rules ---

#[test]
fn multiple_rules_are_all_parsed() {
    let source = r#"
        rule foo { condition: all of them }
        rule bar { condition: all of them }
    "#;
    let result = parse(source).unwrap();

    assert_eq!(result.rules.len(), 2);
}

#[test]
fn multiple_rules_have_correct_names() {
    let source = r#"
        rule foo { condition: all of them }
        rule bar { condition: all of them }
    "#;
    let result = parse(source).unwrap();

    assert_eq!(result.rules[0].name, "foo");
    assert_eq!(result.rules[1].name, "bar");
}

// --- meta section ---

#[test]
fn rule_with_meta_string_is_parsed() {
    let source = r#"
        rule foo {
            meta:
                author = "DeTraced Security"
            strings:
                $s1 = "foo"
            condition:
                all of them
        }
    "#;
    let result = parse(source).unwrap();

    assert_eq!(result.rules[0].meta[0].key, "author");
}

#[test]
fn rule_with_multiple_meta_entries_are_all_parsed() {
    let source = r#"
        rule foo {
            meta:
                author = "DeTraced Security"
                version = "1.0"
            strings:
                $s1 = "foo"
            condition:
                all of them
        }
    "#;
    let result = parse(source).unwrap();

    assert_eq!(result.rules[0].meta.len(), 2);
}

// --- strings section ---

#[test]
fn rule_with_string_is_parsed() {
    let source = r#"
        rule foo {
            strings:
                $s1 = "cmd.exe"
            condition:
                $s1
        }
    "#;
    let result = parse(source).unwrap();

    assert_eq!(result.rules[0].strings[0].identifier, "$s1");
    assert_eq!(
        result.rules[0].strings[0].value,
        StringType::Text("cmd.exe".to_owned())
    );
}

#[test]
fn rule_with_multiple_strings_are_all_parsed() {
    let source = r#"
        rule foo {
            strings:
                $s1 = "cmd.exe"
                $s2 = "powershell"
            condition:
                all of them
        }
    "#;
    let result = parse(source).unwrap();

    assert_eq!(result.rules[0].strings.len(), 2);
}

#[test]
fn rule_with_wide_modifier_is_parsed() {
    use yarlint::parser::syntax::strings::StringModifier;

    let source = r#"
        rule foo {
            strings:
                $s1 = "cmd.exe" wide
            condition:
                $s1
        }
    "#;
    let result = parse(source).unwrap();

    assert!(
        result.rules[0].strings[0]
            .modifiers
            .contains(&StringModifier::Wide)
    );
}

// --- condition section ---

#[test]
fn condition_all_of_them_is_parsed() {
    let source = r#"
        rule foo {
            strings:
                $s1 = "foo"
            condition:
                all of them
        }
    "#;
    let result = parse(source).unwrap();

    assert!(matches!(
        result.rules[0].condition.expression,
        ExprNode::AllOfThem
    ));
}

#[test]
fn parse_rule_without_condition_uses_default_condition() {
    // condition block is optional per the parser logic
    let tokens = tokenize("rule foo { }").unwrap();
    let result = AstParser::parse_rule_file(AstParser::new(tokens));
    assert!(result.is_ok());
}

#[test]
fn empty_condition_is_parsed() {
    // condition block without any contents is parsed
    let tokens = tokenize("rule foo { condition: }").unwrap();
    let result = AstParser::parse_rule_file(AstParser::new(tokens));
    assert!(result.is_ok());
}

// --- error cases ---

#[test]
fn missing_closing_brace_returns_err() {
    let result = parse("rule foo { condition: all of them");

    assert!(result.is_err());
}

#[test]
fn missing_rule_body_returns_err() {
    let result = parse("rule foo");

    assert!(result.is_err());
}

// --- imports with rules ---

#[test]
fn import_followed_by_rule_is_parsed() {
    let source = r#"
        import "pe"
        rule foo { condition: all of them }
    "#;
    let result = parse(source).unwrap();

    assert_eq!(result.imports.len(), 1);
    assert_eq!(result.rules.len(), 1);
}
