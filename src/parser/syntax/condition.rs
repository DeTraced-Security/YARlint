//! Condition block representation.

use crate::parser::syntax::expr::ExprNode;

/// Node representing a YARA rule's `condition` section.
///
/// The condition section contains a single root expression that determines
/// whether the rule matches. This expression may consist of identifiers,
/// literals, function calls, logical operations, comparisons, and other
/// supported YARA condition constructs.
#[derive(Debug, Clone)]
pub struct ConditionNode {
    /// Root expression of the condition block.
    pub expression: ExprNode,
}
