//! The read handle a cop receives when it is built.
//!
//! Every accessor is infallible. Validation has already materialised every
//! declared parameter into the resolved map, range-checked and typed, so there
//! is no "key missing" case and no "wrong type" case left to handle.
//!
//! The only way an accessor can miss is if a cop asks for a key it did not
//! declare, which is a bug in YARlint rather than in the user's configuration.
//! Those are caught by `debug_assert!` and degrade to the declared default in
//! release builds, because the release profile aborts on panic.

use std::collections::BTreeMap;

use crate::config::spec::{ParamKind, ParamSpec, ParamValue};

/// Read access to one cop's resolved parameters.
#[derive(Debug, Clone, Copy)]
pub struct CopParams<'a> {
    /// Resolved values, keyed by spec key.
    values: &'a BTreeMap<&'static str, ParamValue>,

    /// The declaring cop's specs, used to assert lookups and to supply a
    /// fallback if one ever misses.
    specs: &'static [ParamSpec],
}

impl<'a> CopParams<'a> {
    /// Wrap a resolved parameter map.
    ///
    /// # Arguments
    ///
    /// * `values` - resolved values, keyed by spec key
    /// * `specs` - the declaring cop's parameter specs
    #[must_use]
    pub(crate) fn new(
        values: &'a BTreeMap<&'static str, ParamValue>,
        specs: &'static [ParamSpec],
    ) -> Self {
        Self { values, specs }
    }

    /// Read an unsigned integer parameter.
    ///
    /// # Arguments
    ///
    /// * `key` - the parameter key, which must be one this cop declared
    #[must_use]
    pub fn uint(&self, key: &str) -> u64 {
        self.assert_declared(key);
        match self.values.get(key) {
            Some(ParamValue::Uint(value)) => *value,
            other => {
                debug_assert!(false, "`{key}` is not an integer parameter: {other:?}");
                match self.spec(key).map(|spec| spec.kind) {
                    Some(ParamKind::Uint { default, .. }) => default,
                    _ => 0,
                }
            }
        }
    }

    /// Read a boolean parameter.
    ///
    /// # Arguments
    ///
    /// * `key` - the parameter key, which must be one this cop declared
    #[must_use]
    pub fn bool(&self, key: &str) -> bool {
        self.assert_declared(key);
        match self.values.get(key) {
            Some(ParamValue::Bool(value)) => *value,
            other => {
                debug_assert!(false, "`{key}` is not a boolean parameter: {other:?}");
                match self.spec(key).map(|spec| spec.kind) {
                    Some(ParamKind::Bool { default }) => default,
                    _ => false,
                }
            }
        }
    }

    /// Read a string parameter.
    ///
    /// Also used for enum parameters, whose value is guaranteed by validation
    /// to be one of the declared variants.
    ///
    /// # Arguments
    ///
    /// * `key` - the parameter key, which must be one this cop declared
    #[must_use]
    pub fn str(&self, key: &str) -> &'a str {
        self.assert_declared(key);
        match self.values.get(key) {
            Some(ParamValue::Str(value)) => value.as_str(),
            other => {
                debug_assert!(false, "`{key}` is not a string parameter: {other:?}");
                match self.spec(key).map(|spec| spec.kind) {
                    Some(ParamKind::Str { default } | ParamKind::Enum { default, .. }) => default,
                    _ => "",
                }
            }
        }
    }

    /// Read a string list parameter.
    ///
    /// # Arguments
    ///
    /// * `key` - the parameter key, which must be one this cop declared
    #[must_use]
    pub fn str_list(&self, key: &str) -> &'a [String] {
        self.assert_declared(key);
        match self.values.get(key) {
            Some(ParamValue::StrList(value)) => value.as_slice(),
            other => {
                debug_assert!(false, "`{key}` is not a string list parameter: {other:?}");
                &[]
            }
        }
    }

    /// Look up the spec for a key.
    ///
    /// # Arguments
    ///
    /// * `key` - the parameter key
    fn spec(&self, key: &str) -> Option<&'static ParamSpec> {
        self.specs.iter().find(|spec| spec.key == key)
    }

    /// Assert that the cop declared the key it is asking for.
    ///
    /// # Arguments
    ///
    /// * `key` - the parameter key
    fn assert_declared(&self, key: &str) {
        debug_assert!(
            self.spec(key).is_some(),
            "`{key}` was not declared by this cop"
        );
    }
}
