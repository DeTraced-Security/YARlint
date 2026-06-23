use yarlint::linter::{
    context::LintContext, finding::Severity, rule::Rule,
    rules::lint::empty_strings_block::LintEmptyStringsBlock,
};
use yarlint::parser::syntax::strings::StringType;
use yarlint::parser::syntax::{
    condition::ConditionNode, expr::ExprNode, rule::RuleNode, rule_file::RuleFileNode,
    strings::StringNode,
};

#[test]
fn rule_with_strings_produces_no_findings() {
    let file = RuleFileNode {
        imports: vec![],
        rules: vec![RuleNode {
            name: "test_rule".to_string(),
            is_global: false,
            is_private: false,
            tags: vec![],
            meta: vec![],
            strings: vec![StringNode {
                identifier: "$s1".to_string(),
                value: StringType::Text("cmd.exe".to_string()),
                modifiers: vec![],
            }],
            condition: ConditionNode {
                expression: ExprNode::AllOfThem,
            },
        }],
    };
    let context = LintContext { file: &file };
    let mut findings = vec![];

    LintEmptyStringsBlock.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn rule_with_empty_strings_block_produces_warning() {
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

    LintEmptyStringsBlock.check(&context, &mut findings);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].severity, Severity::Warning);
    assert!(findings[0].message.contains("test_rule"));
}

#[test]
fn multiple_rules_with_empty_blocks_produce_one_finding_each() {
    let file = RuleFileNode {
        imports: vec![],
        rules: vec![
            RuleNode {
                name: "rule_one".to_string(),
                is_global: false,
                is_private: false,
                tags: vec![],
                meta: vec![],
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
                meta: vec![],
                strings: vec![],
                condition: ConditionNode {
                    expression: ExprNode::AllOfThem,
                },
            },
        ],
    };
    let context = LintContext { file: &file };
    let mut findings = vec![];

    LintEmptyStringsBlock.check(&context, &mut findings);

    assert_eq!(findings.len(), 2);
}

#[test]
fn mixed_rules_only_produces_findings_for_empty_blocks() {
    let file = RuleFileNode {
        imports: vec![],
        rules: vec![
            RuleNode {
                name: "good_rule".to_string(),
                is_global: false,
                is_private: false,
                tags: vec![],
                meta: vec![],
                strings: vec![StringNode {
                    identifier: "$s1".to_string(),
                    value: StringType::Text("cmd.exe".to_string()),
                    modifiers: vec![],
                }],
                condition: ConditionNode {
                    expression: ExprNode::AllOfThem,
                },
            },
            RuleNode {
                name: "bad_rule".to_string(),
                is_global: false,
                is_private: false,
                tags: vec![],
                meta: vec![],
                strings: vec![],
                condition: ConditionNode {
                    expression: ExprNode::AllOfThem,
                },
            },
        ],
    };
    let context = LintContext { file: &file };
    let mut findings = vec![];

    LintEmptyStringsBlock.check(&context, &mut findings);

    assert_eq!(findings.len(), 1);
    assert!(findings[0].message.contains("bad_rule"));
}

#[test]
fn empty_rule_list_produces_no_findings() {
    let file = RuleFileNode {
        imports: vec![],
        rules: vec![],
    };
    let context = LintContext { file: &file };
    let mut findings = vec![];

    LintEmptyStringsBlock.check(&context, &mut findings);

    assert!(findings.is_empty());
}
