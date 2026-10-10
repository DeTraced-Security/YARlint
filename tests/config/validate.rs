use yarlint::config::ParamValue;
use yarlint::linter::cop::Category;

use crate::{load, messages};

#[test]
fn an_unknown_param_warns_and_is_ignored() {
    let (resolved, diagnostics) = load("[cops.Naming.RuleNameLength]\nmax_name_lenght = 50\n");

    let entry = &diagnostics.entries()[0];
    assert!(entry.message.contains("unknown option `max_name_lenght`"));
    assert_eq!(
        entry.help.as_deref(),
        Some("did you mean `max_name_length`?")
    );
    assert_eq!(
        resolved
            .get(Category::Naming, "RuleNameLength")
            .unwrap()
            .params
            .get("max_name_length"),
        Some(&ParamValue::Uint(80))
    );
}

#[test]
fn a_negative_integer_is_rejected() {
    let (resolved, diagnostics) = load("[cops.Naming.RuleNameLength]\nmax_name_length = -5\n");

    assert!(diagnostics.has_errors(), "{:?}", messages(&diagnostics));
    assert_eq!(
        resolved
            .get(Category::Naming, "RuleNameLength")
            .unwrap()
            .params
            .get("max_name_length"),
        Some(&ParamValue::Uint(80)),
        "a rejected value falls back to the declared default"
    );
}

#[test]
fn an_out_of_range_integer_is_rejected() {
    let (_, diagnostics) = load("[cops.Naming.RuleNameLength]\nmax_name_length = 9000\n");

    assert!(diagnostics.has_errors());
    assert!(
        diagnostics.entries()[0]
            .message
            .contains("between 1 and 512")
    );
}

#[test]
fn a_float_does_not_coerce_to_an_integer() {
    let (_, diagnostics) = load("[cops.Naming.RuleNameLength]\nmax_name_length = 50.0\n");

    assert!(diagnostics.has_errors());
    assert!(
        diagnostics.entries()[0]
            .message
            .contains("must be an integer")
    );
}

#[test]
fn a_string_is_not_an_integer() {
    let (_, diagnostics) = load("[cops.Naming.RuleNameLength]\nmax_name_length = \"50\"\n");

    assert!(diagnostics.has_errors());
    assert!(diagnostics.entries()[0].message.contains("found a string"));
}

#[test]
fn an_enum_value_outside_its_variants_is_rejected() {
    let (resolved, diagnostics) = load("[cops.Style.RuleNameCase]\ncase = \"kebab\"\n");

    assert!(diagnostics.has_errors());
    assert!(
        diagnostics.entries()[0]
            .help
            .as_deref()
            .unwrap()
            .contains("PascalCase")
    );
    assert_eq!(
        resolved
            .get(Category::Style, "RuleNameCase")
            .unwrap()
            .params
            .get("case"),
        Some(&ParamValue::Str("PascalCase".to_string()))
    );
}

#[test]
fn a_valid_enum_value_is_accepted() {
    let (resolved, diagnostics) = load("[cops.Style.RuleNameCase]\ncase = \"SnakeCase\"\n");

    assert!(diagnostics.is_empty(), "{:?}", messages(&diagnostics));
    assert_eq!(
        resolved
            .get(Category::Style, "RuleNameCase")
            .unwrap()
            .params
            .get("case"),
        Some(&ParamValue::Str("SnakeCase".to_string()))
    );
}

#[test]
fn every_declared_param_is_present_even_when_unmentioned() {
    let (resolved, _) = load("");

    for settings in resolved.iter() {
        for spec in settings.decl.params {
            assert!(
                settings.params.contains_key(spec.key),
                "{} is missing {}",
                settings.decl.qualified_name(),
                spec.key
            );
        }
    }
}

#[test]
fn diagnostics_are_collected_not_short_circuited() {
    let (_, diagnostics) = load(
        r#"
[cops.Naming.RuleNameLength]
max_name_length = 9000
bogus = 1

[cops.Style.RuleNameCase]
case = "kebab"
"#,
    );

    assert!(
        diagnostics.entries().len() >= 3,
        "{:?}",
        messages(&diagnostics)
    );
}

#[test]
fn a_boolean_param_is_coerced() {
    let (resolved, diagnostics) = load("[cops.Lint.Fixture]\nstrict = false\n");

    assert!(diagnostics.is_empty(), "{:?}", messages(&diagnostics));
    assert_eq!(
        resolved
            .get(Category::Lint, "Fixture")
            .unwrap()
            .params
            .get("strict"),
        Some(&ParamValue::Bool(false))
    );
}

#[test]
fn a_string_param_is_coerced() {
    let (resolved, diagnostics) = load("[cops.Lint.Fixture]\nprefix = \"yara_\"\n");

    assert!(diagnostics.is_empty(), "{:?}", messages(&diagnostics));
    assert_eq!(
        resolved
            .get(Category::Lint, "Fixture")
            .unwrap()
            .params
            .get("prefix"),
        Some(&ParamValue::Str("yara_".to_string()))
    );
}

#[test]
fn a_string_list_param_is_coerced() {
    let (resolved, diagnostics) =
        load("[cops.Lint.Fixture]\nignored_tags = [\"draft\", \"old\"]\n");

    assert!(diagnostics.is_empty(), "{:?}", messages(&diagnostics));
    assert_eq!(
        resolved
            .get(Category::Lint, "Fixture")
            .unwrap()
            .params
            .get("ignored_tags"),
        Some(&ParamValue::StrList(vec![
            "draft".to_string(),
            "old".to_string()
        ]))
    );
}

#[test]
fn an_empty_string_list_is_accepted() {
    let (resolved, diagnostics) = load("[cops.Lint.Fixture]\nignored_tags = []\n");

    assert!(diagnostics.is_empty(), "{:?}", messages(&diagnostics));
    assert_eq!(
        resolved
            .get(Category::Lint, "Fixture")
            .unwrap()
            .params
            .get("ignored_tags"),
        Some(&ParamValue::StrList(Vec::new()))
    );
}

#[test]
fn every_param_kind_rejects_the_wrong_shape() {
    let cases = [
        ("strict = 1", "must be a boolean, found an integer"),
        ("prefix = 3", "must be a string, found an integer"),
        (
            "ignored_tags = \"draft\"",
            "must be an array of strings, found a string",
        ),
        (
            "ignored_tags = [\"a\", 1]",
            "must be an array of strings, found an array with non-string elements",
        ),
    ];

    for (line, expected) in cases {
        let text = format!("[cops.Lint.Fixture]\n{line}\n");
        let (_, diagnostics) = load(&text);

        assert!(diagnostics.has_errors(), "{line} should be rejected");
        assert!(
            diagnostics.entries()[0].message.contains(expected),
            "{line} should say {expected:?}, got {:?}",
            messages(&diagnostics)
        );
    }
}

#[test]
fn a_boolean_where_an_integer_belongs_names_the_boolean() {
    let (_, diagnostics) = load("[cops.Naming.RuleNameLength]\nmax_name_length = true\n");

    assert!(diagnostics.has_errors());
    assert!(
        diagnostics.entries()[0]
            .message
            .contains("must be an integer, found a boolean")
    );
}

#[test]
fn a_rejected_param_falls_back_but_the_rest_still_apply() {
    let (resolved, diagnostics) = load(
        r#"
[cops.Lint.Fixture]
strict = 1
prefix = "yara_"
"#,
    );

    assert!(diagnostics.has_errors());
    let settings = resolved.get(Category::Lint, "Fixture").unwrap();
    assert_eq!(settings.params.get("strict"), Some(&ParamValue::Bool(true)));
    assert_eq!(
        settings.params.get("prefix"),
        Some(&ParamValue::Str("yara_".to_string()))
    );
}

#[test]
fn an_unknown_global_key_warns() {
    let (_, diagnostics) = load("[globals]\nrequired_meta_key = [\"author\"]\n");

    let entry = &diagnostics.entries()[0];
    assert!(entry.message.contains("unknown option"));
    assert!(entry.message.contains("for globals"));
    assert_eq!(
        entry.help.as_deref(),
        Some("did you mean `required_meta_keys`?")
    );
}

#[test]
fn a_global_of_the_wrong_type_falls_back_to_its_default() {
    let (resolved, diagnostics) = load("[globals]\nrequired_meta_keys = \"author\"\n");

    assert!(diagnostics.has_errors());
    assert_eq!(
        resolved.globals.required_meta_keys,
        vec!["author", "description", "reference", "date"]
    );
}

#[test]
fn a_cop_with_no_params_gets_an_empty_map() {
    let (resolved, _) = load("");

    assert!(
        resolved
            .get(Category::Style, "MissingRequiredMeta")
            .unwrap()
            .params
            .is_empty()
    );
}
