//! Metadata entries contained within a YARA rule.

/// Node representing a single entry within a YARA rule's `meta` section.
///
/// A metadata entry consists of a key and an associated value. Values may
/// be strings, numbers, or boolean literals.
#[derive(Debug, Clone)]
pub struct MetaEntryNode {
    /// Metadata key.
    pub key: String,

    /// Metadata value.
    pub value: MetaValue,
}

/// Represents the possible value types for a YARA metadata entry.
#[derive(Debug, Clone)]
pub enum MetaValue {
    /// String value.
    ///
    /// Example:
    /// `"DeTraced Security"`
    String(String),

    /// Numeric value.
    ///
    /// Example:
    /// `2025`
    Number(String),

    /// Boolean value.
    ///
    /// Examples:
    /// `true`
    /// `false`
    Boolean(bool),
}
