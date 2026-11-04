//! The effective configuration, after merging and validation.
//!
//! Everything here is concrete. Every framework key has a value and every
//! declared parameter is present, whether the user mentioned it or not. Nothing
//! downstream needs to supply a fallback, which is what makes
//! [`CopParams`] infallible.

use std::collections::BTreeMap;
use std::fmt;
use std::ops::Range;
use std::path::{Path, PathBuf};

use crate::config::diagnostics::line_col;
use crate::config::globals::Globals;
use crate::config::params::CopParams;
use crate::config::spec::{CopDecl, ParamValue};
use crate::linter::{cop::Category, finding::Severity};

/// The severity names accepted in a configuration file.
pub const SEVERITY_NAMES: &[&str] = &["info", "warning", "error"];

/// Parse a severity name as written in a configuration file.
///
/// # Arguments
///
/// * `name` - the name to parse, case-insensitively
#[must_use]
pub fn parse_severity(name: &str) -> Option<Severity> {
    match name.to_ascii_lowercase().as_str() {
        "info" => Some(Severity::Info),
        "warning" => Some(Severity::Warning),
        "error" => Some(Severity::Error),
        _ => None,
    }
}

/// Render a severity the way it is written in a configuration file.
///
/// # Arguments
///
/// * `severity` - the severity to render
#[must_use]
pub fn severity_name(severity: &Severity) -> &'static str {
    match severity {
        Severity::Info => "info",
        Severity::Warning => "warning",
        Severity::Error => "error",
    }
}

/// Where a resolved value came from.
///
/// Recorded for every setting so that `config show` can explain itself. This is
/// the difference between "my config is not working" being a support thread and
/// being one command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Origin {
    /// No configuration file mentioned this key; the declared default was used.
    Default,

    /// A command-line flag set this key.
    CommandLine,

    /// A configuration file set this key.
    File {
        /// The file that set it.
        path: PathBuf,
        /// One-based line number.
        line: usize,
        /// One-based column number.
        column: usize,
    },
}

impl Origin {
    /// Build a [`Origin::File`] from a byte span, resolving line and column.
    ///
    /// Falls back to line 1 column 1 when the parser recorded no span.
    ///
    /// # Arguments
    ///
    /// * `path` - the file the value came from
    /// * `text` - full text of that file
    /// * `span` - byte range of the value, if known
    #[must_use]
    pub fn from_span(path: &Path, text: &str, span: Option<Range<usize>>) -> Self {
        let (line, column) = span.map_or((1, 1), |s| line_col(text, s.start));
        Origin::File {
            path: path.to_path_buf(),
            line,
            column,
        }
    }
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Origin::Default => f.write_str("default"),
            Origin::CommandLine => f.write_str("command line"),
            Origin::File { path, line, column } => {
                write!(f, "{}:{line}:{column}", path.display())
            }
        }
    }
}

/// Everything resolved for one cop.
#[derive(Debug, Clone)]
pub struct CopSettings {
    /// The cop's declaration, carried so that parameter lookups can assert
    /// against the keys the cop actually declared.
    pub decl: &'static CopDecl,

    /// Whether this cop should run.
    pub enabled: bool,

    /// Severity to stamp on this cop's findings.
    pub severity: Severity,

    /// Every declared parameter, always complete.
    pub params: BTreeMap<&'static str, ParamValue>,

    /// Where each setting came from, keyed by `enabled`, `severity`, or a
    /// parameter key.
    pub origins: BTreeMap<&'static str, Origin>,
}

impl CopSettings {
    /// A read handle for this cop's parameters.
    #[must_use]
    pub fn params(&self) -> CopParams<'_> {
        CopParams::new(&self.params, self.decl.params)
    }

    /// Where a setting came from, or [`Origin::Default`] if it was never
    /// recorded.
    ///
    /// # Arguments
    ///
    /// * `key` - `enabled`, `severity`, or a parameter key
    #[must_use]
    pub fn origin(&self, key: &str) -> Origin {
        self.origins.get(key).cloned().unwrap_or(Origin::Default)
    }
}

/// The effective configuration for a whole run.
#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    /// Resolved `[globals]` values.
    pub globals: Globals,

    /// Resolved settings for every registered cop, keyed by category and name.
    cops: BTreeMap<(Category, &'static str), CopSettings>,
}

impl ResolvedConfig {
    /// Build from already-resolved parts.
    ///
    /// # Arguments
    ///
    /// * `globals` - resolved global values
    /// * `cops` - resolved per-cop settings
    #[must_use]
    pub(crate) fn new(
        globals: Globals,
        cops: BTreeMap<(Category, &'static str), CopSettings>,
    ) -> Self {
        Self { globals, cops }
    }

    /// Settings for one cop, if it is registered.
    ///
    /// # Arguments
    ///
    /// * `category` - the cop's family
    /// * `name` - the cop's bare name
    #[must_use]
    pub fn get(&self, category: Category, name: &str) -> Option<&CopSettings> {
        self.cops
            .iter()
            .find_map(|(key, settings)| (key.0 == category && key.1 == name).then_some(settings))
    }

    /// Every cop's settings, ordered by category then name.
    pub fn iter(&self) -> impl Iterator<Item = &CopSettings> {
        self.cops.values()
    }

    /// Every enabled cop's settings, ordered by category then name.
    pub fn enabled(&self) -> impl Iterator<Item = &CopSettings> {
        self.cops.values().filter(|settings| settings.enabled)
    }
}

impl fmt::Display for ResolvedConfig {
    /// Render the effective configuration, annotated with where each value came
    /// from. This is the body of `yarlint config show`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[globals]")?;
        writeln!(
            f,
            "required_meta_keys = {:?}",
            self.globals.required_meta_keys
        )?;

        for settings in self.cops.values() {
            writeln!(f)?;
            writeln!(
                f,
                "[cops.{}.{}]",
                settings.decl.category, settings.decl.name
            )?;
            writeln!(
                f,
                "enabled = {}  # {}",
                settings.enabled,
                settings.origin("enabled")
            )?;
            writeln!(
                f,
                "severity = \"{}\"  # {}",
                severity_name(&settings.severity),
                settings.origin("severity")
            )?;
            for (key, value) in &settings.params {
                let rendered = match value {
                    ParamValue::Uint(v) => v.to_string(),
                    ParamValue::Bool(v) => v.to_string(),
                    ParamValue::Str(v) => format!("{v:?}"),
                    ParamValue::StrList(v) => format!("{v:?}"),
                };
                writeln!(f, "{key} = {rendered}  # {}", settings.origin(key))?;
            }
        }

        Ok(())
    }
}
