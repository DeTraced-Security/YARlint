use yarlint::linter::{
    context::LintContext, finding::Severity, rule::Rule,
    rules::style::meta_keys_order::StyleMetaKeysOrder,
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
fn rule_with_no_out_of_order_keys_produces_no_findings() {
    // default expected order is author, description, reference, date
    let file = make_rule_with_meta(vec!["author", "description", "reference", "date"]);
    let context = LintContext { file: &file };
    let mut findings = vec![];

    StyleMetaKeysOrder.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn rule_with_one_out_of_order_key_produces_one_finding() {
    // "description" appears before "author", but author is expected first
    let file = make_rule_with_meta(vec!["description", "author", "reference", "date"]);
    let context = LintContext { file: &file };
    let mut findings = vec![];

    StyleMetaKeysOrder.check(&context, &mut findings);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].severity, Severity::Info);
    assert!(findings[0].message.contains("author"));
    assert!(findings[0].message.contains("description"));
}

#[test]
fn rule_with_multiple_out_of_order_keys_produces_multiple_findings() {
    let file = make_rule_with_meta(vec!["date", "reference", "author", "description"]);
    let context = LintContext { file: &file };
    let mut findings = vec![];

    StyleMetaKeysOrder.check(&context, &mut findings);

    assert!(findings.len() > 1);
}

#[test]
fn rule_with_no_keys_produces_no_findings() {
    let file = make_rule_with_meta(vec![]);
    let context = LintContext { file: &file };
    let mut findings = vec![];

    StyleMetaKeysOrder.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn rule_with_unexpected_keys_interspersed_produces_no_finding() {
    // "hash" and "family" are not in the expected schema and should be ignored for ordering
    let file = make_rule_with_meta(vec![
        "author",
        "hash",
        "description",
        "family",
        "reference",
        "date",
    ]);
    let context = LintContext { file: &file };
    let mut findings = vec![];

    StyleMetaKeysOrder.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn rule_with_single_meta_key_produces_no_findings() {
    let file = make_rule_with_meta(vec!["description"]);
    let context = LintContext { file: &file };
    let mut findings = vec![];

    StyleMetaKeysOrder.check(&context, &mut findings);

    assert!(findings.is_empty());
}
