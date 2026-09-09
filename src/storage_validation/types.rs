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

/// A delegatecall that could not be followed with the available runtime data.
///
/// This remains non-blocking: the caller may have deployment-specific context
/// that is not available to static bytecode validation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct DelegateCallWarning {
    pub caller_selector: String,
    pub pc: usize,
    pub target: String,
    pub selector: Option<String>,
    pub reason: String,
}

/// Chain context shared by every recursive delegatecall trace.
///
/// `storage_address` remains the original diamond/proxy address because every
/// `DELEGATECALL` in the trace executes against that same storage state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DelegateCallValidationContext {
    pub storage_address: [u8; 20],
    pub max_depth: usize,
}

impl DelegateCallValidationContext {
    pub const fn new(storage_address: [u8; 20]) -> Self {
        Self {
            storage_address,
            max_depth: 8,
        }
    }
}

/// Supplies runtime code for delegatecall targets.
pub trait RuntimeCodeSource {
    fn code_at(&self, address: [u8; 20]) -> Result<Vec<u8>, String>;
    fn storage_at(&self, address: [u8; 20], slot: [u8; 32]) -> Result<[u8; 32], String>;
}

/// Minimal blocking JSON-RPC client used by the standalone validator.
///
/// The caller chooses a pinned block tag when constructing this source so all
/// recursive `eth_getCode` calls observe one chain state.
#[cfg(feature = "rpc")]
pub struct HttpRpcCodeSource {
    endpoint: String,
    block_tag: String,
    client: reqwest::blocking::Client,
}

#[cfg(feature = "rpc")]
impl HttpRpcCodeSource {
    pub fn new(endpoint: impl Into<String>, block_tag: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            block_tag: block_tag.into(),
            client: reqwest::blocking::Client::new(),
        }
    }
}

#[cfg(feature = "rpc")]
impl RuntimeCodeSource for HttpRpcCodeSource {
    fn code_at(&self, address: [u8; 20]) -> Result<Vec<u8>, String> {
        let address = format!("0x{}", alloy_primitives::hex::encode(address));
        let value = self.request("eth_getCode", serde_json::json!([address, self.block_tag]))?;
        let value = value
            .as_str()
            .ok_or_else(|| "eth_getCode response has no hex result".to_owned())?;
        let value = value.strip_prefix("0x").unwrap_or(value);
        alloy_primitives::hex::decode(value).map_err(|error| error.to_string())
    }

    fn storage_at(&self, address: [u8; 20], slot: [u8; 32]) -> Result<[u8; 32], String> {
        let address = format!("0x{}", alloy_primitives::hex::encode(address));
        let slot = format!("0x{}", alloy_primitives::hex::encode(slot));
        let value = self.request(
            "eth_getStorageAt",
            serde_json::json!([address, slot, self.block_tag]),
        )?;
        let value = value
            .as_str()
            .ok_or_else(|| "eth_getStorageAt response has no hex result".to_owned())?;
        let value = value.strip_prefix("0x").unwrap_or(value);
        let bytes = alloy_primitives::hex::decode(value).map_err(|error| error.to_string())?;
        bytes
            .try_into()
            .map_err(|_| "eth_getStorageAt did not return a 32-byte word".to_owned())
    }
}

#[cfg(feature = "rpc")]
impl HttpRpcCodeSource {
    fn request(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        let response: serde_json::Value = self
            .client
            .post(&self.endpoint)
            .json(&serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": method,
                "params": params,
            }))
            .send()
            .map_err(|error| error.to_string())?
            .error_for_status()
            .map_err(|error| error.to_string())?
            .json()
            .map_err(|error| error.to_string())?;
        if let Some(error) = response.get("error") {
            return Err(error.to_string());
        }
        response
            .get("result")
            .cloned()
            .ok_or_else(|| format!("{method} response has no result"))
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct StorageValidationReport {
    pub collisions: Vec<StorageCollision>,
    pub validated_variables: Vec<ValidatedVariable>,
    pub uncertain_scopes: Vec<UncertainStorageScope>,
    pub diagnostics: Vec<StorageDiagnostic>,
    pub delegatecall_warnings: Vec<DelegateCallWarning>,
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
            "[storage-validation] collisions={} validated={} uncertain_scopes={} diagnostics={} delegatecall_warnings={}",
            self.collisions.len(),
            self.validated_variables.len(),
            self.uncertain_scopes.len(),
            self.diagnostics.len(),
            self.delegatecall_warnings.len(),
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
        for warning in &self.delegatecall_warnings {
            writeln!(
                f,
                "  delegatecall-warning caller_selector={} pc={} target={} selector={:?} reason={}",
                warning.caller_selector,
                warning.pc,
                warning.target,
                warning.selector,
                warning.reason,
            )?;
        }
        Ok(())
    }
}
