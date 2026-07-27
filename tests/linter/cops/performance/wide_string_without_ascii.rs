use yarlint::{
    linter::{
        context::LintContext, rule::Rule,
        rules::performance::wide_string_without_ascii::PerformanceWideStringWithoutAscii,
    },
    parser::syntax::{
        ConditionNode, ExprNode, RuleNode, StringModifier, StringNode,
        hex::{HexExprNode, HexNode},
        rule_file::RuleFileNode,
        strings::StringType,
    },
};

fn make_file_with_string(strings: Vec<StringNode>) -> RuleFileNode {
    RuleFileNode {
        imports: vec![],
        rules: vec![RuleNode {
            name: "RuleName".to_string(),
            is_global: false,
            is_private: false,
            tags: vec![],
            meta: vec![],
            strings,
            condition: ConditionNode {
                expression: ExprNode::BoolLiteral(true),
            },
        }],
    }
}

#[test]
fn text_string_with_wide_and_no_ascii_produces_finding() {
    let strings: Vec<StringNode> = vec![StringNode {
        identifier: "$s1".to_owned(),
        value: StringType::Text("Test text".to_string()),
        modifiers: vec![StringModifier::Wide],
    }];
    let file = make_file_with_string(strings);
    let context = LintContext { file: &file };
    let mut findings = vec![];

    PerformanceWideStringWithoutAscii.check(&context, &mut findings);

    assert!(!findings.is_empty());
    assert_eq!(findings.len(), 1);
}

#[test]
fn text_string_with_wide_and_ascii_produces_no_finding() {
    let strings: Vec<StringNode> = vec![StringNode {
        identifier: "$s1".to_owned(),
        value: StringType::Text("Test text".to_string()),
        modifiers: vec![StringModifier::Wide, StringModifier::Ascii],
    }];
    let file = make_file_with_string(strings);
    let context = LintContext { file: &file };
    let mut findings = vec![];

    PerformanceWideStringWithoutAscii.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn regex_string_with_wide_and_no_ascii_produces_finding() {
    let strings: Vec<StringNode> = vec![StringNode {
        identifier: "$s1".to_owned(),
        value: StringType::RegEx("regex here".to_string()),
        modifiers: vec![StringModifier::Wide],
    }];
    let file = make_file_with_string(strings);
    let context = LintContext { file: &file };
    let mut findings = vec![];

    PerformanceWideStringWithoutAscii.check(&context, &mut findings);

    assert!(!findings.is_empty());
    assert_eq!(findings.len(), 1);
}

#[test]
fn regex_string_with_wide_and_ascii_produces_no_finding() {
    let strings: Vec<StringNode> = vec![StringNode {
        identifier: "$s1".to_owned(),
        value: StringType::RegEx("regex here".to_string()),
        modifiers: vec![StringModifier::Wide, StringModifier::Ascii],
    }];
    let file = make_file_with_string(strings);
    let context = LintContext { file: &file };
    let mut findings = vec![];

    PerformanceWideStringWithoutAscii.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn hex_string_with_wide_and_no_ascii_produces_no_finding() {
    let strings: Vec<StringNode> = vec![StringNode {
        identifier: "$s1".to_owned(),
        value: StringType::Hex(HexNode {
            expression: HexExprNode { atoms: vec![] },
            original_string: "hex string".to_string(),
        }),
        modifiers: vec![StringModifier::Wide],
    }];
    let file = make_file_with_string(strings);
    let context = LintContext { file: &file };
    let mut findings = vec![];

    PerformanceWideStringWithoutAscii.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn hex_string_with_wide_and_ascii_produces_no_finding() {
    let strings: Vec<StringNode> = vec![StringNode {
        identifier: "$s1".to_owned(),
        value: StringType::Hex(HexNode {
            expression: HexExprNode { atoms: vec![] },
            original_string: "hex string".to_string(),
        }),
        modifiers: vec![StringModifier::Wide, StringModifier::Ascii],
    }];
    let file = make_file_with_string(strings);
    let context = LintContext { file: &file };
    let mut findings = vec![];

    PerformanceWideStringWithoutAscii.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn text_string_with_multiple_wide_and_no_ascii_produces_multiple_findings() {
    let strings: Vec<StringNode> = vec![
        StringNode {
            identifier: "$s1".to_owned(),
            value: StringType::Text("Test text".to_string()),
            modifiers: vec![StringModifier::Wide],
        },
        StringNode {
            identifier: "$s2".to_owned(),
            value: StringType::Text("More test text".to_string()),
            modifiers: vec![StringModifier::Wide],
        },
    ];
    let file = make_file_with_string(strings);
    let context = LintContext { file: &file };
    let mut findings = vec![];

    PerformanceWideStringWithoutAscii.check(&context, &mut findings);

    assert!(!findings.is_empty());
    assert_eq!(findings.len(), 2);
}

#[test]
fn text_string_with_multiple_wide_and_one_no_ascii_produces_one_finding() {
    let strings: Vec<StringNode> = vec![
        StringNode {
            identifier: "$s1".to_owned(),
            value: StringType::Text("Test text".to_string()),
            modifiers: vec![StringModifier::Wide],
        },
        StringNode {
            identifier: "$s2".to_owned(),
            value: StringType::Text("More test text".to_string()),
            modifiers: vec![StringModifier::Wide, StringModifier::Ascii],
        },
    ];
    let file = make_file_with_string(strings);
    let context = LintContext { file: &file };
    let mut findings = vec![];

    PerformanceWideStringWithoutAscii.check(&context, &mut findings);

    assert!(!findings.is_empty());
    assert_eq!(findings.len(), 1);
}
