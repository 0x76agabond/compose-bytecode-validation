//! VSL-independent bytecode inference followed by physical layout comparison.

use super::{
    analyze_storage,
    matcher::{base_observation, report_status},
    types::{ComposeEngine, ComposeEngineReport, ComposeValidationInput, RawStorageObservation},
};

/// Runs EVMole inference without using VSL as an inference hint.
pub fn validate(input: &ComposeValidationInput) -> ComposeEngineReport {
    let observations = analyze_storage(&input.bytecode);
    validate_observations(input, &observations)
}

pub(crate) fn validate_observations(
    input: &ComposeValidationInput,
    raw_observations: &[RawStorageObservation],
) -> ComposeEngineReport {
    let observations = raw_observations
        .iter()
        .map(|raw| base_observation(raw, &input.virtual_storage_layout))
        .collect::<Vec<_>>();
    let status = report_status(&observations);

    ComposeEngineReport {
        engine: ComposeEngine::Compose,
        status,
        observations,
        assumptions: Vec::new(),
    }
}
