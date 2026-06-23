use yarlint::linter::{
    context::LintContext, finding::Severity, rule::Rule, rules::naming::rule_name::NamingRuleName,
};
use yarlint::parser::syntax::{
    condition::ConditionNode, expr::ExprNode, rule::RuleNode, rule_file::RuleFileNode,
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
fn rule_name_without_dash_produces_no_findings() {
    let file = make_file_with_rule_name("good_rule_name");
    let context = LintContext { file: &file };
    let mut findings = vec![];

    NamingRuleName.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn rule_name_with_dash_produces_warning() {
    let file = make_file_with_rule_name("bad-rule-name");
    let context = LintContext { file: &file };
    let mut findings = vec![];

    NamingRuleName.check(&context, &mut findings);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].severity, Severity::Warning);
    assert!(findings[0].message.contains("bad-rule-name"));
}

#[test]
fn rule_name_with_single_dash_produces_warning() {
    let file = make_file_with_rule_name("bad-rule");
    let context = LintContext { file: &file };
    let mut findings = vec![];

    NamingRuleName.check(&context, &mut findings);

    assert_eq!(findings.len(), 1);
}

#[test]
fn rule_name_with_leading_dash_produces_warning() {
    let file = make_file_with_rule_name("-bad_rule");
    let context = LintContext { file: &file };
    let mut findings = vec![];

    NamingRuleName.check(&context, &mut findings);

    assert_eq!(findings.len(), 1);
}
