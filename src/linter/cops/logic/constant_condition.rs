//! Checks if the `condition:` block resolves to a boolean value.

use crate::linter::{
    analysis::const_fold::{
        FoldResult::{Constant, NotConstant},
        fold,
    },
    context::LintContext,
    cop::{Category, Cop},
    finding::{Finding, Severity},
};

/// Checks if the `condition:` block resolves to a boolean value.
pub struct LogicConstantCondition;

impl Cop for LogicConstantCondition {
    /// Returns the category of the cop
    fn category(&self) -> Category {
        Category::Logic
    }
    fn name(&self) -> &'static str {
        "ConstantCondition"
    }

    fn check(&self, context: &LintContext, findings: &mut Vec<Finding>) {
        for rule in &context.file.rules {
            let fold_result = fold(&rule.condition.expression);
            match fold_result {
                Constant(bool_result) => {
                    findings.push(Finding {
                        rule: self.name(),
                        category: self.category(),
                        message: format!(
                            "The condition in rule {} results in a unilateral {}",
                            rule.name, bool_result
                        ),
                        severity: Severity::Warning,
                    });
                }

                NotConstant => {
                    continue;
                }
            }
        }
    }
}
