use yarlint::linter::{
    context::LintContext, cops::lint::duplicate_string::LintDuplicateString, finding::Severity,
    rule::Rule,
};
use yarlint::parser::syntax::{
    condition::ConditionNode,
    expr::ExprNode,
    rule::RuleNode,
    rule_file::RuleFileNode,
    strings::{StringModifier, StringNode},
};

#[test]
fn unique_strings_produce_no_findings() {
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
                    value: "foo".to_string(),
                    modifiers: vec![],
                },
                StringNode {
                    identifier: "$s2".to_string(),
                    value: "bar".to_string(),
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

    LintDuplicateString.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn identical_value_and_modifiers_produces_warning() {
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
                    value: "foo".to_string(),
                    modifiers: vec![],
                },
                StringNode {
                    identifier: "$s2".to_string(),
                    value: "foo".to_string(),
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

    LintDuplicateString.check(&context, &mut findings);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].severity, Severity::Warning);
    assert!(findings[0].message.contains("$s2"));
    assert!(findings[0].message.contains("$s1"));
}

#[test]
fn same_value_different_modifiers_produces_no_findings() {
    // "foo" wide and "foo" ascii are distinct strings in YARA
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
                    value: "foo".to_string(),
                    modifiers: vec![StringModifier::Wide],
                },
                StringNode {
                    identifier: "$s2".to_string(),
                    value: "foo".to_string(),
                    modifiers: vec![StringModifier::Ascii],
                },
            ],
            condition: ConditionNode {
                expression: ExprNode::AllOfThem,
            },
        }],
    };
    let context = LintContext { file: &file };
    let mut findings = vec![];

    LintDuplicateString.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn same_value_same_modifiers_different_order_produces_warning() {
    // modifier order shouldn't matter - wide ascii == ascii wide
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
                    value: "foo".to_string(),
                    modifiers: vec![StringModifier::Wide, StringModifier::Ascii],
                },
                StringNode {
                    identifier: "$s2".to_string(),
                    value: "foo".to_string(),
                    modifiers: vec![StringModifier::Ascii, StringModifier::Wide],
                },
            ],
            condition: ConditionNode {
                expression: ExprNode::AllOfThem,
            },
        }],
    };
    let context = LintContext { file: &file };
    let mut findings = vec![];

    LintDuplicateString.check(&context, &mut findings);

    assert_eq!(findings.len(), 1);
}

#[test]
fn duplicates_across_different_rules_produce_no_findings() {
    // duplicate detection is scoped per rule, not across the file
    let file = RuleFileNode {
        imports: vec![],
        rules: vec![
            RuleNode {
                name: "rule_one".to_string(),
                is_global: false,
                is_private: false,
                tags: vec![],
                meta: vec![],
                strings: vec![StringNode {
                    identifier: "$s1".to_string(),
                    value: "foo".to_string(),
                    modifiers: vec![],
                }],
                condition: ConditionNode {
                    expression: ExprNode::AllOfThem,
                },
            },
            RuleNode {
                name: "rule_two".to_string(),
                is_global: false,
                is_private: false,
                tags: vec![],
                meta: vec![],
                strings: vec![StringNode {
                    identifier: "$s1".to_string(),
                    value: "foo".to_string(),
                    modifiers: vec![],
                }],
                condition: ConditionNode {
                    expression: ExprNode::AllOfThem,
                },
            },
        ],
    };
    let context = LintContext { file: &file };
    let mut findings = vec![];

    LintDuplicateString.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn three_identical_strings_produce_two_findings() {
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
                    value: "foo".to_string(),
                    modifiers: vec![],
                },
                StringNode {
                    identifier: "$s2".to_string(),
                    value: "foo".to_string(),
                    modifiers: vec![],
                },
                StringNode {
                    identifier: "$s3".to_string(),
                    value: "foo".to_string(),
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

    LintDuplicateString.check(&context, &mut findings);

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

    LintDuplicateString.check(&context, &mut findings);

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

    LintDuplicateString.check(&context, &mut findings);

    assert!(findings.is_empty());
}
