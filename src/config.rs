//! Configuration: how YARlint decides which cops run and how they behave.
//!
//! The pipeline has two inputs, not one. The registry declares what parameters
//! exist and what their defaults are; the configuration file supplies overrides
//! only. Those are merged, validated, and resolved into a
//! [`ResolvedConfig`] in which every value is
//! concrete.
//!
//! ```text
//! registry decls ─┐
//!                 ├─> merge ─> validate ─> ResolvedConfig ─> cop fields
//! yarlint.toml ───┘
//! ```
//!
//! Each stage is a separate module because each has a different failure mode
//! and a different test setup:
//!
//! - [`raw`] fails on malformed TOML
//! - [`merge`] fails on wrong precedence
//! - [`validate`] fails on bad user input
//! - [`params`] cannot fail

use std::sync::OnceLock;

use crate::linter::cops::style::rule_name_case::NameCase;

pub mod diagnostics;
pub mod error;
pub mod globals;
pub mod merge;
pub mod params;
pub mod raw;
pub mod resolved;
pub mod source;
pub mod spec;
pub mod validate;

pub use diagnostics::Diagnostics;
pub use error::ConfigError;
pub use globals::Globals;
pub use merge::CliOverrides;
pub use params::CopParams;
pub use resolved::{CopSettings, Origin, ResolvedConfig};
pub use source::{ConfigSource, FsLoader, MapLoader, SourceLoader};
pub use spec::{CopDecl, ParamKind, ParamSpec, ParamValue};

/// Verbose setting. Sets it once if true, otherwise set to be false.
static VERBOSE: OnceLock<bool> = OnceLock::new();

/// Sets the verbose via arguments passed form command-line
pub fn init_verbose(v: bool) {
    VERBOSE.set(v).ok();
    if verbose() {
        println!("Verbose is set")
    }
}

/// Returns true if verbose is set, and false if not.
pub fn verbose() -> bool {
    *VERBOSE.get().unwrap_or(&false)
}

/// Returns the configured rule name casing convention.
///
/// Hardcoded until `Style/RuleNameCase` is migrated onto its declared `case`
/// parameter.
pub fn rule_name_case() -> NameCase {
    NameCase::PascalCase
}

/// Load the effective configuration.
///
/// Discovery, reading, merging, and validation, in that order. Problems inside
/// an otherwise readable file are collected into `diagnostics`; only a
/// missing-but-named file or malformed TOML is returned as an error.
///
/// When no configuration file is found, the registry defaults are returned
/// unchanged.
///
/// # Arguments
///
/// * `loader` - where configuration text comes from
/// * `decls` - every cop declaration in the registry
/// * `cli` - command-line cop selection, applied last
/// * `diagnostics` - collector for non-fatal problems
///
/// # Errors
///
/// Returns [`ConfigError::Io`] if the configuration file could not be read, or
/// [`ConfigError::Parse`] if it is not valid TOML.
pub fn load(
    loader: &dyn SourceLoader,
    decls: &'static [CopDecl],
    cli: &CliOverrides,
    diagnostics: &mut Diagnostics,
) -> Result<ResolvedConfig, ConfigError> {
    let mut merged = merge::base(decls);

    if let Some(path) = loader.discover()? {
        let source = loader.load(&path)?;
        let raw = raw::parse(&source, diagnostics)?;
        merge::apply(&mut merged, &raw, &source, diagnostics);
    }

    if !cli.is_empty() {
        merge::apply_cli(&mut merged, cli, diagnostics);
    }

    Ok(validate::run(merged, diagnostics))
}
