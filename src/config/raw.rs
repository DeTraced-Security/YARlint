//! The literal contents of a configuration file.
//!
//! This layer answers "what did the user write", not "is it any good". Keys
//! stay as [`String`] because nothing has been checked against the registry
//! yet, and values are converted into a small local enum so that no other part
//! of the configuration module needs to know about TOML.
//!
//! Only a TOML syntax error is fatal here. Everything else, a `cops` key that
//! is not a table, a cop entry that is a string, is reported as a diagnostic
//! and skipped.

use std::collections::BTreeMap;
use std::ops::Range;

use toml_edit::{Document, Item, Value};

use crate::config::diagnostics::{DiagnosticBuilder, Diagnostics};
use crate::config::error::ConfigError;
use crate::config::source::ConfigSource;

/// A value as it appeared in the file, before any type checking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedValue {
    /// A TOML integer. Signed, because TOML integers are.
    Int(i64),
    /// A TOML boolean.
    Bool(bool),
    /// A TOML string.
    Str(String),
    /// A TOML array whose elements are all strings.
    StrList(Vec<String>),
    /// Anything else, carrying a human-readable name for diagnostics.
    Other(&'static str),
}

impl ParsedValue {
    /// Human-readable type name, used in diagnostics.
    #[must_use]
    pub const fn type_name(&self) -> &'static str {
        match self {
            ParsedValue::Int(_) => "an integer",
            ParsedValue::Bool(_) => "a boolean",
            ParsedValue::Str(_) => "a string",
            ParsedValue::StrList(_) => "an array of strings",
            ParsedValue::Other(name) => name,
        }
    }
}

/// A value together with the spans needed to point at it in an error message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawValue {
    /// The value itself.
    pub value: ParsedValue,

    /// Byte range of the key, used when the key is what is wrong.
    pub key_span: Option<Range<usize>>,

    /// Byte range of the value, used when the value is what is wrong.
    pub value_span: Option<Range<usize>>,
}

impl RawValue {
    /// The span to blame for a problem with the value.
    ///
    /// Falls back to the key's span when the value has none.
    #[must_use]
    pub fn blame_value(&self) -> Option<Range<usize>> {
        self.value_span.clone().or_else(|| self.key_span.clone())
    }

    /// The span to blame for a problem with the key.
    ///
    /// Falls back to the value's span when the key has none.
    #[must_use]
    pub fn blame_key(&self) -> Option<Range<usize>> {
        self.key_span.clone().or_else(|| self.value_span.clone())
    }
}

/// One cop's entry, exactly as written.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RawCop {
    /// The `enabled` key, if present.
    pub enabled: Option<RawValue>,

    /// The `severity` key, if present.
    pub severity: Option<RawValue>,

    /// Every other key, unchecked.
    pub params: BTreeMap<String, RawValue>,
}

/// The `[defaults]` section, exactly as written.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RawDefaults {
    /// Default `enabled` for every cop, if present.
    pub enabled: Option<RawValue>,

    /// Default `severity` for every cop, if present.
    pub severity: Option<RawValue>,
}

/// One configuration file, parsed but not checked.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RawConfig {
    /// The `[defaults]` section.
    pub defaults: RawDefaults,

    /// The `[globals]` section, unchecked.
    pub globals: BTreeMap<String, RawValue>,

    /// Every cop entry, keyed by `(family, name)` as written.
    pub cops: BTreeMap<(String, String), RawCop>,
}

/// Top-level keys this format recognises.
const TOP_LEVEL_KEYS: &[&str] = &["defaults", "globals", "cops"];

/// Keys that belong to the framework rather than to a cop's own parameters.
const FRAMEWORK_KEYS: &[&str] = &["enabled", "severity"];

/// Parse one configuration file.
///
/// # Arguments
///
/// * `source` - the file to parse
/// * `diagnostics` - collector for non-fatal problems
///
/// # Errors
///
/// Returns [`ConfigError::Parse`] if the text is not valid TOML. Structural
/// problems within valid TOML are reported through `diagnostics` instead.
pub fn parse(
    source: &ConfigSource,
    diagnostics: &mut Diagnostics,
) -> Result<RawConfig, ConfigError> {
    let document = Document::parse(source.text.as_str()).map_err(|err| ConfigError::Parse {
        path: source.path.clone(),
        message: err.message().to_owned(),
        span: err.span(),
    })?;

    let root = document.as_table();
    let mut config = RawConfig::default();

    for (key, item) in root.iter() {
        match key {
            "defaults" => {
                if let Some(table) = expect_table(root, key, item, source, diagnostics) {
                    config.defaults = parse_defaults(table, source, diagnostics);
                }
            }
            "globals" => {
                if let Some(table) = expect_table(root, key, item, source, diagnostics) {
                    config.globals = parse_kv_table(table);
                }
            }
            "cops" => {
                if let Some(table) = expect_table(root, key, item, source, diagnostics) {
                    config.cops = parse_cops(table, source, diagnostics);
                }
            }
            other => {
                DiagnosticBuilder::warning(format!("unknown top-level key `{other}`"))
                    .at(&source.path, &source.text, key_span(root, other))
                    .suggest(other, TOP_LEVEL_KEYS.iter().copied())
                    .emit(diagnostics);
            }
        }
    }

    Ok(config)
}

/// Parse the `[defaults]` section.
///
/// # Arguments
///
/// * `table` - the `[defaults]` table
/// * `source` - the file being parsed, for diagnostics
/// * `diagnostics` - collector for non-fatal problems
fn parse_defaults(
    table: &toml_edit::Table,
    source: &ConfigSource,
    diagnostics: &mut Diagnostics,
) -> RawDefaults {
    let mut defaults = RawDefaults::default();

    for (key, item) in table.iter() {
        let raw = raw_value(table, key, item);
        match key {
            "enabled" => defaults.enabled = Some(raw),
            "severity" => defaults.severity = Some(raw),
            other => {
                DiagnosticBuilder::warning(format!("unknown key `{other}` in `[defaults]`"))
                    .at(&source.path, &source.text, raw.blame_key())
                    .suggest(other, FRAMEWORK_KEYS.iter().copied())
                    .emit(diagnostics);
            }
        }
    }

    defaults
}

/// Parse the `[cops]` table into `(family, name)` entries.
///
/// # Arguments
///
/// * `table` - the `[cops]` table
/// * `source` - the file being parsed, for diagnostics
/// * `diagnostics` - collector for non-fatal problems
fn parse_cops(
    table: &toml_edit::Table,
    source: &ConfigSource,
    diagnostics: &mut Diagnostics,
) -> BTreeMap<(String, String), RawCop> {
    let mut cops = BTreeMap::new();

    for (family, family_item) in table.iter() {
        let Some(family_table) = expect_table(table, family, family_item, source, diagnostics)
        else {
            continue;
        };

        for (name, cop_item) in family_table.iter() {
            let Some(cop_table) = expect_table(family_table, name, cop_item, source, diagnostics)
            else {
                continue;
            };

            let mut cop = RawCop::default();
            for (key, item) in cop_table.iter() {
                let raw = raw_value(cop_table, key, item);
                match key {
                    "enabled" => cop.enabled = Some(raw),
                    "severity" => cop.severity = Some(raw),
                    other => {
                        cop.params.insert(other.to_owned(), raw);
                    }
                }
            }

            cops.insert((family.to_owned(), name.to_owned()), cop);
        }
    }

    cops
}

/// Collect a flat table of key/value pairs without interpreting them.
///
/// # Arguments
///
/// * `table` - the table to collect
fn parse_kv_table(table: &toml_edit::Table) -> BTreeMap<String, RawValue> {
    table
        .iter()
        .map(|(key, item)| (key.to_owned(), raw_value(table, key, item)))
        .collect()
}

/// Require that an item is a table, reporting a diagnostic if it is not.
///
/// # Arguments
///
/// * `parent` - the table the item was found in, used to locate the key's span
/// * `key` - the key the item was found under
/// * `item` - the item to check
/// * `source` - the file being parsed, for diagnostics
/// * `diagnostics` - collector for non-fatal problems
fn expect_table<'a>(
    parent: &'a toml_edit::Table,
    key: &str,
    item: &'a Item,
    source: &ConfigSource,
    diagnostics: &mut Diagnostics,
) -> Option<&'a toml_edit::Table> {
    match item.as_table() {
        Some(table) => Some(table),
        None => {
            let found = describe(item);
            DiagnosticBuilder::error(format!("`{key}` must be a table, found {found}"))
                .at(&source.path, &source.text, key_span(parent, key))
                .emit(diagnostics);
            None
        }
    }
}

/// Build a [`RawValue`] for one key/item pair.
///
/// # Arguments
///
/// * `parent` - the table the item was found in, used to locate the key's span
/// * `key` - the key the item was found under
/// * `item` - the item to convert
fn raw_value(parent: &toml_edit::Table, key: &str, item: &Item) -> RawValue {
    RawValue {
        value: convert(item),
        key_span: key_span(parent, key),
        value_span: item.span(),
    }
}

/// The byte range of a key within its parent table, if the parser recorded one.
///
/// # Arguments
///
/// * `parent` - the table to look the key up in
/// * `key` - the key to locate
fn key_span(parent: &toml_edit::Table, key: &str) -> Option<Range<usize>> {
    parent.key(key).and_then(toml_edit::Key::span)
}

/// Convert a TOML item into a [`ParsedValue`].
///
/// # Arguments
///
/// * `item` - the item to convert
fn convert(item: &Item) -> ParsedValue {
    match item {
        Item::Value(Value::Integer(v)) => ParsedValue::Int(*v.value()),
        Item::Value(Value::Boolean(v)) => ParsedValue::Bool(*v.value()),
        Item::Value(Value::String(v)) => ParsedValue::Str(v.value().clone()),
        Item::Value(Value::Array(array)) => {
            let mut list = Vec::with_capacity(array.len());
            for element in array.iter() {
                match element.as_str() {
                    Some(s) => list.push(s.to_owned()),
                    None => return ParsedValue::Other("an array with non-string elements"),
                }
            }
            ParsedValue::StrList(list)
        }
        other => ParsedValue::Other(describe(other)),
    }
}

/// Human-readable description of a TOML item, for diagnostics.
///
/// # Arguments
///
/// * `item` - the item to describe
fn describe(item: &Item) -> &'static str {
    match item {
        Item::None => "nothing",
        Item::Table(_) => "a table",
        Item::ArrayOfTables(_) => "an array of tables",
        Item::Value(value) => match value {
            Value::String(_) => "a string",
            Value::Integer(_) => "an integer",
            Value::Float(_) => "a float",
            Value::Boolean(_) => "a boolean",
            Value::Datetime(_) => "a datetime",
            Value::Array(_) => "an array",
            Value::InlineTable(_) => "a table",
        },
    }
}
