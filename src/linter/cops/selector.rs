//! Selector is in charge of deciding which cops run in a scan.
use crate::linter::cop::{Category, Cop};
use std::str::FromStr;

/// A selector naming either a whole category or one specific cop,
/// as accepted by --only / --except.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CopSelector {
    /// Cop Category
    Category(Category),
    /// Singular fully qualified cop name
    Cop(String),
}

impl FromStr for CopSelector {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(category) = Category::from_str(s) {
            return Ok(CopSelector::Category(category));
        }
        if s.contains('/') {
            return Ok(CopSelector::Cop(s.to_string()));
        }
        Err(format!(
            "'{s}' is not a known category or a fully qualified cop name (expected Category/CopName)"
        ))
    }
}

/// Returns true if it can match a CopSelector to an existing Cop
fn matches_specific_cop(selector: &CopSelector, cop: &dyn Cop) -> bool {
    match selector {
        CopSelector::Cop(name) => *name == cop.qualified_name(),
        CopSelector::Category(_) => false,
    }
}

/// Returns true if it can match a CopSelector to an existing Category
fn matches_category(selector: &CopSelector, cop: &dyn Cop) -> bool {
    match selector {
        CopSelector::Category(c) => *c == cop.category(),
        CopSelector::Cop(_) => false,
    }
}

/// Error returned when the same specific cop appears in both --only and --except.
#[derive(Debug, PartialEq, Eq)]
pub struct ConflictingSelectorError(pub String);

/// Resolved the enabled cops
///
/// Takes the enabled cops, the cops/categories that are excepted, and the
/// cops/categories that are only allowed to run, and creates a list of cops to
/// use in the scan.
pub fn resolve_enabled_cops<'a>(
    all_cops: &[&'a dyn Cop],
    only: &[CopSelector],
    except: &[CopSelector],
) -> Result<Vec<&'a dyn Cop>, ConflictingSelectorError> {
    // Reject the same specific cop appearing in both lists outright,
    // rather than letting one silently win.
    for cop in all_cops {
        let in_only = only.iter().any(|s| matches_specific_cop(s, *cop));
        let in_except = except.iter().any(|s| matches_specific_cop(s, *cop));
        if in_only && in_except {
            return Err(ConflictingSelectorError(cop.qualified_name()));
        }
    }

    let enabled = all_cops
        .iter()
        .copied()
        .filter(|cop| {
            // A specific-cop selector, on either list, decides outright.
            if except.iter().any(|s| matches_specific_cop(s, *cop)) {
                return false;
            }
            if only.iter().any(|s| matches_specific_cop(s, *cop)) {
                return true;
            }

            // No specific selector applies, fall back to category level.
            if except.iter().any(|s| matches_category(s, *cop)) {
                return false;
            }
            only.is_empty() || only.iter().any(|s| matches_category(s, *cop))
        })
        .collect();

    Ok(enabled)
}
