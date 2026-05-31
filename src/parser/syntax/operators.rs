//! Operators used within YARA expressions.

/// Binary operators that operate on two expressions.
///
/// Examples:
/// - `a and b`
/// - `filesize < 100KB`
/// - `pe.imphash() == "..."`
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    /// Logical AND.
    And,

    /// Logical OR.
    Or,

    /// Equality comparison (`==`).
    Equals,

    /// Inequality comparison (`!=`).
    NotEquals,

    /// Less-than comparison (`<`).
    LessThan,

    /// Less-than-or-equal comparison (`<=`).
    LessThanEqual,

    /// Greater-than comparison (`>`).
    GreaterThan,

    /// Greater-than-or-equal comparison (`>=`).
    GreaterThanEqual,

    /// String containment check (`contains`).
    Contains,

    /// Regular expression match (`matches`).
    Matches,
}

/// Unary operators that operate on a single expression.
///
/// Examples:
/// - `not $a`
/// - `not defined pe.entry_point`
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    /// Logical negation.
    Not,
}
