//! String declarations contained within a YARA rule.

use crate::parser::syntax::hex::HexNode;

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
    /// Example:
    /// `"cmd.exe"`
    pub value: StringType,

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

/// Encapsulates the three different types of string values in a YARA Rule
///
/// There is still much to implement, though that will all most likely be
/// https://yara.readthedocs.io/en/latest/writingrules.html#strings
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum StringType {
    /// String value.
    ///
    /// Example:
    /// `"foo.exe"`
    Text(String),

    /// Hex Value.
    ///
    /// Example:
    /// `{ a6 f8 ?d 82 }`
    Hex(HexNode),

    /// Regular Expression
    ///
    /// Example:
    /// `/md5: [0-9a-fA-F]{32}/`
    RegEx(String),
}
