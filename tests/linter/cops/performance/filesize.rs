use yarlint::{
    linter::{
        context::LintContext, cop::Cop, cops::performance::filesize::PerformanceFilesize,
    },
    parser::syntax::{
        ConditionNode, ExprNode, RuleNode, StringModifier, StringNode,
        operators::BinaryOperator,
        rule_file::RuleFileNode,
        strings::StringType,
    },
};

fn make_file(strings: Vec<StringNode>, condition: ExprNode) -> RuleFileNode {
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
                expression: condition,
            },
        }],
    }
}

fn filesize_condition() -> ExprNode {
    // filesize < 100
    ExprNode::Binary {
        left: Box::new(ExprNode::Identifier("filesize".to_string())),
        operator: BinaryOperator::LessThan,
        right: Box::new(ExprNode::Number {
            size: 100,
            unit: None,
            original: "100".to_string(),
        }),
    }
}

fn wide_string(identifier: &str) -> StringNode {
    StringNode {
        identifier: identifier.to_string(),
        value: StringType::Text("Test text".to_string()),
        modifiers: vec![StringModifier::Wide],
    }
}

fn regex_string(identifier: &str) -> StringNode {
    StringNode {
        identifier: identifier.to_string(),
        value: StringType::RegEx("regex here".to_string()),
        modifiers: vec![],
    }
}

fn plain_text_string(identifier: &str) -> StringNode {
    StringNode {
        identifier: identifier.to_string(),
        value: StringType::Text("plain".to_string()),
        modifiers: vec![],
    }
}

#[test]
fn wide_string_without_filesize_produces_finding() {
    let file = make_file(vec![wide_string("$s1")], ExprNode::BoolLiteral(true));
    let context = LintContext { file: &file };
    let mut findings = vec![];

    PerformanceFilesize.check(&context, &mut findings);

    assert_eq!(findings.len(), 1);
}

#[test]
fn regex_string_without_filesize_produces_finding() {
    let file = make_file(vec![regex_string("$s1")], ExprNode::BoolLiteral(true));
    let context = LintContext { file: &file };
    let mut findings = vec![];

    PerformanceFilesize.check(&context, &mut findings);

    assert_eq!(findings.len(), 1);
}

#[test]
fn wide_string_with_filesize_produces_no_finding() {
    let file = make_file(vec![wide_string("$s1")], filesize_condition());
    let context = LintContext { file: &file };
    let mut findings = vec![];

    PerformanceFilesize.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn regex_string_with_filesize_produces_no_finding() {
    let file = make_file(vec![regex_string("$s1")], filesize_condition());
    let context = LintContext { file: &file };
    let mut findings = vec![];

    PerformanceFilesize.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn plain_text_string_without_filesize_produces_no_finding() {
    // No heavy pattern present at all, so the cop should stay quiet
    // regardless of the missing filesize bound.
    let file = make_file(vec![plain_text_string("$s1")], ExprNode::BoolLiteral(true));
    let context = LintContext { file: &file };
    let mut findings = vec![];

    PerformanceFilesize.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn filesize_nested_in_group_is_detected() {
    // (filesize < 100 and $s1)
    let condition = ExprNode::Group(Box::new(ExprNode::Binary {
        left: Box::new(filesize_condition()),
        operator: BinaryOperator::And,
        right: Box::new(ExprNode::Identifier("$s1".to_string())),
    }));
    let file = make_file(vec![wide_string("$s1")], condition);
    let context = LintContext { file: &file };
    let mut findings = vec![];

    PerformanceFilesize.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn filesize_nested_in_function_call_arguments_is_detected() {
    // some_func(filesize)
    let condition = ExprNode::FunctionCall {
        name: "some_func".to_string(),
        arguments: vec![ExprNode::Identifier("filesize".to_string())],
    };
    let file = make_file(vec![wide_string("$s1")], condition);
    let context = LintContext { file: &file };
    let mut findings = vec![];

    PerformanceFilesize.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn filesize_nested_in_unary_is_detected() {
    // not filesize
    let condition = ExprNode::Unary {
        operator: yarlint::parser::syntax::operators::UnaryOperator::Not,
        expression: Box::new(ExprNode::Identifier("filesize".to_string())),
    };
    let file = make_file(vec![wide_string("$s1")], condition);
    let context = LintContext { file: &file };
    let mut findings = vec![];

    PerformanceFilesize.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn filesize_nested_in_of_count_is_detected() {
    // filesize of ($x*)  -- unusual, but the count position is itself
    // an expression and must be walked.
    let condition = ExprNode::Of {
        count: Box::new(ExprNode::Identifier("filesize".to_string())),
        pattern: "$x*".to_string(),
    };
    let file = make_file(vec![wide_string("$s1")], condition);
    let context = LintContext { file: &file };
    let mut findings = vec![];

    PerformanceFilesize.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn filesize_nested_in_module_function_arguments_is_detected() {
    // pe.some_func(filesize)
    let condition = ExprNode::ModuleFunction {
        module: "pe".to_string(),
        function: "some_func".to_string(),
        arguments: vec![ExprNode::Identifier("filesize".to_string())],
    };
    let file = make_file(vec![wide_string("$s1")], condition);
    let context = LintContext { file: &file };
    let mut findings = vec![];

    PerformanceFilesize.check(&context, &mut findings);

    assert!(findings.is_empty());
}

#[test]
fn multiple_heavy_strings_without_filesize_produce_one_finding_per_rule() {
    // The cop reports once per rule, not once per heavy string.
    let file = make_file(
        vec![wide_string("$s1"), regex_string("$s2")],
        ExprNode::BoolLiteral(true),
    );
    let context = LintContext { file: &file };
    let mut findings = vec![];

    PerformanceFilesize.check(&context, &mut findings);

    assert_eq!(findings.len(), 1);
}