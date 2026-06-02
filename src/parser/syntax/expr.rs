//! Expression nodes used in YARA condition blocks.

use crate::parser::syntax::operators::{BinaryOperator, UnaryOperator};

/// Represents an expression within a YARA rule condition.
///
/// Expressions form a tree structure and may consist of literals,
/// identifiers, function calls, unary operations, or binary operations.
#[derive(Debug, Clone)]
pub enum ExprNode {
    /// A named identifier such as `filesize`, `them`, or a rule name.
    Identifier(String),

    /// A string literal value.
    ///
    /// Example:
    /// `"hello"`
    StringLiteral(String),

    /// A numeric literal value.
    ///
    /// Examples:
    /// `100`
    /// `0x5a4d`
    /// `100KB`
    Number(String),

    /// A function call expression.
    ///
    /// Example:
    /// `uint16(0)`
    /// `pe.imphash()`
    FunctionCall {
        /// Name of the function being called.
        name: String,

        /// Arguments passed to the function.
        arguments: Vec<ExprNode>,
    },

    /// A binary operation involving two expressions.
    ///
    /// Example:
    /// `filesize < 100KB`
    Binary {
        /// Left-hand side expression.
        left: Box<ExprNode>,

        /// Operator applied between both expressions.
        operator: BinaryOperator,

        /// Right-hand side expression.
        right: Box<ExprNode>,
    },

    /// A unary operation applied to a single expression.
    ///
    /// Example:
    /// `not $a`
    Unary {
        /// Unary operator.
        operator: UnaryOperator,

        /// Expression the operator is applied to.
        expression: Box<ExprNode>,
    },

    // All of
    AllOf {
        pattern: String,
    },

    // all of them
    AllOfThem,

    // 1 of ($x*)
    Of {
        count: Box<ExprNode>,
        pattern: String,
    },

    // parenthesized expressions
    Group(Box<ExprNode>),
}
