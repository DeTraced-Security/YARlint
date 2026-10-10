//! Layering the configuration file over the registry defaults.
//!
//! Precedence, lowest to highest: the declared defaults from the registry, then
//! the configuration file, then the command line.
//!
//! List parameters *replace* rather than append. A `StrList` set in the
//! configuration file discards the declared default entirely.

use std::collections::BTreeMap;
use std::ops::Range;

use crate::config::diagnostics::{DiagnosticBuilder, Diagnostics};
use crate::config::raw::{ParsedValue, RawConfig, RawCop, RawValue};
use crate::config::resolved::{Origin, SEVERITY_NAMES, parse_severity};
use crate::config::source::ConfigSource;
use crate::config::spec::CopDecl;
use crate::linter::{cop::Category, finding::Severity};

/// A value carried through merging together with the file it came from.
#[derive(Debug, Clone)]
pub(crate) struct Sourced {
    /// The value as written.
    pub raw: RawValue,

    /// Index into [`Merged::files`] identifying the file it came from.
    pub file: usize,
}

/// One cop's state part-way through resolution.
#[derive(Debug, Clone)]
pub(crate) struct MergedCop {
    /// The cop's declaration.
    pub decl: &'static CopDecl,

    /// Whether the cop should run.
    pub enabled: bool,

    /// Where `enabled` came from.
    pub enabled_origin: Origin,

    /// Severity for this cop's findings.
    pub severity: Severity,

    /// Where `severity` came from.
    pub severity_origin: Origin,

    /// Parameters as written, still string-keyed and unchecked.
    pub params: BTreeMap<String, Sourced>,
}

/// The merged configuration, before parameter types have been checked.
///
/// Deliberately `pub(crate)`: it is an intermediate state and must not escape
/// the configuration module.
#[derive(Debug, Clone, Default)]
pub(crate) struct Merged {
    /// Every file that contributed, for resolving diagnostics back to source.
    pub files: Vec<ConfigSource>,

    /// The `[globals]` section as written.
    pub globals: BTreeMap<String, Sourced>,

    /// Every registered cop, keyed by category and name.
    pub cops: BTreeMap<(Category, &'static str), MergedCop>,
}

/// Overrides supplied on the command line.
#[derive(Debug, Clone, Default)]
pub struct CliOverrides {
    /// Run only these cops. Each entry is a qualified name such as
    /// `Naming/RuleNameLength`, or a bare category such as `Naming`.
    pub only: Vec<String>,

    /// Run everything except these cops, in the same format as `only`.
    pub except: Vec<String>,
}

impl CliOverrides {
    /// True if neither list was supplied.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.only.is_empty() && self.except.is_empty()
    }
}

/// Build the starting state from the registry.
///
/// Every registered cop is present with its declared defaults. Parameters start
/// empty; validation fills in the ones the user did not set.
///
/// # Arguments
///
/// * `decls` - every cop declaration in the registry
#[must_use]
pub(crate) fn base(decls: &'static [CopDecl]) -> Merged {
    let mut cops = BTreeMap::new();
    for decl in decls {
        cops.insert(
            (decl.category, decl.name),
            MergedCop {
                decl,
                enabled: decl.default_enabled,
                enabled_origin: Origin::Default,
                severity: decl.default_severity.clone(),
                severity_origin: Origin::Default,
                params: BTreeMap::new(),
            },
        );
    }

    Merged {
        files: Vec::new(),
        globals: BTreeMap::new(),
        cops,
    }
}

/// Lay one configuration file over the current state.
///
/// # Arguments
///
/// * `merged` - state to update in place
/// * `raw` - the parsed file
/// * `source` - the file it was parsed from
/// * `diagnostics` - collector for non-fatal problems
pub(crate) fn apply(
    merged: &mut Merged,
    raw: &RawConfig,
    source: &ConfigSource,
    diagnostics: &mut Diagnostics,
) {
    let file = merged.files.len();
    merged.files.push(source.clone());

    apply_defaults(merged, raw, source, diagnostics);

    for (key, value) in &raw.globals {
        merged.globals.insert(
            key.clone(),
            Sourced {
                raw: value.clone(),
                file,
            },
        );
    }

    let known: Vec<String> = merged
        .cops
        .values()
        .map(|cop| cop.decl.qualified_name())
        .collect();
    let families: Vec<String> = {
        let mut names: Vec<String> = merged
            .cops
            .keys()
            .map(|(category, _)| category.to_string())
            .collect();
        names.dedup();
        names
    };

    for ((family, name), raw_cop) in &raw.cops {
        let Ok(category) = family.parse::<Category>() else {
            let span = entry_span(raw_cop);
            DiagnosticBuilder::warning(format!("unknown cop family `{family}`"))
                .at(&source.path, &source.text, span)
                .suggest(family, families.iter().map(String::as_str))
                .emit(diagnostics);
            continue;
        };

        let key = merged
            .cops
            .keys()
            .find(|(c, n)| *c == category && *n == name.as_str())
            .copied();

        let Some(key) = key else {
            let span = entry_span(raw_cop);
            DiagnosticBuilder::warning(format!("unknown cop `{family}/{name}`"))
                .at(&source.path, &source.text, span)
                .suggest(
                    &format!("{family}/{name}"),
                    known.iter().map(String::as_str),
                )
                .emit(diagnostics);
            continue;
        };

        let Some(cop) = merged.cops.get_mut(&key) else {
            continue;
        };

        if let Some(value) = &raw_cop.enabled
            && let Some(flag) = expect_bool(value, "enabled", source, diagnostics)
        {
            cop.enabled = flag;
            cop.enabled_origin = Origin::from_span(&source.path, &source.text, value.blame_value());
        }

        if let Some(value) = &raw_cop.severity
            && let Some(severity) = expect_severity(value, source, diagnostics)
        {
            cop.severity = severity;
            cop.severity_origin =
                Origin::from_span(&source.path, &source.text, value.blame_value());
        }

        for (param_key, param_value) in &raw_cop.params {
            cop.params.insert(
                param_key.clone(),
                Sourced {
                    raw: param_value.clone(),
                    file,
                },
            );
        }
    }
}

/// Apply the `[defaults]` section to every cop that the file does not override
/// individually.
///
/// # Arguments
///
/// * `merged` - state to update in place
/// * `raw` - the parsed file
/// * `source` - the file it was parsed from
/// * `diagnostics` - collector for non-fatal problems
fn apply_defaults(
    merged: &mut Merged,
    raw: &RawConfig,
    source: &ConfigSource,
    diagnostics: &mut Diagnostics,
) {
    if let Some(value) = &raw.defaults.enabled
        && let Some(flag) = expect_bool(value, "enabled", source, diagnostics)
    {
        let origin = Origin::from_span(&source.path, &source.text, value.blame_value());
        for cop in merged.cops.values_mut() {
            cop.enabled = flag;
            cop.enabled_origin = origin.clone();
        }
    }

    if let Some(value) = &raw.defaults.severity
        && let Some(severity) = expect_severity(value, source, diagnostics)
    {
        let origin = Origin::from_span(&source.path, &source.text, value.blame_value());
        for cop in merged.cops.values_mut() {
            cop.severity = severity.clone();
            cop.severity_origin = origin.clone();
        }
    }
}

/// Apply command-line cop selection, which wins over the configuration file.
///
/// `only` replaces the enabled set rather than intersecting with it: if it is
/// non-empty, everything not named is switched off.
///
/// # Arguments
///
/// * `merged` - state to update in place
/// * `cli` - the overrides to apply
/// * `diagnostics` - collector for non-fatal problems
pub(crate) fn apply_cli(merged: &mut Merged, cli: &CliOverrides, diagnostics: &mut Diagnostics) {
    let known: Vec<String> = {
        let mut names: Vec<String> = merged
            .cops
            .values()
            .map(|cop| cop.decl.qualified_name())
            .collect();
        for (category, _) in merged.cops.keys() {
            let name = category.to_string();
            if !names.contains(&name) {
                names.push(name);
            }
        }
        names
    };

    for selector in cli.only.iter().chain(cli.except.iter()) {
        if !known.iter().any(|name| name == selector) {
            DiagnosticBuilder::warning(format!("unknown cop selector `{selector}`"))
                .suggest(selector, known.iter().map(String::as_str))
                .emit(diagnostics);
        }
    }

    if !cli.only.is_empty() {
        for cop in merged.cops.values_mut() {
            cop.enabled = matches_any(cop.decl, &cli.only);
            cop.enabled_origin = Origin::CommandLine;
        }
    }

    for cop in merged.cops.values_mut() {
        if matches_any(cop.decl, &cli.except) {
            cop.enabled = false;
            cop.enabled_origin = Origin::CommandLine;
        }
    }
}

/// A span somewhere inside a cop entry, used to point at an entry whose own
/// heading cannot be located.
///
/// # Arguments
///
/// * `cop` - the entry to find a span within
fn entry_span(cop: &RawCop) -> Option<Range<usize>> {
    cop.enabled
        .as_ref()
        .or(cop.severity.as_ref())
        .or_else(|| cop.params.values().next())
        .and_then(RawValue::blame_key)
}

/// True if any selector names this cop, either by qualified name or by
/// category.
///
/// # Arguments
///
/// * `decl` - the cop to test
/// * `selectors` - qualified names or category names
fn matches_any(decl: &CopDecl, selectors: &[String]) -> bool {
    let qualified = decl.qualified_name();
    let category = decl.category.to_string();
    selectors
        .iter()
        .any(|selector| *selector == qualified || *selector == category)
}

/// Require that a raw value is a boolean.
///
/// # Arguments
///
/// * `value` - the value to check
/// * `key` - the key it was found under, for the message
/// * `source` - the file it came from, for diagnostics
/// * `diagnostics` - collector for non-fatal problems
fn expect_bool(
    value: &RawValue,
    key: &str,
    source: &ConfigSource,
    diagnostics: &mut Diagnostics,
) -> Option<bool> {
    match value.value {
        ParsedValue::Bool(flag) => Some(flag),
        ref other => {
            DiagnosticBuilder::error(format!(
                "`{key}` must be a boolean, found {}",
                other.type_name()
            ))
            .at(&source.path, &source.text, value.blame_value())
            .emit(diagnostics);
            None
        }
    }
}

/// Require that a raw value names a severity.
///
/// # Arguments
///
/// * `value` - the value to check
/// * `source` - the file it came from, for diagnostics
/// * `diagnostics` - collector for non-fatal problems
fn expect_severity(
    value: &RawValue,
    source: &ConfigSource,
    diagnostics: &mut Diagnostics,
) -> Option<Severity> {
    match &value.value {
        ParsedValue::Str(name) => match parse_severity(name) {
            Some(severity) => Some(severity),
            None => {
                DiagnosticBuilder::error(format!("`{name}` is not a severity"))
                    .at(&source.path, &source.text, value.blame_value())
                    .suggest(name, SEVERITY_NAMES.iter().copied())
                    .emit(diagnostics);
                None
            }
        },
        other => {
            DiagnosticBuilder::error(format!(
                "`severity` must be a string, found {}",
                other.type_name()
            ))
            .at(&source.path, &source.text, value.blame_value())
            .emit(diagnostics);
            None
        }
    }
}
