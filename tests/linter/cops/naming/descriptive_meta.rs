use yarlint::{
    linter::{
        context::LintContext, rule::Rule, rules::naming::descriptive_meta::NamingDescriptiveMeta,
    },
    parser::syntax::{
        ConditionNode, ExprNode, MetaEntryNode, MetaValue, RuleNode, rule_file::RuleFileNode,
    },
};

fn make_file_with_meta_description(description: MetaValue) -> RuleFileNode {
    RuleFileNode {
        imports: vec![],
        rules: vec![RuleNode {
            name: "RuleName".to_string(),
            is_global: false,
            is_private: false,
            tags: vec![],
            meta: vec![MetaEntryNode {
                key: "description".to_string(),
                value: description,
            }],
            strings: vec![],
            condition: ConditionNode {
                expression: ExprNode::AllOfThem,
            },
        }],
    }
}

#[test]
fn good_description_creates_no_warnings() {
    let file = make_file_with_meta_description(MetaValue::String(
        "This is an amazing string with no issues".to_string(),
    ));

    let context = LintContext { file: &file };

    let mut findings = vec![];

    NamingDescriptiveMeta.check(&context, &mut findings);

    assert!(findings.is_empty())
}

#[test]
fn too_few_characters_creates_warning() {
    let file = make_file_with_meta_description(MetaValue::String(
        "a a a a a a".to_string(),
    ));

    let context = LintContext { file: &file };

    let mut findings = vec![];

    NamingDescriptiveMeta.check(&context, &mut findings);

    assert!(!findings.is_empty());
    assert_eq!(findings.len(), 1);
}

#[test]
fn too_few_words_creates_warning() {
    let file = make_file_with_meta_description(MetaValue::String(
        "testtesttesttesttesttesttest".to_string(),
    ));

    let context = LintContext { file: &file };

    let mut findings = vec![];

    NamingDescriptiveMeta.check(&context, &mut findings);

    assert!(!findings.is_empty());
    assert_eq!(findings.len(), 1);
}

#[test]
fn placeholder_match_creates_warning() {
    let file = make_file_with_meta_description(MetaValue::String(
        "foo".to_string(),
    ));

    let context = LintContext { file: &file };

    let mut findings = vec![];

    NamingDescriptiveMeta.check(&context, &mut findings);

    assert!(!findings.is_empty());
    assert_eq!(findings.len(), 3);
}

#[test]
fn multiple_issues_creates_multiple_warnings() {
    let file = make_file_with_meta_description(MetaValue::String(
        "Malware".to_string(),
    ));

    let context = LintContext { file: &file };

    let mut findings = vec![];

    NamingDescriptiveMeta.check(&context, &mut findings);

    assert!(!findings.is_empty());
    assert_eq!(findings.len(), 3);
}

#[test]
fn number_meta_value_is_skipped() {
    let file = make_file_with_meta_description(MetaValue::Number("1".to_string()));

    let context = LintContext { file: &file };

    let mut findings = vec![];

    NamingDescriptiveMeta.check(&context, &mut findings);

    assert!(findings.is_empty())
}

#[test]
fn boolean_meta_value_is_skipped() {
    let file = make_file_with_meta_description(MetaValue::Boolean(true));

    let context = LintContext { file: &file };

    let mut findings = vec![];

    NamingDescriptiveMeta.check(&context, &mut findings);

    assert!(findings.is_empty())
}
