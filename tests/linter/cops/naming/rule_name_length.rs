use yarlint::{
    linter::{
        context::LintContext, rule::Rule, rules::naming::rule_name_length::NamingRuleNameLength,
    },
    parser::syntax::{ConditionNode, RuleNode, expr::ExprNode, rule_file::RuleFileNode},
};

fn make_file_with_rule_name(name: &str) -> RuleFileNode {
    RuleFileNode {
        imports: vec![],
        rules: vec![RuleNode {
            name: name.to_string(),
            is_global: false,
            is_private: false,
            tags: vec![],
            meta: vec![],
            strings: vec![],
            condition: ConditionNode {
                expression: ExprNode::AllOfThem,
            },
        }],
    }
}

#[test]
fn rule_name_with_good_length_produces_no_findings() {
    let file = make_file_with_rule_name("thisIsAGoodRuleName");
    let context = LintContext { file: &file };
    let mut findings = vec![];

    NamingRuleNameLength.check(&context, &mut findings);

    assert!(findings.is_empty())
}

#[test]
fn too_long_rule_name_produces_findings() {
    let file = make_file_with_rule_name(
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    );
    let context = LintContext { file: &file };
    let mut findings = vec![];

    NamingRuleNameLength.check(&context, &mut findings);

    assert_eq!(findings.len(), 1);
    assert!(findings[0].message.contains("maximum"))
}

#[test]
fn too_short_rule_name_produces_findings() {
    let file = make_file_with_rule_name("a");
    let context = LintContext { file: &file };
    let mut findings = vec![];

    NamingRuleNameLength.check(&context, &mut findings);

    assert_eq!(findings.len(), 1);
    assert!(findings[0].message.contains("minimum"))
}
