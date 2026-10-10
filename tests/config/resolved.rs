use yarlint::config::resolved::{parse_severity, severity_name};
use yarlint::config::{CliOverrides, Origin};
use yarlint::linter::{cop::Category, finding::Severity};

use crate::{load, load_with};

#[test]
fn severity_names_round_trip() {
    for name in ["info", "warning", "error"] {
        let severity = parse_severity(name).expect("a severity");
        assert_eq!(severity_name(&severity), name);
    }
}

#[test]
fn severity_parsing_is_case_insensitive() {
    assert_eq!(parse_severity("WARNING"), Some(Severity::Warning));
    assert_eq!(parse_severity("Error"), Some(Severity::Error));
    assert_eq!(parse_severity("nonsense"), None);
}

#[test]
fn origin_renders_each_variant() {
    assert_eq!(Origin::Default.to_string(), "default");
    assert_eq!(Origin::CommandLine.to_string(), "command line");
    assert_eq!(
        Origin::File {
            path: "yarlint.toml".into(),
            line: 4,
            column: 19,
        }
        .to_string(),
        "yarlint.toml:4:19"
    );
}

#[test]
fn origin_from_span_falls_back_when_there_is_no_span() {
    let origin = Origin::from_span(std::path::Path::new("yarlint.toml"), "a = 1\n", None);

    assert_eq!(origin.to_string(), "yarlint.toml:1:1");
}

#[test]
fn a_command_line_override_is_recorded_as_such() {
    let cli = CliOverrides {
        only: vec!["Naming/RuleNameLength".to_string()],
        except: vec![],
    };
    let (resolved, _) = load_with("", &cli);

    assert_eq!(
        resolved
            .get(Category::Naming, "RuleNameLength")
            .unwrap()
            .origin("enabled"),
        Origin::CommandLine
    );
}

#[test]
fn an_unrecorded_key_reports_itself_as_default() {
    let (resolved, _) = load("");
    let settings = resolved.get(Category::Naming, "RuleNameLength").unwrap();

    assert_eq!(settings.origin("never_set_by_anything"), Origin::Default);
}

#[test]
fn config_show_renders_every_value_with_its_origin() {
    let (resolved, _) = load(
        r#"
[globals]
required_meta_keys = ["author"]

[cops.Lint.Fixture]
severity = "error"
strict = false
prefix = "yara_"
ignored_tags = ["draft"]

[cops.Naming.RuleNameLength]
max_name_length = 50
"#,
    );

    let rendered = resolved.to_string();

    assert!(rendered.contains("[globals]"));
    assert!(rendered.contains("required_meta_keys = [\"author\"]"));

    assert!(rendered.contains("[cops.Lint.Fixture]"));
    assert!(rendered.contains("severity = \"error\""));
    assert!(rendered.contains("strict = false"));
    assert!(rendered.contains("prefix = \"yara_\""));
    assert!(rendered.contains("ignored_tags = [\"draft\"]"));

    assert!(rendered.contains("[cops.Naming.RuleNameLength]"));
    assert!(rendered.contains("max_name_length = 50"));

    // Origins are rendered as trailing comments.
    assert!(rendered.contains("# default"));
    assert!(rendered.contains("# yarlint.toml:"));
}

#[test]
fn config_show_covers_every_registered_cop() {
    let (resolved, _) = load("");
    let rendered = resolved.to_string();

    for settings in resolved.iter() {
        let heading = format!("[cops.{}.{}]", settings.decl.category, settings.decl.name);
        assert!(rendered.contains(&heading), "missing {heading}");
    }
}

#[test]
fn enabled_is_a_subset_of_iter() {
    let (resolved, _) = load("[cops.Style.RuleNameCase]\nenabled = false\n");

    assert_eq!(resolved.iter().count(), crate::DECLS.len());
    assert_eq!(resolved.enabled().count(), crate::DECLS.len() - 1);
}

#[test]
fn get_returns_none_for_an_unregistered_cop() {
    let (resolved, _) = load("");

    assert!(resolved.get(Category::Logic, "NotARealCop").is_none());
    assert!(resolved.get(Category::Logic, "RuleNameLength").is_none());
}
