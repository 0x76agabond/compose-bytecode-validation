use std::fmt;

pub use crate::compose::{VirtualStorageLayout, VirtualStorageLayoutRecord};

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct StorageValidationInput {
    pub bytecode: Vec<u8>,
    pub virtual_storage_layout: VirtualStorageLayout,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct StorageLocation {
    pub slot: String,
    pub offset: u8,
    pub selector: String,
    pub pc: Option<usize>,
    pub symbolic_path: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct StorageCollision {
    pub location: StorageLocation,
    pub virtual_path: String,
    pub expected_type: String,
    pub observed_type: String,
    pub reason: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ValidatedVariable {
    pub location: StorageLocation,
    pub virtual_path: String,
    pub expected_type: String,
    pub observed_type: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct UncertainStorageScope {
    pub location: StorageLocation,
    pub virtual_path: Option<String>,
    pub reason: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct StorageDiagnostic {
    pub selector: String,
    pub pc: Option<usize>,
    pub symbolic_path: String,
    pub message: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct StorageValidationReport {
    pub collisions: Vec<StorageCollision>,
    pub validated_variables: Vec<ValidatedVariable>,
    pub uncertain_scopes: Vec<UncertainStorageScope>,
    pub diagnostics: Vec<StorageDiagnostic>,
}

impl StorageValidationReport {
    pub fn has_collisions(&self) -> bool {
        !self.collisions.is_empty()
    }
}

impl fmt::Display for StorageValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "[storage-validation] collisions={} validated={} uncertain_scopes={} diagnostics={}",
            self.collisions.len(),
            self.validated_variables.len(),
            self.uncertain_scopes.len(),
            self.diagnostics.len(),
        )?;
        for collision in &self.collisions {
            writeln!(
                f,
                "  collision slot={} offset={} selector={} pc={:?} expected={} observed={} path={} reason={}",
                collision.location.slot,
                collision.location.offset,
                collision.location.selector,
                collision.location.pc,
                collision.expected_type,
                collision.observed_type,
                collision.virtual_path,
                collision.reason,
            )?;
        }
        for variable in &self.validated_variables {
            writeln!(
                f,
                "  validated slot={} offset={} selector={} pc={:?} expected={} observed={} path={}",
                variable.location.slot,
                variable.location.offset,
                variable.location.selector,
                variable.location.pc,
                variable.expected_type,
                variable.observed_type,
                variable.virtual_path,
            )?;
        }
        for scope in &self.uncertain_scopes {
            writeln!(
                f,
                "  unresolved slot={} offset={} selector={} pc={:?} path={:?} reason={}",
                scope.location.slot,
                scope.location.offset,
                scope.location.selector,
                scope.location.pc,
                scope.virtual_path,
                scope.reason,
            )?;
        }
        Ok(())
    }
}
