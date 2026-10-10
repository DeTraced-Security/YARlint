//! Checking the merged configuration against what the registry declares.
//!
//! This is the only place that sees both the merged file contents and the
//! parameter specs. Four things happen here, all of them collecting into
//! diagnostics rather than returning early:
//!
//! 1. every key the user wrote is one the owning cop declared
//! 2. every value coerces to its declared type
//! 3. every integer falls inside its declared range
//! 4. every declared key the user did not write is filled in from its default
//!
//! Step four is what makes the resolved configuration complete, and therefore
//! what makes [`CopParams`](crate::config::params::CopParams) infallible.

use std::collections::BTreeMap;

use crate::config::diagnostics::{DiagnosticBuilder, Diagnostics};
use crate::config::globals::{GLOBALS_SPEC, Globals};
use crate::config::merge::{Merged, Sourced};
use crate::config::raw::ParsedValue;
use crate::config::resolved::{CopSettings, Origin, ResolvedConfig};
use crate::config::source::ConfigSource;
use crate::config::spec::{ParamKind, ParamSpec, ParamValue};

/// Validate a merged configuration and produce the effective one.
///
/// # Arguments
///
/// * `merged` - the merged state
/// * `diagnostics` - collector for non-fatal problems
#[must_use]
pub(crate) fn run(merged: Merged, diagnostics: &mut Diagnostics) -> ResolvedConfig {
    let files = merged.files;

    let (globals_values, _) = resolve_params(
        "globals",
        GLOBALS_SPEC,
        &merged.globals,
        &files,
        diagnostics,
    );
    let globals = Globals::from_params(&globals_values);

    let mut cops = BTreeMap::new();
    for (key, cop) in merged.cops {
        let owner = cop.decl.qualified_name();
        let (values, mut origins) =
            resolve_params(&owner, cop.decl.params, &cop.params, &files, diagnostics);

        origins.insert("enabled", cop.enabled_origin);
        origins.insert("severity", cop.severity_origin);

        cops.insert(
            key,
            CopSettings {
                decl: cop.decl,
                enabled: cop.enabled,
                severity: cop.severity,
                params: values,
                origins,
            },
        );
    }

    ResolvedConfig::new(globals, cops)
}

/// Resolve one owner's parameters against its specs.
///
/// Returns the complete value map and the origin of each value.
///
/// # Arguments
///
/// * `owner` - qualified cop name, or `globals`, used in messages
/// * `specs` - the parameters this owner declares
/// * `written` - what the user actually wrote, keyed as written
/// * `files` - every contributing file, for resolving spans
/// * `diagnostics` - collector for non-fatal problems
fn resolve_params(
    owner: &str,
    specs: &'static [ParamSpec],
    written: &BTreeMap<String, Sourced>,
    files: &[ConfigSource],
    diagnostics: &mut Diagnostics,
) -> (
    BTreeMap<&'static str, ParamValue>,
    BTreeMap<&'static str, Origin>,
) {
    for (key, sourced) in written {
        if specs.iter().any(|spec| spec.key == key.as_str()) {
            continue;
        }
        let source = &files[sourced.file];
        DiagnosticBuilder::warning(format!("unknown option `{key}` for {owner}"))
            .at(&source.path, &source.text, sourced.raw.blame_key())
            .suggest(key, specs.iter().map(|spec| spec.key))
            .emit(diagnostics);
    }

    let mut values = BTreeMap::new();
    let mut origins = BTreeMap::new();

    for spec in specs {
        match written.get(spec.key) {
            Some(sourced) => {
                let source = &files[sourced.file];
                match coerce(owner, spec, sourced, source, diagnostics) {
                    Some(value) => {
                        values.insert(spec.key, value);
                        origins.insert(
                            spec.key,
                            Origin::from_span(
                                &source.path,
                                &source.text,
                                sourced.raw.blame_value(),
                            ),
                        );
                    }
                    None => {
                        values.insert(spec.key, spec.kind.default_value());
                        origins.insert(spec.key, Origin::Default);
                    }
                }
            }
            None => {
                values.insert(spec.key, spec.kind.default_value());
                origins.insert(spec.key, Origin::Default);
            }
        }
    }

    (values, origins)
}

/// Convert one written value into its declared type, or report why it cannot
/// be converted.
///
/// # Arguments
///
/// * `owner` - qualified cop name, or `globals`, used in messages
/// * `spec` - the declaration to check against
/// * `sourced` - the written value
/// * `source` - the file it came from
/// * `diagnostics` - collector for non-fatal problems
fn coerce(
    owner: &str,
    spec: &ParamSpec,
    sourced: &Sourced,
    source: &ConfigSource,
    diagnostics: &mut Diagnostics,
) -> Option<ParamValue> {
    let span = sourced.raw.blame_value();
    let found = sourced.raw.value.type_name();

    match (&spec.kind, &sourced.raw.value) {
        (ParamKind::Uint { min, max, .. }, ParsedValue::Int(raw)) => {
            let Ok(value) = u64::try_from(*raw) else {
                DiagnosticBuilder::error(format!(
                    "`{}` for {owner} must be at least {min}, found {raw}",
                    spec.key
                ))
                .at(&source.path, &source.text, span)
                .emit(diagnostics);
                return None;
            };

            if value < *min || value > *max {
                DiagnosticBuilder::error(format!(
                    "`{}` for {owner} must be between {min} and {max}, found {value}",
                    spec.key
                ))
                .at(&source.path, &source.text, span)
                .emit(diagnostics);
                return None;
            }

            Some(ParamValue::Uint(value))
        }

        (ParamKind::Bool { .. }, ParsedValue::Bool(value)) => Some(ParamValue::Bool(*value)),

        (ParamKind::Str { .. }, ParsedValue::Str(value)) => Some(ParamValue::Str(value.clone())),

        (ParamKind::StrList { .. }, ParsedValue::StrList(value)) => {
            Some(ParamValue::StrList(value.clone()))
        }

        (ParamKind::Enum { variants, .. }, ParsedValue::Str(value)) => {
            if variants.contains(&value.as_str()) {
                Some(ParamValue::Str(value.clone()))
            } else {
                DiagnosticBuilder::error(format!(
                    "`{value}` is not a valid value for `{}` on {owner}",
                    spec.key
                ))
                .at(&source.path, &source.text, span)
                .help(format!("expected one of: {}", variants.join(", ")))
                .emit(diagnostics);
                None
            }
        }

        _ => {
            DiagnosticBuilder::error(format!(
                "`{}` for {owner} must be {}, found {found}",
                spec.key,
                spec.kind.type_name()
            ))
            .at(&source.path, &source.text, span)
            .emit(diagnostics);
            None
        }
    }
}
