use yarlint::config::{
    CliOverrides, CopDecl, Diagnostics, MapLoader, ParamKind, ParamSpec, ParamValue,
};
use yarlint::linter::{cop::Category, finding::Severity};

#[path = "config/spec.rs"]
pub mod spec;

#[path = "config/raw.rs"]
pub mod raw;

#[path = "config/merge.rs"]
pub mod merge;

#[path = "config/validate.rs"]
pub mod validate;

#[path = "config/params.rs"]
pub mod params;

#[path = "config/error.rs"]
pub mod error;

#[path = "config/diagnostics.rs"]
pub mod diagnostics;

#[path = "config/source.rs"]
pub mod source;

#[path = "config/resolved.rs"]
pub mod resolved;

/// A stand-in registry used by every configuration test.
pub static DECLS: &[CopDecl] = &[
    CopDecl {
        category: Category::Lint,
        name: "Fixture",
        default_severity: Severity::Warning,
        default_enabled: true,
        params: &[
            ParamSpec {
                key: "strict",
                kind: ParamKind::Bool { default: true },
                doc: "Whether to apply the strict interpretation.",
            },
            ParamSpec {
                key: "prefix",
                kind: ParamKind::Str { default: "rule_" },
                doc: "Prefix rule names are expected to carry.",
            },
            ParamSpec {
                key: "ignored_tags",
                kind: ParamKind::StrList {
                    default: &["test", "wip"],
                },
                doc: "Tags that exempt a rule from this cop.",
            },
        ],
    },
    CopDecl {
        category: Category::Naming,
        name: "RuleNameLength",
        default_severity: Severity::Info,
        default_enabled: true,
        params: &[
            ParamSpec {
                key: "min_name_length",
                kind: ParamKind::Uint {
                    default: 4,
                    min: 1,
                    max: 512,
                },
                doc: "Shortest permitted rule name, in characters.",
            },
            ParamSpec {
                key: "max_name_length",
                kind: ParamKind::Uint {
                    default: 80,
                    min: 1,
                    max: 512,
                },
                doc: "Longest permitted rule name, in characters.",
            },
        ],
    },
    CopDecl {
        category: Category::Style,
        name: "RuleNameCase",
        default_severity: Severity::Warning,
        default_enabled: true,
        params: &[ParamSpec {
            key: "case",
            kind: ParamKind::Enum {
                variants: &["PascalCase", "SnakeCase"],
                default: "PascalCase",
            },
            doc: "Casing convention rule names must follow.",
        }],
    },
    CopDecl {
        category: Category::Style,
        name: "MissingRequiredMeta",
        default_severity: Severity::Error,
        default_enabled: true,
        params: &[],
    },
];

/// Load configuration from a TOML string, returning the result and any
/// diagnostics.
pub fn load(text: &str) -> (yarlint::config::ResolvedConfig, Diagnostics) {
    load_with(text, &CliOverrides::default())
}

/// Load configuration from a TOML string with command-line overrides applied.
pub fn load_with(text: &str, cli: &CliOverrides) -> (yarlint::config::ResolvedConfig, Diagnostics) {
    let loader = MapLoader::single("yarlint.toml", text);
    let mut diagnostics = Diagnostics::new();
    let resolved = yarlint::config::load(&loader, DECLS, cli, &mut diagnostics)
        .expect("configuration should load");
    (resolved, diagnostics)
}

/// Every diagnostic message, for assertions.
pub fn messages(diagnostics: &Diagnostics) -> Vec<String> {
    diagnostics
        .entries()
        .iter()
        .map(|entry| entry.message.clone())
        .collect()
}

#[test]
fn no_config_file_yields_registry_defaults() {
    let loader = MapLoader::new();
    let mut diagnostics = Diagnostics::new();

    let resolved =
        yarlint::config::load(&loader, DECLS, &CliOverrides::default(), &mut diagnostics)
            .expect("configuration should load");

    assert!(diagnostics.is_empty());
    let settings = resolved
        .get(Category::Naming, "RuleNameLength")
        .expect("cop should be registered");
    assert!(settings.enabled);
    assert_eq!(settings.severity, Severity::Info);
    assert_eq!(
        settings.params.get("max_name_length"),
        Some(&ParamValue::Uint(80))
    );
}

#[test]
fn file_overrides_defaults_and_leaves_the_rest_alone() {
    let (resolved, diagnostics) = load(
        r#"
[cops.Naming.RuleNameLength]
max_name_length = 50
"#,
    );

    assert!(diagnostics.is_empty(), "{:?}", messages(&diagnostics));
    let settings = resolved.get(Category::Naming, "RuleNameLength").unwrap();
    assert_eq!(
        settings.params.get("max_name_length"),
        Some(&ParamValue::Uint(50))
    );
    assert_eq!(
        settings.params.get("min_name_length"),
        Some(&ParamValue::Uint(4))
    );
}

#[test]
fn malformed_toml_is_a_hard_error() {
    let loader = MapLoader::single("yarlint.toml", "[cops.Naming.RuleNameLength\n");
    let mut diagnostics = Diagnostics::new();

    let result = yarlint::config::load(&loader, DECLS, &CliOverrides::default(), &mut diagnostics);

    assert!(result.is_err());
}
