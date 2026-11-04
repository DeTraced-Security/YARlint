//! Static declarations describing every configurable knob in YARlint.
//!
//! A [`ParamSpec`] is a *declaration*, not a value. It bundles three things
//! that must never drift apart:
//!
//! - the schema: what type the key accepts and what range is legal
//! - the default: the value used when nobody says otherwise
//! - the documentation: one line describing what the key means
//!
//! All of it is `&'static` const data, read once at startup and never again.
//! A cop's old hardcoded `const` becomes the `default` field of its spec.

use crate::linter::{cop::Category, finding::Severity};

/// The type of a configurable parameter, together with its default and the
/// range of values the configuration file is permitted to supply.
///
/// The default lives inside the kind rather than beside it so that a
/// type/default mismatch (an integer parameter with a boolean default) is
/// unrepresentable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamKind {
    /// An unsigned integer.
    ///
    /// Note that `min` and `max` bound the value the *configuration file* may
    /// supply; they say nothing about the cop's semantics. TOML integers are
    /// signed, so this range is also what rejects negative input.
    Uint {
        /// Value used when the key is absent from the configuration.
        default: u64,
        /// Smallest value the configuration file may supply, inclusive.
        min: u64,
        /// Largest value the configuration file may supply, inclusive.
        max: u64,
    },

    /// A boolean.
    Bool {
        /// Value used when the key is absent from the configuration.
        default: bool,
    },

    /// A free-form string.
    Str {
        /// Value used when the key is absent from the configuration.
        default: &'static str,
    },

    /// A list of strings.
    ///
    /// A list supplied by the configuration file *replaces* the default; it
    /// is never appended to it.
    StrList {
        /// Value used when the key is absent from the configuration.
        default: &'static [&'static str],
    },

    /// A string restricted to a fixed set of permitted values.
    Enum {
        /// The permitted values, in the order they should be documented.
        variants: &'static [&'static str],
        /// Value used when the key is absent from the configuration.
        ///
        /// Must appear in `variants`; [`check_invariants`] enforces this.
        default: &'static str,
    },
}

impl ParamKind {
    /// Human-readable type name, used in diagnostics such as
    /// "expected an integer, found a string".
    #[must_use]
    pub const fn type_name(&self) -> &'static str {
        match self {
            ParamKind::Uint { .. } => "an integer",
            ParamKind::Bool { .. } => "a boolean",
            ParamKind::Str { .. } | ParamKind::Enum { .. } => "a string",
            ParamKind::StrList { .. } => "an array of strings",
        }
    }

    /// The declared default, materialised as an owned [`ParamValue`].
    ///
    /// Called by validation for every declared key the user did not set, so
    /// that the resolved configuration is always complete.
    #[must_use]
    pub fn default_value(&self) -> ParamValue {
        match *self {
            ParamKind::Uint { default, .. } => ParamValue::Uint(default),
            ParamKind::Bool { default } => ParamValue::Bool(default),
            ParamKind::Str { default } | ParamKind::Enum { default, .. } => {
                ParamValue::Str(default.to_owned())
            }
            ParamKind::StrList { default } => {
                ParamValue::StrList(default.iter().map(|s| (*s).to_owned()).collect())
            }
        }
    }
}

/// A single configurable parameter belonging to one cop, or to the global
/// section.
#[derive(Debug, Clone, Copy)]
pub struct ParamSpec {
    /// The key as it appears in the configuration file.
    pub key: &'static str,

    /// Type, default, and permitted range.
    pub kind: ParamKind,

    /// One-line description, shown in generated documentation.
    pub doc: &'static str,
}

/// A fully resolved parameter value.
///
/// There is no `Enum` variant: a validated enum parameter resolves to
/// [`ParamValue::Str`], because validation has already confirmed the string is
/// one of the declared variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParamValue {
    /// A resolved unsigned integer.
    Uint(u64),
    /// A resolved boolean.
    Bool(bool),
    /// A resolved string.
    Str(String),
    /// A resolved list of strings.
    StrList(Vec<String>),
}

impl ParamValue {
    /// Human-readable type name, used in diagnostics.
    #[must_use]
    pub const fn type_name(&self) -> &'static str {
        match self {
            ParamValue::Uint(_) => "an integer",
            ParamValue::Bool(_) => "a boolean",
            ParamValue::Str(_) => "a string",
            ParamValue::StrList(_) => "an array of strings",
        }
    }
}

/// Everything the configuration system needs to know about one cop.
///
/// Deliberately does not reference [`Cop`](crate::linter::cop::Cop): the build
/// function that turns parameters into a cop instance lives in the registry, so
/// that this module stays free of linter types and testable on its own.
///
/// Not `Copy` only because [`Severity`] is not; it is always held as
/// `&'static CopDecl`, so that costs nothing.
#[derive(Debug, Clone)]
pub struct CopDecl {
    /// The cop's family.
    pub category: Category,

    /// The cop's bare name, without its category prefix.
    pub name: &'static str,

    /// Severity used for this cop's findings unless the configuration
    /// overrides it.
    pub default_severity: Severity,

    /// Whether this cop runs unless the configuration disables it.
    pub default_enabled: bool,

    /// The cop's configurable parameters. May be empty.
    pub params: &'static [ParamSpec],
}

impl CopDecl {
    /// The fully qualified id, for example `Naming/RuleNameLength`.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}/{}", self.category, self.name)
    }

    /// Look up one of this cop's declared parameters by key.
    #[must_use]
    pub fn spec(&self, key: &str) -> Option<&'static ParamSpec> {
        self.params.iter().find(|s| s.key == key)
    }
}

/// A violated declaration invariant.
///
/// These are authoring mistakes in YARlint's own source, not user
/// configuration errors, so they are surfaced by a test rather than at runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecViolation {
    /// An enum parameter's default is not listed among its variants.
    EnumDefaultNotAVariant {
        /// Qualified name of the owning cop, or `globals`.
        owner: String,
        /// The offending parameter key.
        key: &'static str,
    },

    /// A `Uint` parameter's default lies outside its own permitted range.
    UintDefaultOutOfRange {
        /// Qualified name of the owning cop, or `globals`.
        owner: String,
        /// The offending parameter key.
        key: &'static str,
    },

    /// A `Uint` parameter declares `min` greater than `max`.
    UintRangeInverted {
        /// Qualified name of the owning cop, or `globals`.
        owner: String,
        /// The offending parameter key.
        key: &'static str,
    },

    /// An enum parameter declares no variants.
    EnumHasNoVariants {
        /// Qualified name of the owning cop, or `globals`.
        owner: String,
        /// The offending parameter key.
        key: &'static str,
    },

    /// The same parameter key is declared twice for one owner.
    DuplicateParamKey {
        /// Qualified name of the owning cop, or `globals`.
        owner: String,
        /// The duplicated parameter key.
        key: &'static str,
    },

    /// Two cops share the same category and name.
    DuplicateCop {
        /// The duplicated qualified name.
        qualified_name: String,
    },
}

/// Check every invariant the type system cannot express.
///
/// Call this from a test over the real registry. It is cheap, but it exists to
/// catch authoring mistakes at build time rather than shipping them.
///
/// # Arguments
///
/// * `decls` - every cop declaration in the registry
/// * `globals` - the global parameter specs
#[must_use]
pub fn check_invariants(decls: &[CopDecl], globals: &[ParamSpec]) -> Vec<SpecViolation> {
    let mut out = Vec::new();
    check_params("globals", globals, &mut out);

    let mut seen: Vec<String> = Vec::new();
    for decl in decls {
        let qualified = decl.qualified_name();
        if seen.contains(&qualified) {
            out.push(SpecViolation::DuplicateCop {
                qualified_name: qualified.clone(),
            });
        } else {
            seen.push(qualified.clone());
        }
        check_params(&qualified, decl.params, &mut out);
    }
    out
}

/// Check the invariants for one owner's parameter list.
///
/// # Arguments
///
/// * `owner` - qualified cop name, or `globals`
/// * `params` - the parameter specs to check
/// * `out` - vector to push violations to
fn check_params(owner: &str, params: &[ParamSpec], out: &mut Vec<SpecViolation>) {
    let mut seen: Vec<&'static str> = Vec::new();
    for spec in params {
        if seen.contains(&spec.key) {
            out.push(SpecViolation::DuplicateParamKey {
                owner: owner.to_owned(),
                key: spec.key,
            });
        } else {
            seen.push(spec.key);
        }

        match spec.kind {
            ParamKind::Uint { default, min, max } => {
                if min > max {
                    out.push(SpecViolation::UintRangeInverted {
                        owner: owner.to_owned(),
                        key: spec.key,
                    });
                } else if default < min || default > max {
                    out.push(SpecViolation::UintDefaultOutOfRange {
                        owner: owner.to_owned(),
                        key: spec.key,
                    });
                }
            }
            ParamKind::Enum { variants, default } => {
                if variants.is_empty() {
                    out.push(SpecViolation::EnumHasNoVariants {
                        owner: owner.to_owned(),
                        key: spec.key,
                    });
                } else if !variants.contains(&default) {
                    out.push(SpecViolation::EnumDefaultNotAVariant {
                        owner: owner.to_owned(),
                        key: spec.key,
                    });
                }
            }
            ParamKind::Bool { .. } | ParamKind::Str { .. } | ParamKind::StrList { .. } => {}
        }
    }
}
