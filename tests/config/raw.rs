use yarlint::config::raw::{ParsedValue, parse};
use yarlint::config::{ConfigSource, Diagnostics};

fn parse_str(text: &str) -> (yarlint::config::raw::RawConfig, Diagnostics) {
    let source = ConfigSource {
        path: "yarlint.toml".into(),
        text: text.to_string(),
    };
    let mut diagnostics = Diagnostics::new();
    let raw = parse(&source, &mut diagnostics).expect("valid toml");
    (raw, diagnostics)
}

#[test]
fn splits_framework_keys_from_params() {
    let (raw, diagnostics) = parse_str(
        r#"
[cops.Naming.RuleNameLength]
enabled = false
severity = "error"
max_name_length = 50
"#,
    );

    assert!(diagnostics.is_empty());
    let cop = raw
        .cops
        .get(&("Naming".to_string(), "RuleNameLength".to_string()))
        .expect("cop entry");

    assert_eq!(
        cop.enabled.as_ref().unwrap().value,
        ParsedValue::Bool(false)
    );
    assert_eq!(
        cop.severity.as_ref().unwrap().value,
        ParsedValue::Str("error".to_string())
    );
    assert_eq!(cop.params.len(), 1);
    assert_eq!(
        cop.params.get("max_name_length").unwrap().value,
        ParsedValue::Int(50)
    );
}

#[test]
fn records_a_span_pointing_at_the_value() {
    let text = "[cops.Naming.RuleNameLength]\nmax_name_length = 50\n";
    let (raw, _) = parse_str(text);

    let cop = raw
        .cops
        .get(&("Naming".to_string(), "RuleNameLength".to_string()))
        .unwrap();
    let span = cop
        .params
        .get("max_name_length")
        .unwrap()
        .value_span
        .clone()
        .expect("value span");

    assert_eq!(&text[span], "50");
}

#[test]
fn negative_integers_survive_parsing() {
    let (raw, _) = parse_str("[cops.Naming.RuleNameLength]\nmax_name_length = -5\n");

    let cop = raw
        .cops
        .get(&("Naming".to_string(), "RuleNameLength".to_string()))
        .unwrap();

    assert_eq!(
        cop.params.get("max_name_length").unwrap().value,
        ParsedValue::Int(-5)
    );
}

#[test]
fn mixed_arrays_are_not_string_lists() {
    let (raw, _) = parse_str("[globals]\nrequired_meta_keys = [\"author\", 3]\n");

    assert!(matches!(
        raw.globals.get("required_meta_keys").unwrap().value,
        ParsedValue::Other(_)
    ));
}

#[test]
fn unknown_top_level_key_warns_with_a_suggestion() {
    let (_, diagnostics) = parse_str("[global]\nrequired_meta_keys = []\n");

    let entry = &diagnostics.entries()[0];
    assert!(entry.message.contains("unknown top-level key"));
    assert_eq!(entry.help.as_deref(), Some("did you mean `globals`?"));
}

#[test]
fn a_cop_entry_that_is_not_a_table_is_an_error() {
    let (_, diagnostics) = parse_str("[cops.Naming]\nRuleNameLength = 3\n");

    assert!(diagnostics.has_errors());
    assert!(diagnostics.entries()[0].message.contains("must be a table"));
}

#[test]
fn an_unknown_key_in_defaults_warns_with_a_suggestion() {
    let (_, diagnostics) = parse_str("[defaults]\nenabeld = true\n");

    let entry = &diagnostics.entries()[0];
    assert!(
        entry
            .message
            .contains("unknown key `enabeld` in `[defaults]`")
    );
    assert_eq!(entry.help.as_deref(), Some("did you mean `enabled`?"));
}

#[test]
fn defaults_keeps_both_framework_keys() {
    let (raw, diagnostics) = parse_str("[defaults]\nenabled = false\nseverity = \"error\"\n");

    assert!(diagnostics.is_empty());
    assert_eq!(
        raw.defaults.enabled.unwrap().value,
        ParsedValue::Bool(false)
    );
    assert_eq!(
        raw.defaults.severity.unwrap().value,
        ParsedValue::Str("error".to_string())
    );
}

#[test]
fn a_family_that_is_not_a_table_is_skipped() {
    let (raw, diagnostics) = parse_str("[cops]\nNaming = 3\n");

    assert!(diagnostics.has_errors());
    assert!(diagnostics.entries()[0].message.contains("must be a table"));
    assert!(raw.cops.is_empty());
}

#[test]
fn a_non_table_top_level_section_is_an_error() {
    let (raw, diagnostics) = parse_str("globals = 3\n");

    assert!(diagnostics.has_errors());
    assert!(raw.globals.is_empty());
}

#[test]
fn describe_names_each_toml_shape_it_can_meet() {
    let cases = [
        ("[cops]\nNaming = 3\n", "an integer"),
        ("[cops]\nNaming = true\n", "a boolean"),
        ("[cops]\nNaming = \"x\"\n", "a string"),
        ("[cops]\nNaming = 1.5\n", "a float"),
        ("[cops]\nNaming = [1]\n", "an array"),
        ("[[cops.Naming]]\nx = 1\n", "an array of tables"),
    ];

    for (text, expected) in cases {
        let (_, diagnostics) = parse_str(text);
        let message = &diagnostics.entries()[0].message;
        assert!(
            message.contains(expected),
            "{text:?} should report {expected}, got {message:?}"
        );
    }
}

#[test]
fn non_scalar_values_become_other_with_a_type_name() {
    let cases = [
        ("[globals]\nx = 1.5\n", "a float"),
        ("[globals]\nx = { a = 1 }\n", "a table"),
        ("[globals]\nx = 1979-05-27T07:32:00Z\n", "a datetime"),
    ];

    for (text, expected) in cases {
        let (raw, _) = parse_str(text);
        let value = &raw.globals.get("x").expect("the key").value;
        assert_eq!(value.type_name(), expected, "for {text:?}");
        assert!(matches!(value, ParsedValue::Other(_)));
    }
}

#[test]
fn a_nested_table_under_globals_is_an_other_value() {
    let (raw, _) = parse_str("[globals.nested]\nx = 1\n");

    assert_eq!(
        raw.globals.get("nested").unwrap().value,
        ParsedValue::Other("a table")
    );
}

#[test]
fn scalar_values_report_their_own_type_names() {
    let (raw, _) = parse_str("[globals]\ni = 1\nb = true\ns = \"x\"\nl = [\"a\", \"b\"]\n");

    assert_eq!(raw.globals["i"].value.type_name(), "an integer");
    assert_eq!(raw.globals["b"].value.type_name(), "a boolean");
    assert_eq!(raw.globals["s"].value.type_name(), "a string");
    assert_eq!(raw.globals["l"].value.type_name(), "an array of strings");
}

#[test]
fn an_empty_file_parses_to_an_empty_config() {
    let (raw, diagnostics) = parse_str("");

    assert!(diagnostics.is_empty());
    assert_eq!(raw, yarlint::config::raw::RawConfig::default());
}

#[test]
fn blame_falls_back_between_key_and_value_spans() {
    let text = "[globals]\nrequired_meta_keys = [\"author\"]\n";
    let (raw, _) = parse_str(text);
    let value = &raw.globals["required_meta_keys"];

    assert_eq!(&text[value.blame_key().unwrap()], "required_meta_keys");
    assert_eq!(&text[value.blame_value().unwrap()], "[\"author\"]");
}
