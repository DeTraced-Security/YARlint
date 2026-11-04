use yarlint::config::spec::{ParamKind, ParamSpec, SpecViolation, check_invariants};
use yarlint::config::{CopDecl, ParamValue};
use yarlint::linter::{cop::Category, finding::Severity};

use crate::DECLS;

#[test]
fn fixture_registry_satisfies_every_invariant() {
    let violations = check_invariants(DECLS, yarlint::config::globals::GLOBALS_SPEC);
    assert!(violations.is_empty(), "{violations:?}");
}

#[test]
fn enum_default_must_be_a_variant() {
    static BAD: &[CopDecl] = &[CopDecl {
        category: Category::Style,
        name: "Broken",
        default_severity: Severity::Info,
        default_enabled: true,
        params: &[ParamSpec {
            key: "case",
            kind: ParamKind::Enum {
                variants: &["PascalCase"],
                default: "SnakeCase",
            },
            doc: "",
        }],
    }];

    let violations = check_invariants(BAD, &[]);

    assert!(matches!(
        violations.as_slice(),
        [SpecViolation::EnumDefaultNotAVariant { key: "case", .. }]
    ));
}

#[test]
fn uint_default_must_sit_inside_its_range() {
    static BAD: &[CopDecl] = &[CopDecl {
        category: Category::Naming,
        name: "Broken",
        default_severity: Severity::Info,
        default_enabled: true,
        params: &[ParamSpec {
            key: "size",
            kind: ParamKind::Uint {
                default: 900,
                min: 1,
                max: 512,
            },
            doc: "",
        }],
    }];

    let violations = check_invariants(BAD, &[]);

    assert!(matches!(
        violations.as_slice(),
        [SpecViolation::UintDefaultOutOfRange { key: "size", .. }]
    ));
}

#[test]
fn duplicate_cop_names_are_caught() {
    static BAD: &[CopDecl] = &[
        CopDecl {
            category: Category::Naming,
            name: "Same",
            default_severity: Severity::Info,
            default_enabled: true,
            params: &[],
        },
        CopDecl {
            category: Category::Naming,
            name: "Same",
            default_severity: Severity::Info,
            default_enabled: true,
            params: &[],
        },
    ];

    let violations = check_invariants(BAD, &[]);

    assert!(matches!(
        violations.as_slice(),
        [SpecViolation::DuplicateCop { .. }]
    ));
}

#[test]
fn defaults_materialise_with_the_right_variant() {
    let kind = ParamKind::StrList {
        default: &["author", "date"],
    };

    assert_eq!(
        kind.default_value(),
        ParamValue::StrList(vec!["author".to_string(), "date".to_string()])
    );
}

#[test]
fn every_kind_reports_a_type_name() {
    assert_eq!(
        ParamKind::Uint {
            default: 1,
            min: 1,
            max: 2
        }
        .type_name(),
        "an integer"
    );
    assert_eq!(ParamKind::Bool { default: true }.type_name(), "a boolean");
    assert_eq!(ParamKind::Str { default: "x" }.type_name(), "a string");
    assert_eq!(
        ParamKind::Enum {
            variants: &["x"],
            default: "x"
        }
        .type_name(),
        "a string"
    );
    assert_eq!(
        ParamKind::StrList { default: &[] }.type_name(),
        "an array of strings"
    );
}

#[test]
fn every_value_reports_a_type_name() {
    assert_eq!(ParamValue::Uint(1).type_name(), "an integer");
    assert_eq!(ParamValue::Bool(true).type_name(), "a boolean");
    assert_eq!(ParamValue::Str(String::new()).type_name(), "a string");
    assert_eq!(
        ParamValue::StrList(Vec::new()).type_name(),
        "an array of strings"
    );
}

#[test]
fn every_kind_materialises_its_default() {
    assert_eq!(
        ParamKind::Uint {
            default: 7,
            min: 1,
            max: 9
        }
        .default_value(),
        ParamValue::Uint(7)
    );
    assert_eq!(
        ParamKind::Bool { default: false }.default_value(),
        ParamValue::Bool(false)
    );
    assert_eq!(
        ParamKind::Str { default: "rule_" }.default_value(),
        ParamValue::Str("rule_".to_string())
    );
    assert_eq!(
        ParamKind::Enum {
            variants: &["a", "b"],
            default: "b"
        }
        .default_value(),
        ParamValue::Str("b".to_string())
    );
}

#[test]
fn a_decl_can_look_up_its_own_specs() {
    let decl = DECLS
        .iter()
        .find(|d| d.name == "RuleNameLength")
        .expect("fixture cop");

    assert_eq!(decl.qualified_name(), "Naming/RuleNameLength");
    assert_eq!(decl.spec("max_name_length").unwrap().key, "max_name_length");
    assert!(decl.spec("not_a_key").is_none());
}

#[test]
fn a_cop_with_no_params_looks_up_nothing() {
    let decl = DECLS
        .iter()
        .find(|d| d.name == "MissingRequiredMeta")
        .expect("fixture cop");

    assert!(decl.params.is_empty());
    assert!(decl.spec("anything").is_none());
}

#[test]
fn duplicate_param_keys_are_caught() {
    static BAD: &[ParamSpec] = &[
        ParamSpec {
            key: "size",
            kind: ParamKind::Uint {
                default: 1,
                min: 1,
                max: 2,
            },
            doc: "",
        },
        ParamSpec {
            key: "size",
            kind: ParamKind::Bool { default: true },
            doc: "",
        },
    ];

    let violations = check_invariants(&[], BAD);

    assert!(matches!(
        violations.as_slice(),
        [SpecViolation::DuplicateParamKey { key: "size", .. }]
    ));
}

#[test]
fn an_inverted_uint_range_is_caught_instead_of_the_default_check() {
    static BAD: &[ParamSpec] = &[ParamSpec {
        key: "size",
        kind: ParamKind::Uint {
            default: 5,
            min: 10,
            max: 2,
        },
        doc: "",
    }];

    let violations = check_invariants(&[], BAD);

    assert!(matches!(
        violations.as_slice(),
        [SpecViolation::UintRangeInverted { key: "size", .. }]
    ));
}

#[test]
fn an_enum_with_no_variants_is_caught() {
    static BAD: &[ParamSpec] = &[ParamSpec {
        key: "case",
        kind: ParamKind::Enum {
            variants: &[],
            default: "PascalCase",
        },
        doc: "",
    }];

    let violations = check_invariants(&[], BAD);

    assert!(matches!(
        violations.as_slice(),
        [SpecViolation::EnumHasNoVariants { key: "case", .. }]
    ));
}

#[test]
fn violations_name_the_owner_they_came_from() {
    static BAD: &[CopDecl] = &[CopDecl {
        category: Category::Logic,
        name: "Broken",
        default_severity: Severity::Info,
        default_enabled: true,
        params: &[ParamSpec {
            key: "size",
            kind: ParamKind::Uint {
                default: 900,
                min: 1,
                max: 2,
            },
            doc: "",
        }],
    }];

    let violations = check_invariants(BAD, &[]);

    match &violations[0] {
        SpecViolation::UintDefaultOutOfRange { owner, .. } => {
            assert_eq!(owner, "Logic/Broken");
        }
        other => panic!("expected an out-of-range violation, got {other:?}"),
    }
}

#[test]
fn global_violations_are_attributed_to_globals() {
    static BAD: &[ParamSpec] = &[ParamSpec {
        key: "case",
        kind: ParamKind::Enum {
            variants: &["a"],
            default: "b",
        },
        doc: "",
    }];

    let violations = check_invariants(&[], BAD);

    match &violations[0] {
        SpecViolation::EnumDefaultNotAVariant { owner, .. } => {
            assert_eq!(owner, "globals");
        }
        other => panic!("expected an enum violation, got {other:?}"),
    }
}
