use yarlint::linter::{context::LintContext, default_engine};
use yarlint::parser::syntax::{
    condition::ConditionNode, expr::ExprNode, rule::RuleNode, rule_file::RuleFileNode,
    strings::StringNode,
};

#[path = "linter/cops.rs"]
pub mod cops;

#[path = "linter/engine.rs"]
pub mod engine;

#[test]
fn default_engine_has_cops_registered() {
    // an engine with no cops produces no findings regardless of input
    // so if we get findings, cops must be registered
    let file = RuleFileNode {
        imports: vec![],
        rules: vec![RuleNode {
            name: "bad-rule-name".to_string(),
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
        }],
    };
    let context = LintContext { file: &file };

    let findings = default_engine().run(&context);

    assert!(!findings.is_empty());
}

#[test]
fn default_engine_produces_no_findings_for_clean_rule() {
    let file = RuleFileNode {
        imports: vec![],
        rules: vec![RuleNode {
            name: "clean_rule".to_string(),
            is_global: false,
            is_private: false,
            tags: vec![],
            meta: vec![],
            strings: vec![StringNode {
                identifier: "$s1".to_string(),
                value: "cmd.exe".to_string(),
                modifiers: vec![],
            }],
            condition: ConditionNode {
                expression: ExprNode::AllOfThem,
            },
        }],
    };
    let context = LintContext { file: &file };

    let findings = default_engine().run(&context);

    assert!(findings.is_empty());
}
