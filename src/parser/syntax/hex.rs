//! AST representation of a parsed YARA hex string.

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Representation of a byte in a YARA hex string
///
/// YARA hex strings have various ways to represent bytes. They can be simple
/// such as `a5`, `f3`, and `e4`, which all are represented by the [`Byte(u8)`]
/// field. or they can have a `?` as a wildcard, and etc.
pub enum HexAtom {
    /// unsigned 8bit integer representation of a byte, converted from chars.
    Byte(u8),

    /// Represents a `??` byte.
    Wildcard,

    /// Either a byte that starts with `?` or ends with `?`. That character
    /// becomes [`None`]. The other character is the unsigned 8 bit integer
    /// representation of the char.
    NibbleWildcard {
        /// The high bit
        high: Option<u8>,
        ///The low bit
        low: Option<u8>,
    },

    /// A YARA jump, `[4]` is exactly four bytes, `[4-]` is a minimum of four
    /// bytes, and `[-]` is any number of bytes. If the `min` or `max` aren't
    /// specified then they are [`None`]. If it is an exact jump both `min` and
    /// `max` are the number
    Jump {
        /// Minimum size of jump
        min: Option<u32>,
        /// Maximum size of jump
        max: Option<u32>,
    },

    /// Similar to a logical OR operation, can be recursive. Looks like
    /// `( AA | BB )`
    Alternation(Vec<Vec<HexAtom>>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents a string of YARA hexadecimal values  as a vector of [`HexAtom`]s.
pub struct HexExprNode {
    /// The "string" of hex values
    pub atoms: Vec<HexAtom>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// A Hex node, contains both the original string for simple lints, and a
/// ['HexExprNode`] for AST parsed output that will be used for more complex
/// lints
pub struct HexNode {
    /// AST representation of the original hex string
    pub expression: HexExprNode,
    /// The original hex string from the YARA rule
    pub original_string: String,
}
