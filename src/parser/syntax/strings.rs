//! String declarations contained within a YARA rule.

/// Represents a string definition within a YARA rule's `strings` section.
///
/// A string definition consists of an identifier, a value, and zero or more
/// modifiers that influence how the string is matched.
#[derive(Debug, Clone)]
pub struct StringNode {
    /// String identifier.
    ///
    /// Example:
    /// `$s1`
    pub identifier: String,

    /// String value.
    ///
    /// In the future we look to support all the different types of strings that
    /// YARA can use: Text, Hex, and Regex
    ///
    /// Example:
    /// `"cmd.exe"`
    pub value: String,

    /// Modifiers applied to the string.
    ///
    /// Examples:
    /// `ascii`
    /// `wide`
    /// `fullword`
    pub modifiers: Vec<StringModifier>,
}

/// Modifiers that alter how a YARA string is interpreted and matched.
#[derive(Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum StringModifier {
    /// Match the string as ASCII text.
    Ascii,

    /// Match the string as UTF-16 little-endian text.
    Wide,

    /// Require matches to occur on word boundaries.
    Fullword,

    /// Perform case-insensitive matching.
    Nocase,

    /// Match XOR-encoded variants of the string.
    Xor,

    /// Match the Base64-encoded representation of the string.
    Base64,

    /// Match the UTF-16 representation of the Base64-encoded string.
    Base64Wide,
}
