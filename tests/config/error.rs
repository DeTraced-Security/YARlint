use std::error::Error;

use yarlint::config::ConfigError;

#[test]
fn io_errors_name_the_path_and_keep_their_source() {
    let err = ConfigError::Io {
        path: "yarlint.toml".into(),
        source: std::io::Error::new(std::io::ErrorKind::PermissionDenied, "nope"),
    };

    let rendered = err.to_string();
    assert!(rendered.contains("could not read"));
    assert!(rendered.contains("yarlint.toml"));
    assert!(rendered.contains("nope"));
    assert!(err.source().is_some());
}

#[test]
fn parse_errors_name_the_path_and_have_no_source() {
    let err = ConfigError::Parse {
        path: "yarlint.toml".into(),
        message: "expected `]`".to_string(),
        span: Some(0..3),
    };

    let rendered = err.to_string();
    assert!(rendered.contains("could not parse"));
    assert!(rendered.contains("expected `]`"));
    assert!(err.source().is_none());
}

#[test]
fn invalid_builds_from_anything_stringy_and_has_no_source() {
    let err = ConfigError::invalid(
        "Naming/RuleNameLength",
        "min_name_length cannot exceed max_name_length",
    );

    let rendered = err.to_string();
    assert!(rendered.contains("invalid configuration for Naming/RuleNameLength"));
    assert!(rendered.contains("cannot exceed"));
    assert!(err.source().is_none());
}

#[test]
fn a_real_parse_failure_carries_a_span() {
    use yarlint::config::{CliOverrides, Diagnostics, MapLoader};

    let loader = MapLoader::single("yarlint.toml", "[cops.Naming.RuleNameLength\n");
    let mut diagnostics = Diagnostics::new();

    let err = yarlint::config::load(
        &loader,
        crate::DECLS,
        &CliOverrides::default(),
        &mut diagnostics,
    )
    .expect_err("unterminated table header should fail");

    match err {
        ConfigError::Parse { span, path, .. } => {
            assert_eq!(path, std::path::PathBuf::from("yarlint.toml"));
            assert!(span.is_some(), "toml_edit should report a span");
        }
        other => panic!("expected a parse error, got {other:?}"),
    }
}
