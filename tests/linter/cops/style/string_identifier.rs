use yarlint::linter::{
    context::LintContext, finding::Severity, rule::Rule,
    rules::style::string_identifier::StyleStringIdentifier,
};
use yarlint::parser::syntax::strings::StringType;
use yarlint::parser::syntax::{
    condition::ConditionNode, expr::ExprNode, rule::RuleNode, rule_file::RuleFileNode,
    strings::StringNode,
};

fn make_rule_with_string(identifier: &str) -> RuleFileNode {
    RuleFileNode {
        imports: vec![],
        rules: vec![RuleNode {
            name: "test_rule".to_string(),
            is_global: false,
            is_private: false,
            tags: vec![],
            meta: vec![],
            strings: vec![StringNode {
                identifier: identifier.to_string(),
                value: StringType::Text("foo".to_string()),
                modifiers: vec![],
            }],
            condition: ConditionNode {
                expression: ExprNode::AllOfThem,
            },
        }],
    }
}

#[test]
fn snake_case_identifier_produces_no_findings() {
    let file = make_rule_with_string("$foo_bar");
    let context = LintContext { file: &file };
    let mut findings = vec![];

    StyleStringIdentifier.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn camel_case_identifier_produces_warning() {
    let file = make_rule_with_string("$fooBar");
    let context = LintContext { file: &file };
    let mut findings = vec![];

    StyleStringIdentifier.check(&context, &mut findings);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].severity, Severity::Warning);
    assert!(findings[0].message.contains("$fooBar"));
}

#[test]
fn multiple_bad_identifiers_produce_one_finding_each() {
    // each bad identifier should produce its own finding independently
    let file = RuleFileNode {
        imports: vec![],
        rules: vec![RuleNode {
            name: "test_rule".to_string(),
            is_global: false,
            is_private: false,
            tags: vec![],
            meta: vec![],
            strings: vec![
                StringNode {
                    identifier: "$fooBar".to_string(),
                    value: StringType::Text("a".to_string()),
                    modifiers: vec![],
                },
                StringNode {
                    identifier: "$BazQux".to_string(),
                    value: StringType::Text("b".to_string()),
                    modifiers: vec![],
                },
            ],
            condition: ConditionNode {
                expression: ExprNode::AllOfThem,
            },
        }],
    };
    let context = LintContext { file: &file };
    let mut findings = vec![];

    StyleStringIdentifier.check(&context, &mut findings);

    assert_eq!(findings.len(), 2);
}

#[test]
fn rule_with_no_strings_produces_no_findings() {
    let file = RuleFileNode {
        imports: vec![],
        rules: vec![RuleNode {
            name: "test_rule".to_string(),
            is_global: false,
            is_private: false,
            tags: vec![],
            meta: vec![],
            strings: vec![],
            condition: ConditionNode {
                expression: ExprNode::AllOfThem,
            },
        }],
    };
    let context = LintContext { file: &file };
    let mut findings = vec![];

    StyleStringIdentifier.check(&context, &mut findings);

    assert!(findings.is_empty());
}
