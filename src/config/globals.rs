//! Configuration keys shared by more than one cop.
//!
//! Deliberately small. A key belongs here only when two or more cops genuinely
//! need to agree on it; duplicating it per cop is how the two cops end up
//! disagreeing. Adding a key should be a conscious edit to this file.

use std::collections::BTreeMap;

use crate::config::spec::{ParamKind, ParamSpec, ParamValue};

/// The `[globals]` section's declared keys.
pub const GLOBALS_SPEC: &[ParamSpec] = &[ParamSpec {
    key: "required_meta_keys",
    kind: ParamKind::StrList {
        default: &["author", "description", "reference", "date"],
    },
    doc: "Meta keys every rule must define, in the order they should appear.",
}];

/// Resolved values for the `[globals]` section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Globals {
    /// Meta keys every rule must define, in the order they should appear.
    ///
    /// Shared by `Style/MissingRequiredMeta` and `Style/MetaKeysOrder` so that
    /// the two cannot disagree about what "required" means.
    pub required_meta_keys: Vec<String>,
}

impl Globals {
    /// Build from a validated parameter map.
    ///
    /// Validation guarantees every declared key is present with the right
    /// variant, so a mismatch here means the spec and this struct have drifted.
    /// That is an authoring bug, not user input, so it falls back to the
    /// declared default rather than failing.
    ///
    /// # Arguments
    ///
    /// * `values` - validated values, keyed by spec key
    pub(crate) fn from_params(values: &BTreeMap<&'static str, ParamValue>) -> Self {
        let required_meta_keys = match values.get("required_meta_keys") {
            Some(ParamValue::StrList(list)) => list.clone(),
            _ => {
                debug_assert!(false, "globals spec and Globals struct have drifted");
                Vec::new()
            }
        };
        Self { required_meta_keys }
    }
}

impl Default for Globals {
    fn default() -> Self {
        let mut values = BTreeMap::new();
        for spec in GLOBALS_SPEC {
            values.insert(spec.key, spec.kind.default_value());
        }
        Self::from_params(&values)
    }
}
