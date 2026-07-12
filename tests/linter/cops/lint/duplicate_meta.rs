use yarlint::linter::{
    context::LintContext, rule::Rule, rules::lint::duplicate_meta::LintDuplicateMeta,
};
use yarlint::parser::syntax::MetaValue;
use yarlint::parser::syntax::{
    MetaEntryNode, condition::ConditionNode, expr::ExprNode, rule::RuleNode,
    rule_file::RuleFileNode,
};
#[test]
fn unique_keys_produce_no_findings() {
    let file = RuleFileNode {
        imports: vec![],
        rules: vec![RuleNode {
            name: "rule_one".to_string(),
            is_global: false,
            is_private: false,
            tags: vec![],
            meta: vec![
                MetaEntryNode {
                    key: "date".to_string(),
                    value: MetaValue::String("01-01-1970".to_string()),
                },
                MetaEntryNode {
                    key: "author".to_string(),
                    value: MetaValue::String("DeTraced Security".to_string()),
                },
            ],
            strings: vec![],
            condition: ConditionNode {
                expression: ExprNode::AllOfThem,
            },
        }],
    };
    let context = LintContext { file: &file };
    let mut findings = vec![];

    LintDuplicateMeta.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn same_keys_produce_findings() {
    let file = RuleFileNode {
        imports: vec![],
        rules: vec![RuleNode {
            name: "rule_one".to_string(),
            is_global: false,
            is_private: false,
            tags: vec![],
            meta: vec![
                MetaEntryNode {
                    key: "date".to_string(),
                    value: MetaValue::String("01-01-1970".to_string()),
                },
                MetaEntryNode {
                    key: "date".to_string(),
                    value: MetaValue::String("10-10-0791".to_string()),
                },
            ],
            strings: vec![],
            condition: ConditionNode {
                expression: ExprNode::AllOfThem,
            },
        }],
    };
    let context = LintContext { file: &file };
    let mut findings = vec![];

    LintDuplicateMeta.check(&context, &mut findings);

    assert!(!findings.is_empty());
    assert_eq!(findings.len(), 1)
}

#[test]
fn three_keys_produce_two_findings() {
    let file = RuleFileNode {
        imports: vec![],
        rules: vec![RuleNode {
            name: "rule_one".to_string(),
            is_global: false,
            is_private: false,
            tags: vec![],
            meta: vec![
                MetaEntryNode {
                    key: "date".to_string(),
                    value: MetaValue::String("01-01-1970".to_string()),
                },
                MetaEntryNode {
                    key: "date".to_string(),
                    value: MetaValue::String("10-10-0791".to_string()),
                },
                MetaEntryNode {
                    key: "date".to_string(),
                    value: MetaValue::String("this literally isn't a date lol".to_string()),
                },
            ],
            strings: vec![],
            condition: ConditionNode {
                expression: ExprNode::AllOfThem,
            },
        }],
    };
    let context = LintContext { file: &file };
    let mut findings = vec![];

    LintDuplicateMeta.check(&context, &mut findings);

    assert!(!findings.is_empty());
    assert_eq!(findings.len(), 2)
}

#[test]
fn duplicate_keys_in_different_rules_produce_no_findings() {
    let file = RuleFileNode {
        imports: vec![],
        rules: vec![
            RuleNode {
                name: "rule_one".to_string(),
                is_global: false,
                is_private: false,
                tags: vec![],
                meta: vec![MetaEntryNode {
                    key: "date".to_string(),
                    value: MetaValue::String("01-01-1970".to_string()),
                }],
                strings: vec![],
                condition: ConditionNode {
                    expression: ExprNode::AllOfThem,
                },
            },
            RuleNode {
                name: "rule_two".to_string(),
                is_global: false,
                is_private: false,
                tags: vec![],
                meta: vec![MetaEntryNode {
                    key: "date".to_string(),
                    value: MetaValue::String("01-01-1970".to_string()),
                }],
                strings: vec![],
                condition: ConditionNode {
                    expression: ExprNode::AllOfThem,
                },
            },
        ],
    };
    let context = LintContext { file: &file };
    let mut findings = vec![];

    LintDuplicateMeta.check(&context, &mut findings);

    assert!(findings.is_empty());
}
