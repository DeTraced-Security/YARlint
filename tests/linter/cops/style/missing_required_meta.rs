use yarlint::linter::{
    context::LintContext, cops::style::missing_required_meta::StyleMissingRequiredMeta,
    finding::Severity, rule::Rule,
};
use yarlint::parser::syntax::{
    condition::ConditionNode, expr::ExprNode, meta::MetaEntryNode, meta::MetaValue, rule::RuleNode,
    rule_file::RuleFileNode,
};

fn meta_entry(key: &str) -> MetaEntryNode {
    MetaEntryNode {
        key: key.to_string(),
        value: MetaValue::String("placeholder".to_string()),
    }
}

fn make_rule_with_meta(keys: Vec<&str>) -> RuleFileNode {
    RuleFileNode {
        imports: vec![],
        rules: vec![RuleNode {
            name: "test_rule".to_string(),
            is_global: false,
            is_private: false,
            tags: vec![],
            meta: keys.into_iter().map(meta_entry).collect(),
            strings: vec![],
            condition: ConditionNode {
                expression: ExprNode::AllOfThem,
            },
        }],
    }
}

#[test]
fn rule_with_all_keys_produces_no_findings() {
    let file = make_rule_with_meta(vec!["author", "description", "reference", "date"]);
    let context = LintContext { file: &file };
    let mut findings = vec![];

    StyleMissingRequiredMeta.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn rule_with_one_missing_key_produces_one_warning() {
    // missing "date"
    let file = make_rule_with_meta(vec!["author", "description", "reference"]);
    let context = LintContext { file: &file };
    let mut findings = vec![];

    StyleMissingRequiredMeta.check(&context, &mut findings);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].severity, Severity::Info);
    assert!(findings[0].message.contains("date"));
}

#[test]
fn rule_with_multiple_missing_keys_produces_multiple_warnings() {
    // missing "reference" and "date"
    let file = make_rule_with_meta(vec!["author", "description"]);
    let context = LintContext { file: &file };
    let mut findings = vec![];

    StyleMissingRequiredMeta.check(&context, &mut findings);

    assert_eq!(findings.len(), 2);
    let messages: Vec<&str> = findings.iter().map(|f| f.message.as_str()).collect();
    assert!(messages.iter().any(|m| m.contains("reference")));
    assert!(messages.iter().any(|m| m.contains("date")));
}

#[test]
fn rule_with_no_keys_produces_warnings() {
    let file = make_rule_with_meta(vec![]);
    let context = LintContext { file: &file };
    let mut findings = vec![];

    StyleMissingRequiredMeta.check(&context, &mut findings);

    // every required key is missing
    assert_eq!(findings.len(), 4);
}

#[test]
fn rule_with_extra_keys_produces_no_warning() {
    // unexpected keys present alongside all required ones should not affect the result
    let file = make_rule_with_meta(vec![
        "author",
        "description",
        "reference",
        "date",
        "hash",
        "family",
    ]);
    let context = LintContext { file: &file };
    let mut findings = vec![];

    StyleMissingRequiredMeta.check(&context, &mut findings);

    assert!(findings.is_empty());
}
