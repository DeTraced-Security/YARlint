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

    /// A module-based function call expression.
    ///
    /// Example:
    /// `pe.imphash(0)`
    ///
    /// Represents a function call where the function is
    /// namespaced under a module (e.g. `pe`, `math`, `hash`).
    ModuleFunction {
        /// Module name (e.g. `pe`)
        module: String,

        /// Function name inside the module (e.g. `imphash`)
        function: String,

        /// Arguments passed to the function call.
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

    /// Matches all strings that satisfy the provided pattern.
    ///
    /// Example:
    /// `all of ($a*)`
    ///
    /// The pattern typically represents a string identifier prefix
    /// followed by a wildcard.
    AllOf {
        /// Pattern used to select matching strings.
        pattern: String,
    },

    /// Matches all strings declared within the rule.
    ///
    /// Example:
    /// `all of them`
    ///
    /// This expression evaluates to true only when every declared
    /// string in the rule matches.
    AllOfThem,

    /// Matches a specified number of strings from a pattern group.
    ///
    /// Examples:
    /// `1 of ($x*)`
    /// `3 of ($api*)`
    /// `10 of ($str*)`
    ///
    /// The count may itself be an expression.
    Of {
        /// Number of matching strings required.
        count: Box<ExprNode>,

        /// Pattern used to select candidate strings.
        pattern: String,
    },

    /// A parenthesized expression.
    ///
    /// Example:
    /// `(filesize < 1MB and $a)`
    ///
    /// Grouping expressions allows precedence to be overridden
    /// during evaluation and parsing.
    Group(Box<ExprNode>),

    /// Signifies an empty condition block for the linter to warn
    /// on. Without this the ast_parser would crash if it got to a condition
    /// block that was empty
    Empty,
}
