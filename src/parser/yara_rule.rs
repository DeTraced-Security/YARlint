//! YARA rule representation.
//!
//! This module contains structures and constants used to represent
//! parsed YARA rules.

use std::collections::HashMap;
use std::path::PathBuf;

/// Reserved keywords recognized by the YARA language.
///
/// These keywords are used by the lexer to distinguish language
/// constructs from ordinary identifiers.
pub const KEYWORDS: &[&str] = &[
    "all",
    "and",
    "any",
    "ascii",
    "at",
    "base64",
    "base64wide",
    "condition",
    "contains",
    "endswith",
    "entrypoint",
    "false",
    "filesize",
    "for",
    "fullword",
    "global",
    "import",
    "icontains",
    "iendswith",
    "iequals",
    "in",
    "include",
    "int16",
    "int16be",
    "int32",
    "int32be",
    "int8",
    "int8be",
    "istartswith",
    "matches",
    "meta",
    "nocase",
    "none",
    "not",
    "of",
    "or",
    "private",
    "rule",
    "startswith",
    "strings",
    "them",
    "true",
    "uint16",
    "uint16be",
    "uint32",
    "uint32be",
    "uint8",
    "uint8be",
    "wide",
    "xor",
    "defined",
];

/// Represents a parsed YARA rule.
///
/// This structure stores the rule name and the contents of the
/// `meta` and `strings` sections. Additional sections and metadata
/// may be added as parser support expands.
pub struct YaraRule {
    /// The name of the YARA rule.
    pub name: String,

    /// The path to the source file containing the rule.
    pub path: PathBuf,

    /// Metadata entries defined in the rule's `meta` section.
    ///
    /// The key is the metadata identifier and the value is the
    /// associated metadata value.
    pub meta: HashMap<String, String>,

    /// String definitions defined in the rule's `strings` section.
    ///
    /// The key is the string identifier (for example `$a`) and
    /// the value is the corresponding string content.
    pub strings: HashMap<String, String>,
}

impl YaraRule {
    /// Creates a new empty YARA rule.
    ///
    /// The returned rule contains the supplied name, an empty path,
    /// and empty `meta` and `strings` collections.
    pub fn new(name: String) -> Self {
        Self {
            name,
            path: PathBuf::new(),
            meta: HashMap::new(),
            strings: HashMap::new(),
        }
    }
}
