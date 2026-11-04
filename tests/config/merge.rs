use yarlint::config::{CliOverrides, Origin, ParamValue};
use yarlint::linter::{cop::Category, finding::Severity};

use crate::{load, load_with, messages};

#[test]
fn defaults_section_applies_to_every_cop() {
    let (resolved, diagnostics) = load("[defaults]\nenabled = false\n");

    assert!(diagnostics.is_empty(), "{:?}", messages(&diagnostics));
    assert_eq!(resolved.enabled().count(), 0);
}

#[test]
fn a_cop_entry_beats_the_defaults_section() {
    let (resolved, _) = load(
        r#"
[defaults]
enabled = false

[cops.Style.RuleNameCase]
enabled = true
"#,
    );

    assert_eq!(resolved.enabled().count(), 1);
    assert!(
        resolved
            .get(Category::Style, "RuleNameCase")
            .unwrap()
            .enabled
    );
}

#[test]
fn severity_is_overridable_per_cop() {
    let (resolved, diagnostics) = load(
        r#"
[cops.Naming.RuleNameLength]
severity = "error"
"#,
    );

    assert!(diagnostics.is_empty(), "{:?}", messages(&diagnostics));
    assert_eq!(
        resolved
            .get(Category::Naming, "RuleNameLength")
            .unwrap()
            .severity,
        Severity::Error
    );
}

#[test]
fn an_unknown_severity_is_rejected_with_a_suggestion() {
    let (resolved, diagnostics) = load(
        r#"
[cops.Naming.RuleNameLength]
severity = "warn"
"#,
    );

    assert!(diagnostics.has_errors());
    assert_eq!(
        diagnostics.entries()[0].help.as_deref(),
        Some("did you mean `warning`?")
    );
    assert_eq!(
        resolved
            .get(Category::Naming, "RuleNameLength")
            .unwrap()
            .severity,
        Severity::Info,
        "a rejected severity falls back to the declared default"
    );
}

#[test]
fn an_unknown_cop_warns_with_a_suggestion() {
    let (_, diagnostics) = load("[cops.Naming.RuleNameLenght]\nmax_name_length = 50\n");

    let entry = &diagnostics.entries()[0];
    assert!(entry.message.contains("unknown cop"));
    assert_eq!(
        entry.help.as_deref(),
        Some("did you mean `Naming/RuleNameLength`?")
    );
}

#[test]
fn an_unknown_family_warns() {
    let (_, diagnostics) = load("[cops.Nameing.RuleNameLength]\nmax_name_length = 50\n");

    let entry = &diagnostics.entries()[0];
    assert!(entry.message.contains("unknown cop family"));
    assert_eq!(entry.help.as_deref(), Some("did you mean `Naming`?"));
}

#[test]
fn only_replaces_the_enabled_set_rather_than_intersecting() {
    let cli = CliOverrides {
        only: vec!["Style/RuleNameCase".to_string()],
        except: vec![],
    };

    let (resolved, diagnostics) = load_with("[defaults]\nenabled = true\n", &cli);

    assert!(diagnostics.is_empty(), "{:?}", messages(&diagnostics));
    assert_eq!(resolved.enabled().count(), 1);
    assert!(
        resolved
            .get(Category::Style, "RuleNameCase")
            .unwrap()
            .enabled
    );
}

#[test]
fn except_wins_over_the_config_file() {
    let cli = CliOverrides {
        only: vec![],
        except: vec!["Style".to_string()],
    };

    let (resolved, _) = load_with("", &cli);

    assert!(
        !resolved
            .get(Category::Style, "RuleNameCase")
            .unwrap()
            .enabled
    );
    assert!(
        !resolved
            .get(Category::Style, "MissingRequiredMeta")
            .unwrap()
            .enabled
    );
    assert!(
        resolved
            .get(Category::Naming, "RuleNameLength")
            .unwrap()
            .enabled
    );
}

#[test]
fn a_list_from_the_file_replaces_the_default_rather_than_appending() {
    let (resolved, diagnostics) = load(
        r#"
[globals]
required_meta_keys = ["author"]
"#,
    );

    assert!(diagnostics.is_empty(), "{:?}", messages(&diagnostics));
    assert_eq!(resolved.globals.required_meta_keys, vec!["author"]);
}

#[test]
fn origin_records_where_a_value_came_from() {
    let (resolved, _) = load("[cops.Naming.RuleNameLength]\nmax_name_length = 50\n");
    let settings = resolved.get(Category::Naming, "RuleNameLength").unwrap();

    assert_eq!(settings.origin("min_name_length"), Origin::Default);
    assert!(matches!(
        settings.origin("max_name_length"),
        Origin::File { line: 2, .. }
    ));
    assert_eq!(
        settings.params.get("max_name_length"),
        Some(&ParamValue::Uint(50))
    );
}

#[test]
fn defaults_section_sets_severity_for_every_cop() {
    let (resolved, diagnostics) = load("[defaults]\nseverity = \"error\"\n");

    assert!(diagnostics.is_empty(), "{:?}", messages(&diagnostics));
    for settings in resolved.iter() {
        assert_eq!(
            settings.severity,
            Severity::Error,
            "{} kept its own severity",
            settings.decl.qualified_name()
        );
    }
}

#[test]
fn a_cop_entry_beats_the_defaults_severity() {
    let (resolved, _) = load(
        r#"
[defaults]
severity = "error"

[cops.Naming.RuleNameLength]
severity = "info"
"#,
    );

    assert_eq!(
        resolved
            .get(Category::Naming, "RuleNameLength")
            .unwrap()
            .severity,
        Severity::Info
    );
    assert_eq!(
        resolved
            .get(Category::Style, "RuleNameCase")
            .unwrap()
            .severity,
        Severity::Error
    );
}

#[test]
fn an_unknown_severity_in_defaults_is_rejected() {
    let (resolved, diagnostics) = load("[defaults]\nseverity = \"loud\"\n");

    assert!(diagnostics.has_errors());
    assert_eq!(
        resolved
            .get(Category::Naming, "RuleNameLength")
            .unwrap()
            .severity,
        Severity::Info,
        "every cop keeps its declared default"
    );
}

#[test]
fn a_non_boolean_enabled_is_rejected() {
    let (resolved, diagnostics) = load("[cops.Naming.RuleNameLength]\nenabled = \"yes\"\n");

    assert!(diagnostics.has_errors());
    assert!(
        diagnostics.entries()[0]
            .message
            .contains("`enabled` must be a boolean, found a string")
    );
    assert!(
        resolved
            .get(Category::Naming, "RuleNameLength")
            .unwrap()
            .enabled
    );
}

#[test]
fn a_non_boolean_enabled_in_defaults_is_rejected() {
    let (_, diagnostics) = load("[defaults]\nenabled = 1\n");

    assert!(diagnostics.has_errors());
    assert!(
        diagnostics.entries()[0]
            .message
            .contains("must be a boolean, found an integer")
    );
}

#[test]
fn a_non_string_severity_is_rejected() {
    let (_, diagnostics) = load("[cops.Naming.RuleNameLength]\nseverity = 3\n");

    assert!(diagnostics.has_errors());
    assert!(
        diagnostics.entries()[0]
            .message
            .contains("`severity` must be a string, found an integer")
    );
}

#[test]
fn an_unknown_cop_selector_warns_with_a_suggestion() {
    let cli = CliOverrides {
        only: vec!["Naming/RuleNameLenght".to_string()],
        except: vec![],
    };

    let (_, diagnostics) = load_with("", &cli);

    let entry = &diagnostics.entries()[0];
    assert!(entry.message.contains("unknown cop selector"));
    assert_eq!(
        entry.help.as_deref(),
        Some("did you mean `Naming/RuleNameLength`?")
    );
}

#[test]
fn an_unknown_except_selector_warns_too() {
    let cli = CliOverrides {
        only: vec![],
        except: vec!["Nonsense".to_string()],
    };

    let (resolved, diagnostics) = load_with("", &cli);

    assert!(
        diagnostics.entries()[0]
            .message
            .contains("unknown cop selector")
    );
    assert_eq!(resolved.enabled().count(), crate::DECLS.len());
}

#[test]
fn a_category_selector_enables_a_whole_family() {
    let cli = CliOverrides {
        only: vec!["Style".to_string()],
        except: vec![],
    };

    let (resolved, _) = load_with("", &cli);

    assert_eq!(resolved.enabled().count(), 2);
    assert!(
        resolved
            .get(Category::Style, "RuleNameCase")
            .unwrap()
            .enabled
    );
    assert!(
        !resolved
            .get(Category::Naming, "RuleNameLength")
            .unwrap()
            .enabled
    );
}

#[test]
fn except_narrows_only() {
    let cli = CliOverrides {
        only: vec!["Style".to_string()],
        except: vec!["Style/RuleNameCase".to_string()],
    };

    let (resolved, _) = load_with("", &cli);

    assert_eq!(resolved.enabled().count(), 1);
    assert!(
        resolved
            .get(Category::Style, "MissingRequiredMeta")
            .unwrap()
            .enabled
    );
}

#[test]
fn severity_origin_points_at_the_line_that_set_it() {
    let (resolved, _) = load("[cops.Naming.RuleNameLength]\nseverity = \"error\"\n");
    let settings = resolved.get(Category::Naming, "RuleNameLength").unwrap();

    assert!(matches!(
        settings.origin("severity"),
        Origin::File { line: 2, .. }
    ));
    assert_eq!(settings.origin("enabled"), Origin::Default);
}

#[test]
fn a_rejected_value_leaves_its_origin_as_default() {
    let (resolved, _) = load("[cops.Naming.RuleNameLength]\nmax_name_length = 9000\n");
    let settings = resolved.get(Category::Naming, "RuleNameLength").unwrap();

    assert_eq!(settings.origin("max_name_length"), Origin::Default);
    assert_eq!(
        settings.params.get("max_name_length"),
        Some(&ParamValue::Uint(80))
    );
}
