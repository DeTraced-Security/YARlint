use crate::parser::syntax::{BinaryOperator, ExprNode, expr::NumberUnitType, UnaryOperator};

/// Result of attempting to constant-fold a boolean expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoldResult {
    Constant(bool),
    NotConstant,
}

/// Result of attempting to read a numeric value out of an expression node,
/// including unit conversion (KB/MB) where applicable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericValue {
    /// Successfully resolved to a concrete byte value.
    Value(u64),
    /// The node was numeric, but unit conversion overflowed `u64`.
    Overflow,
    /// The node isn't a numeric literal at all.
    NotNumeric,
}

pub fn fold(expr: &ExprNode) -> FoldResult {
    use FoldResult::{Constant, NotConstant};

    match expr {
        ExprNode::Identifier(name) => NotConstant,
        

        ExprNode::BoolLiteral(bool_type) => Constant(*bool_type),

        ExprNode::Group(inner) => fold(inner),

        ExprNode::Unary { operator, expression } => match operator {
            UnaryOperator::Not => match fold(expression) {
                Constant(b) => Constant(!b),
                NotConstant => NotConstant,
            },
        },

        ExprNode::Binary { left, operator, right } => match operator {
            BinaryOperator::And => fold_and(fold(left), fold(right)),
            BinaryOperator::Or => fold_or(fold(left), fold(right)),

            BinaryOperator::Equals
            | BinaryOperator::NotEquals
            | BinaryOperator::LessThan
            | BinaryOperator::LessThanEqual
            | BinaryOperator::GreaterThan
            | BinaryOperator::GreaterThanEqual => {
                match (numeric_value(left), numeric_value(right)) {
                    (NumericValue::Value(l), NumericValue::Value(r)) => {
                        Constant(match operator {
                            BinaryOperator::Equals => l == r,
                            BinaryOperator::NotEquals => l != r,
                            BinaryOperator::LessThan => l < r,
                            BinaryOperator::LessThanEqual => l <= r,
                            BinaryOperator::GreaterThan => l > r,
                            BinaryOperator::GreaterThanEqual => l >= r,
                            _ => unreachable!(), // guarded by the outer arm
                        })
                    }
                    // Overflow and NotNumeric both mean "can't fold this
                    // comparison" for boolean-folding purposes. Overflow
                    // specifically is Lint/NumericLiteralOverflow's concern,
                    // not this function's.
                    _ => NotConstant,
                }
            }

            BinaryOperator::Contains | BinaryOperator::Matches => NotConstant,
        },

        ExprNode::StringLiteral(_)
        | ExprNode::Number { .. }
        | ExprNode::FunctionCall { .. }
        | ExprNode::ModuleFunction { .. }
        | ExprNode::AllOf { .. }
        | ExprNode::AllOfThem
        | ExprNode::Of { .. } => NotConstant,

        ExprNode::Empty => NotConstant,
    }
}

fn fold_and(lhs: FoldResult, rhs: FoldResult) -> FoldResult {
    use FoldResult::{Constant, NotConstant};

    match (lhs, rhs) {
        (Constant(false), _) | (_, Constant(false)) => Constant(false),
        (Constant(true), Constant(true)) => Constant(true),
        _ => NotConstant,
    }
}

fn fold_or(lhs: FoldResult, rhs: FoldResult) -> FoldResult {
    use FoldResult::{Constant, NotConstant};

    match (lhs, rhs) {
        (Constant(true), _) | (_, Constant(true)) => Constant(true),
        (Constant(false), Constant(false)) => Constant(false),
        _ => NotConstant,
    }
}

/// Reads a numeric value out of an expression node, applying KB/MB unit
/// conversion per YARA's semantics (KB = ×1024, MB = ×2^20). Shared by
/// `fold`'s comparison folding and `Lint/NumericLiteralOverflow`.
pub fn numeric_value(expr: &ExprNode) -> NumericValue {
    match expr {
        ExprNode::Number { size, unit, .. } => apply_unit(*size, unit),
        _ => NumericValue::NotNumeric,
    }
}

fn apply_unit(size: usize, unit: &Option<NumberUnitType>) -> NumericValue {
    let size = size as u64;

    let result = match unit {
        None => Some(size),
        Some(NumberUnitType::Kilobyte) => size.checked_mul(1024),
        Some(NumberUnitType::Megabyte) => size.checked_mul(1_048_576),
    };

    match result {
        Some(value) => NumericValue::Value(value),
        None => NumericValue::Overflow,
    }
}