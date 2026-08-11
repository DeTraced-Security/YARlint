//! Checks that rules with heavy string patterns (wide strings, regex
//! strings) bound their scan scope with a `filesize` check in the
//! `condition:` block.

use crate::parser::syntax::{
    expr::ExprNode,
    rule::RuleNode,
    strings::{StringModifier, StringType},
};

use crate::linter::{
    context::LintContext,
    cop::{Category, Cop},
    finding::{Finding, Severity},
};

/// Checks that rules with heavy string patterns bound their scan scope
/// with a `filesize` check.
pub struct PerformanceFilesize;

impl PerformanceFilesize {
    /// Returns true if the rule declares at least one "heavy" string
    /// pattern: a `wide` modifier or a regular expression string. Plain
    /// text/hex strings without a `wide` modifier are not considered
    /// heavy on their own.
    fn has_heavy_string(rule: &RuleNode) -> bool {
        rule.strings.iter().any(|string| {
            string.modifiers.contains(&StringModifier::Wide)
                || matches!(string.value, StringType::RegEx(_))
        })
    }

    /// Recursively walks the condition's expression tree looking for a
    /// reference to the `filesize` identifier, anywhere in the tree.
    fn contains_filesize(expr: &ExprNode) -> bool {
        match expr {
            ExprNode::Identifier(name) => name == "filesize",

            ExprNode::Binary { left, right, .. } => {
                Self::contains_filesize(left) || Self::contains_filesize(right)
            }

            ExprNode::Unary { expression, .. } => Self::contains_filesize(expression),

            ExprNode::Group(inner) => Self::contains_filesize(inner),

            ExprNode::Of { count, .. } => Self::contains_filesize(count),

            ExprNode::FunctionCall { arguments, .. }
            | ExprNode::ModuleFunction { arguments, .. } => {
                arguments.iter().any(Self::contains_filesize)
            }

            // BoolLiteral, StringLiteral, Number, AllOf, AllOfThem, and
            // Empty have no nested expressions to walk into.
            _ => false,
        }
    }
}

impl Cop for PerformanceFilesize {
    fn name(&self) -> &'static str {
        "Filesize"
    }

    fn category(&self) -> Category {
        Category::Performance
    }

    fn check(&self, context: &LintContext, findings: &mut Vec<Finding>) {
        for rule in &context.file.rules {
            if !Self::has_heavy_string(rule) {
                continue;
            }

            if !Self::contains_filesize(&rule.condition.expression) {
                findings.push(Finding {
                    rule: self.name(),
                    category: self.category(),
                    message: format!(
                        "Rule '{}' uses a wide or regex string but has no `filesize` bound in its condition. \
                         Consider adding `and filesize < NMB` to limit scan scope.",
                        rule.name
                    ),
                    severity: Severity::Warning,
                });
            }
        }
    }
}
