//! Tests the `condition:` block to see what it results in.

use crate::{
    linter::analysis::const_fold::FoldResult::{Constant, NotConstant},
    parser::syntax::{BinaryOperator, ExprNode, UnaryOperator, expr::NumberUnitType},
};

/// Result of attempting to constant-fold a boolean expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoldResult {
    /// Resolved either to `true` or `false`
    Constant(bool),
    /// Did not resolve to `true` or `false`
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

/// Tests the `condition:` block to see what it results in.
///
/// In order to see if a `condition:` block folds down to a boolean value
/// it must be checked by this function.
///
/// # Arguments
/// * `expr` (`&ExprNode`) - The expression node to be tested
pub fn fold(expr: &ExprNode) -> FoldResult {
    match expr {
        ExprNode::Identifier(_) => NotConstant,

        ExprNode::BoolLiteral(bool_type) => Constant(*bool_type),

        ExprNode::Group(inner) => fold(inner),

        ExprNode::Unary {
            operator,
            expression,
        } => match operator {
            UnaryOperator::Not => match fold(expression) {
                Constant(b) => Constant(!b),
                NotConstant => NotConstant,
            },
        },

        ExprNode::Binary {
            left,
            operator,
            right,
        } => match operator {
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

/// Folds an `and` statement
///
/// # Arguments
/// * `lhs` (`FoldResult`) - left hand side of the `and`
/// * `rhs` (`FoldResult`) - right hand side of the `and`
fn fold_and(lhs: FoldResult, rhs: FoldResult) -> FoldResult {
    match (lhs, rhs) {
        (Constant(false), _) | (_, Constant(false)) => Constant(false),
        (Constant(true), Constant(true)) => Constant(true),
        _ => NotConstant,
    }
}

/// Folds an `or` statement
///
/// # Arguments
/// * `lhs` (`FoldResult`) - left hand side of the `or`
/// * `rhs` (`FoldResult`) - right hand side of the `or`
fn fold_or(lhs: FoldResult, rhs: FoldResult) -> FoldResult {
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

/// Applies the unit to the number to return the number in bytes
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::syntax::ExprNode;

    fn bool_lit(b: bool) -> ExprNode {
        ExprNode::BoolLiteral(b)
    }

    fn number(size: usize, unit: Option<NumberUnitType>) -> ExprNode {
        ExprNode::Number {
            size,
            unit,
            original: size.to_string(),
        }
    }

    // ---- Group ----

    #[test]
    fn group_passes_through_constant_true() {
        let expr = ExprNode::Group(Box::new(bool_lit(true)));
        assert_eq!(fold(&expr), Constant(true));
    }

    #[test]
    fn group_passes_through_not_constant() {
        let expr = ExprNode::Group(Box::new(ExprNode::Identifier("filesize".into())));
        assert_eq!(fold(&expr), NotConstant);
    }

    // ---- Unary / Not ----

    #[test]
    fn not_inverts_constant_true() {
        let expr = ExprNode::Unary {
            operator: UnaryOperator::Not,
            expression: Box::new(bool_lit(true)),
        };
        assert_eq!(fold(&expr), Constant(false));
    }

    #[test]
    fn not_inverts_constant_false() {
        let expr = ExprNode::Unary {
            operator: UnaryOperator::Not,
            expression: Box::new(bool_lit(false)),
        };
        assert_eq!(fold(&expr), Constant(true));
    }

    #[test]
    fn not_of_not_constant_stays_not_constant() {
        let expr = ExprNode::Unary {
            operator: UnaryOperator::Not,
            expression: Box::new(ExprNode::Identifier("filesize".into())),
        };
        assert_eq!(fold(&expr), NotConstant);
    }

    // ---- Binary / And ----

    #[test]
    fn and_true_true_is_true() {
        let expr = ExprNode::Binary {
            left: Box::new(bool_lit(true)),
            operator: BinaryOperator::And,
            right: Box::new(bool_lit(true)),
        };
        assert_eq!(fold(&expr), Constant(true));
    }

    #[test]
    fn and_short_circuits_on_left_false() {
        let expr = ExprNode::Binary {
            left: Box::new(bool_lit(false)),
            operator: BinaryOperator::And,
            right: Box::new(ExprNode::Identifier("filesize".into())),
        };
        assert_eq!(fold(&expr), Constant(false));
    }

    #[test]
    fn and_short_circuits_on_right_false() {
        let expr = ExprNode::Binary {
            left: Box::new(ExprNode::Identifier("filesize".into())),
            operator: BinaryOperator::And,
            right: Box::new(bool_lit(false)),
        };
        assert_eq!(fold(&expr), Constant(false));
    }

    #[test]
    fn and_not_constant_when_unresolved() {
        let expr = ExprNode::Binary {
            left: Box::new(ExprNode::Identifier("filesize".into())),
            operator: BinaryOperator::And,
            right: Box::new(bool_lit(true)),
        };
        assert_eq!(fold(&expr), NotConstant);
    }

    // ---- Binary / Or (fill in the two branches the report shows as 0) ----

    #[test]
    fn or_short_circuits_on_left_true() {
        let expr = ExprNode::Binary {
            left: Box::new(bool_lit(true)),
            operator: BinaryOperator::Or,
            right: Box::new(ExprNode::Identifier("filesize".into())),
        };
        assert_eq!(fold(&expr), Constant(true));
    }

    #[test]
    fn or_both_false_is_false() {
        let expr = ExprNode::Binary {
            left: Box::new(bool_lit(false)),
            operator: BinaryOperator::Or,
            right: Box::new(bool_lit(false)),
        };
        assert_eq!(fold(&expr), Constant(false));
    }

    // ---- Binary / comparisons ----

    #[test]
    fn equals_true_when_values_match() {
        let expr = ExprNode::Binary {
            left: Box::new(number(100, None)),
            operator: BinaryOperator::Equals,
            right: Box::new(number(100, None)),
        };
        assert_eq!(fold(&expr), Constant(true));
    }

    #[test]
    fn not_equals_true_when_values_differ() {
        let expr = ExprNode::Binary {
            left: Box::new(number(100, None)),
            operator: BinaryOperator::NotEquals,
            right: Box::new(number(200, None)),
        };
        assert_eq!(fold(&expr), Constant(true));
    }

    #[test]
    fn less_than_true() {
        let expr = ExprNode::Binary {
            left: Box::new(number(1, None)),
            operator: BinaryOperator::LessThan,
            right: Box::new(number(2, None)),
        };
        assert_eq!(fold(&expr), Constant(true));
    }

    #[test]
    fn less_than_equal_true() {
        let expr = ExprNode::Binary {
            left: Box::new(number(2, None)),
            operator: BinaryOperator::LessThanEqual,
            right: Box::new(number(2, None)),
        };
        assert_eq!(fold(&expr), Constant(true));
    }

    #[test]
    fn greater_than_true() {
        let expr = ExprNode::Binary {
            left: Box::new(number(5, None)),
            operator: BinaryOperator::GreaterThan,
            right: Box::new(number(2, None)),
        };
        assert_eq!(fold(&expr), Constant(true));
    }

    #[test]
    fn greater_than_equal_true() {
        let expr = ExprNode::Binary {
            left: Box::new(number(2, None)),
            operator: BinaryOperator::GreaterThanEqual,
            right: Box::new(number(2, None)),
        };
        assert_eq!(fold(&expr), Constant(true));
    }

    #[test]
    fn comparison_with_kb_suffix_folds() {
        // 1 * 1024 == 1024
        let expr = ExprNode::Binary {
            left: Box::new(number(1, Some(NumberUnitType::Kilobyte))),
            operator: BinaryOperator::Equals,
            right: Box::new(number(1024, None)),
        };
        assert_eq!(fold(&expr), Constant(true));
    }

    #[test]
    fn comparison_not_constant_when_one_side_not_numeric() {
        let expr = ExprNode::Binary {
            left: Box::new(ExprNode::Identifier("filesize".into())),
            operator: BinaryOperator::GreaterThan,
            right: Box::new(number(200, None)),
        };
        assert_eq!(fold(&expr), NotConstant);
    }

    #[test]
    fn comparison_not_constant_on_overflow() {
        let expr = ExprNode::Binary {
            left: Box::new(number(usize::MAX, Some(NumberUnitType::Megabyte))),
            operator: BinaryOperator::Equals,
            right: Box::new(number(1, None)),
        };
        assert_eq!(fold(&expr), NotConstant);
    }

    // ---- Binary / Contains, Matches ----

    #[test]
    fn contains_is_never_constant() {
        let expr = ExprNode::Binary {
            left: Box::new(ExprNode::StringLiteral("abc".into())),
            operator: BinaryOperator::Contains,
            right: Box::new(ExprNode::StringLiteral("b".into())),
        };
        assert_eq!(fold(&expr), NotConstant);
    }

    #[test]
    fn matches_is_never_constant() {
        let expr = ExprNode::Binary {
            left: Box::new(ExprNode::StringLiteral("abc".into())),
            operator: BinaryOperator::Matches,
            right: Box::new(ExprNode::StringLiteral("a.*".into())),
        };
        assert_eq!(fold(&expr), NotConstant);
    }

    // ---- numeric_value / apply_unit ----

    #[test]
    fn numeric_value_none_unit_returns_value_unchanged() {
        assert_eq!(numeric_value(&number(500, None)), NumericValue::Value(500));
    }

    #[test]
    fn numeric_value_kilobyte_multiplies_by_1024() {
        assert_eq!(
            numeric_value(&number(2, Some(NumberUnitType::Kilobyte))),
            NumericValue::Value(2048)
        );
    }

    #[test]
    fn numeric_value_megabyte_multiplies_by_2_20() {
        assert_eq!(
            numeric_value(&number(1, Some(NumberUnitType::Megabyte))),
            NumericValue::Value(1_048_576)
        );
    }

    #[test]
    fn numeric_value_overflows_on_absurd_megabyte_literal() {
        assert_eq!(
            numeric_value(&number(usize::MAX, Some(NumberUnitType::Megabyte))),
            NumericValue::Overflow
        );
    }

    #[test]
    fn numeric_value_not_numeric_for_non_number_node() {
        assert_eq!(
            numeric_value(&ExprNode::Identifier("filesize".into())),
            NumericValue::NotNumeric
        );
    }
}
