use yarlint::linter::{
    context::LintContext, cops::lint::empty_string::LintEmptyString, finding::Severity, rule::Rule,
};
use yarlint::parser::syntax::{
    condition::ConditionNode, expr::ExprNode, rule::RuleNode, rule_file::RuleFileNode,
    strings::StringNode,
};

fn make_file_with_string(identifier: &str, value: &str) -> RuleFileNode {
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
                value: value.to_string(),
                modifiers: vec![],
            }],
            condition: ConditionNode {
                expression: ExprNode::AllOfThem,
            },
        }],
    }
}

#[test]
fn non_empty_string_produces_no_findings() {
    let file = make_file_with_string("$s1", "cmd.exe");
    let context = LintContext { file: &file };
    let mut findings = vec![];

    LintEmptyString.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn empty_string_value_produces_warning() {
    let file = make_file_with_string("$s1", "");
    let context = LintContext { file: &file };
    let mut findings = vec![];

    LintEmptyString.check(&context, &mut findings);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].severity, Severity::Warning);
    assert!(findings[0].message.contains("$s1"));
    assert!(findings[0].message.contains("test_rule"));
}

#[test]
fn multiple_empty_strings_produce_one_finding_each() {
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
                    identifier: "$s1".to_string(),
                    value: "".to_string(),
                    modifiers: vec![],
                },
                StringNode {
                    identifier: "$s2".to_string(),
                    value: "".to_string(),
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

    LintEmptyString.check(&context, &mut findings);

    assert_eq!(findings.len(), 2);
}

#[test]
fn mixed_strings_only_produces_findings_for_empty_ones() {
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
                    identifier: "$s1".to_string(),
                    value: "cmd.exe".to_string(),
                    modifiers: vec![],
                },
                StringNode {
                    identifier: "$s2".to_string(),
                    value: "".to_string(),
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

    LintEmptyString.check(&context, &mut findings);

    assert_eq!(findings.len(), 1);
    assert!(findings[0].message.contains("$s2"));
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

    LintEmptyString.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn empty_rule_list_produces_no_findings() {
    let file = RuleFileNode {
        imports: vec![],
        rules: vec![],
    };
    let context = LintContext { file: &file };
    let mut findings = vec![];

    LintEmptyString.check(&context, &mut findings);

    assert!(findings.is_empty());
}
