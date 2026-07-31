use yarlint::{
    linter::{
        context::LintContext,
        cop::Category,
        cops::style::rule_name_case::{NameCase, StyleRuleNameCase},
        engine::LintEngine,
        finding::Severity,
    },
    parser::syntax::{ConditionNode, ExprNode, RuleNode, rule_file::RuleFileNode},
};

fn make_rule_with_name(name: String) -> RuleFileNode {
    RuleFileNode {
        imports: vec![],
        rules: vec![RuleNode {
            name,
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

fn make_file_with_names(names: Vec<String>) -> RuleFileNode {
    RuleFileNode {
        imports: vec![],
        rules: names
            .into_iter()
            .map(|name| RuleNode {
                name,
                is_global: false,
                is_private: false,
                tags: vec![],
                meta: vec![],
                strings: vec![],
                condition: ConditionNode {
                    expression: ExprNode::AllOfThem,
                },
            })
            .collect(),
    }
}

fn make_engine_with_case(case: NameCase) -> LintEngine {
    let mut engine = LintEngine::new();
    engine.register(StyleRuleNameCase::new(case));
    engine
}

#[test]
fn pascal_case_conforming_name_produces_no_findings() {
    let file = make_rule_with_name("DetectMalware".to_string());
    let context = LintContext { file: &file };
    let engine = make_engine_with_case(NameCase::PascalCase);

    assert!(engine.run(&context).is_empty());
}

#[test]
fn pascal_case_lowercase_start_is_flagged() {
    let file = make_rule_with_name("detectMalware".to_string());
    let context = LintContext { file: &file };
    let engine = make_engine_with_case(NameCase::PascalCase);

    assert_eq!(engine.run(&context).len(), 1);
}

#[test]
fn pascal_case_with_underscore_is_flagged() {
    let file = make_rule_with_name("Detect_Malware".to_string());
    let context = LintContext { file: &file };
    let engine = make_engine_with_case(NameCase::PascalCase);

    assert_eq!(engine.run(&context).len(), 1);
}

#[test]
fn pascal_case_with_valid_version_suffix_is_not_flagged() {
    let file = make_rule_with_name("DetectMalware_v2".to_string());
    let context = LintContext { file: &file };
    let engine = make_engine_with_case(NameCase::PascalCase);

    assert!(engine.run(&context).is_empty());
}

#[test]
fn pascal_case_with_non_numeric_suffix_after_v_is_still_flagged() {
    // "_variant" starts with "_v" but isn't a version suffix -- the
    // digit-only guard in strip_version_suffix should reject it.
    let file = make_rule_with_name("DetectMalware_variant".to_string());
    let context = LintContext { file: &file };
    let engine = make_engine_with_case(NameCase::PascalCase);

    assert_eq!(engine.run(&context).len(), 1);
}

#[test]
fn empty_rule_name_is_flagged() {
    let file = make_rule_with_name(String::new());
    let context = LintContext { file: &file };
    let engine = make_engine_with_case(NameCase::PascalCase);

    assert_eq!(engine.run(&context).len(), 1);
}

#[test]
fn snake_case_conforming_name_produces_no_findings() {
    let file = make_rule_with_name("detect_malware".to_string());
    let context = LintContext { file: &file };
    let engine = make_engine_with_case(NameCase::SnakeCase);

    assert!(engine.run(&context).is_empty());
}

#[test]
fn snake_case_with_uppercase_is_flagged() {
    let file = make_rule_with_name("Detect_Malware".to_string());
    let context = LintContext { file: &file };
    let engine = make_engine_with_case(NameCase::SnakeCase);

    assert_eq!(engine.run(&context).len(), 1);
}

#[test]
fn snake_case_with_numeric_suffix_is_not_flagged() {
    // Unlike PascalCase, snake_case needs no special-case suffix
    // stripping -- a trailing "_2" is already valid under
    // is_snake_case's normal underscore-digit continuation rule.
    let file = make_rule_with_name("detect_malware_2".to_string());
    let context = LintContext { file: &file };
    let engine = make_engine_with_case(NameCase::SnakeCase);

    assert!(engine.run(&context).is_empty());
}

#[test]
fn multiple_non_conforming_rules_each_produce_their_own_finding() {
    let file = make_file_with_names(vec![
        "GoodName".to_string(),
        "bad_name".to_string(),
        "AlsoBad_".to_string(),
        "AnotherGood".to_string(),
    ]);
    let context = LintContext { file: &file };
    let engine = make_engine_with_case(NameCase::PascalCase);

    assert_eq!(engine.run(&context).len(), 2);
}

#[test]
fn finding_reports_style_category_cop_name_and_warning_severity() {
    let file = make_rule_with_name("bad_name".to_string());
    let context = LintContext { file: &file };
    let engine = make_engine_with_case(NameCase::PascalCase);

    let findings = engine.run(&context);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule, "RuleNameCase");
    assert_eq!(findings[0].category, Category::Style);
    assert_eq!(findings[0].severity, Severity::Warning);
    assert!(findings[0].message.contains("bad_name"));
}
