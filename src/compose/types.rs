use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum VirtualStorageLayoutKind {
    Normal,
    Immutable,
}

impl fmt::Display for VirtualStorageLayoutKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal => f.write_str("normal"),
            Self::Immutable => f.write_str("immutable"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum VirtualStorageLayoutSource {
    Erc8042,
    Erc7201,
    SlotAssignment,
    ImplicitState,
}

impl fmt::Display for VirtualStorageLayoutSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Erc8042 => f.write_str("erc8042"),
            Self::Erc7201 => f.write_str("erc7201"),
            Self::SlotAssignment => f.write_str("slot-assignment"),
            Self::ImplicitState => f.write_str("implicit-state"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct VirtualStorageLayoutRecord {
    pub id: String,
    pub virtual_path: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub parent_virtual_path: Option<String>,
    pub kind: VirtualStorageLayoutKind,
    pub code_width: u8,
    pub layout: Vec<String>,
    pub serialized_layout: Vec<String>,
    pub slots: Vec<Vec<u16>>,
    pub source: VirtualStorageLayoutSource,
    pub source_name: String,
    pub contract_name: String,
    pub struct_name: Option<String>,
    pub diamond_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct VirtualStorageLayout {
    pub records: Vec<VirtualStorageLayoutRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ComposeValidationInput {
    pub bytecode: Vec<u8>,
    pub virtual_storage_layout: VirtualStorageLayout,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum ComposeEngine {
    Compose,
    ComposeVslBias,
}

impl fmt::Display for ComposeEngine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compose => f.write_str("compose"),
            Self::ComposeVslBias => f.write_str("compose-vsl-bias"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum ComposeValidationStatus {
    NoContradiction,
    Contradiction,
    Uncertain,
}

impl fmt::Display for ComposeValidationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoContradiction => f.write_str("no-contradiction"),
            Self::Contradiction => f.write_str("contradiction"),
            Self::Uncertain => f.write_str("uncertain"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum InferenceSource {
    Bytecode,
    VslBias,
    BytecodeAndVsl,
    Unknown,
}

impl fmt::Display for InferenceSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bytecode => f.write_str("bytecode"),
            Self::VslBias => f.write_str("vsl-bias"),
            Self::BytecodeAndVsl => f.write_str("bytecode+vsl"),
            Self::Unknown => f.write_str("unknown"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum ObservationVerdict {
    NoPhysicalContradiction,
    Contradiction,
    Uncertain,
    Unmapped,
}

impl fmt::Display for ObservationVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoPhysicalContradiction => f.write_str("no-physical-contradiction"),
            Self::Contradiction => f.write_str("contradiction"),
            Self::Uncertain => f.write_str("uncertain"),
            Self::Unmapped => f.write_str("unmapped"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct VslSlotMatch {
    pub record_id: String,
    pub virtual_path: String,
    pub slot_index: usize,
    pub expected_offset: Option<u8>,
    pub expected_width: Option<u16>,
    pub expected_type: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct StorageObservation {
    pub slot: String,
    pub symbolic_path: String,
    pub offset: u8,
    pub inferred_type: String,
    pub candidate_types: Vec<String>,
    pub inferred_width: Option<u16>,
    pub mask: Option<String>,
    pub is_write: bool,
    pub reads: Vec<String>,
    pub writes: Vec<String>,
    pub vsl_match: Option<VslSlotMatch>,
    pub inference_source: InferenceSource,
    pub verdict: ObservationVerdict,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct VslBiasAssumption {
    pub slot: String,
    pub offset: u8,
    pub original_type: String,
    pub assumed_type: String,
    pub virtual_path: String,
    pub reason: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ComposeEngineReport {
    pub engine: ComposeEngine,
    pub status: ComposeValidationStatus,
    pub observations: Vec<StorageObservation>,
    pub assumptions: Vec<VslBiasAssumption>,
}

impl fmt::Display for ComposeEngineReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "[{}] status={} observations={} assumptions={}",
            self.engine,
            self.status,
            self.observations.len(),
            self.assumptions.len()
        )?;
        for observation in &self.observations {
            let matched = observation.vsl_match.as_ref().map_or_else(
                || "unmapped".to_owned(),
                |matched| {
                    format!(
                        "{} slot_index={} expected_offset={:?} expected_width={:?} expected_type={:?}",
                        matched.virtual_path,
                        matched.slot_index,
                        matched.expected_offset,
                        matched.expected_width,
                        matched.expected_type
                    )
                },
            );
            writeln!(
                f,
                "  slot={} path={} offset={} type={} candidates={:?} width={:?} mask={:?} write={} source={} verdict={} vsl={}",
                observation.slot,
                observation.symbolic_path,
                observation.offset,
                observation.inferred_type,
                observation.candidate_types,
                observation.inferred_width,
                observation.mask,
                observation.is_write,
                observation.inference_source,
                observation.verdict,
                matched
            )?;
        }
        for assumption in &self.assumptions {
            writeln!(
                f,
                "  assumption slot={} offset={} {} -> {} path={} reason={}",
                assumption.slot,
                assumption.offset,
                assumption.original_type,
                assumption.assumed_type,
                assumption.virtual_path,
                assumption.reason
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ComposeValidationReport {
    pub compose: ComposeEngineReport,
    pub compose_vsl_bias: ComposeEngineReport,
}

impl fmt::Display for ComposeValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.compose, self.compose_vsl_bias)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RawStorageObservation {
    pub slot: Option<[u8; 32]>,
    pub symbolic_path: String,
    pub offset: u8,
    pub inferred_type: String,
    pub candidate_types: Vec<String>,
    pub score: usize,
    pub reads: Vec<[u8; 4]>,
    pub writes: Vec<[u8; 4]>,
    pub is_write: bool,
    pub mask: Option<String>,
}
