//! Experimental inference that uses VSL physical constraints to resolve ambiguity.

use super::{
    analyze_storage,
    matcher::{base_observation, report_status},
    types::{
        ComposeEngine, ComposeEngineReport, ComposeValidationInput, ComposeValidationStatus,
        InferenceSource, ObservationVerdict, RawStorageObservation, VslBiasAssumption,
    },
    vsl_semantics::is_virtual_struct_collapse,
};

/// Runs the experimental VSL-biased inference path.
pub fn validate(input: &ComposeValidationInput) -> ComposeEngineReport {
    let observations = analyze_storage(&input.bytecode);
    validate_observations(input, &observations)
}

pub(crate) fn validate_observations(
    input: &ComposeValidationInput,
    raw_observations: &[RawStorageObservation],
) -> ComposeEngineReport {
    let mut assumptions = Vec::new();
    let mut observations = raw_observations
        .iter()
        .map(|raw| {
            let mut observation = base_observation(raw, &input.virtual_storage_layout);
            let Some(matched) = observation.vsl_match.as_ref() else {
                return observation;
            };

            if observation.verdict == ObservationVerdict::NoPhysicalContradiction {
                observation.inference_source = InferenceSource::BytecodeAndVsl;
                return observation;
            }

            let Some(expected_type) = matched.expected_type.as_ref() else {
                return observation;
            };
            let expected_width = matched.expected_width;

            // EVMole's generic uint256 result often means that no narrowing mask was
            // selected. This branch deliberately tests whether the VSL slot width is
            // a useful tie-breaker. The assumption remains explicit in the report.
            let (assumed_type, reason) = if observation.inferred_type == "uint256"
                && expected_width.is_some_and(|width| width < 256)
            {
                (
                    expected_type.clone(),
                    "generic uint256 narrowed to the matching VSL semantic type",
                )
            } else if is_virtual_struct_collapse(&observation.inferred_type, expected_type) {
                (
                    expected_type.clone(),
                    "container value recovered from the matching VSL virtual struct",
                )
            } else {
                return observation;
            };
            assumptions.push(VslBiasAssumption {
                slot: observation.slot.clone(),
                offset: observation.offset,
                original_type: observation.inferred_type.clone(),
                assumed_type: assumed_type.clone(),
                virtual_path: matched.virtual_path.clone(),
                reason: reason.to_owned(),
            });
            observation.inferred_type = assumed_type;
            observation.inferred_width = expected_width;
            observation.inference_source = InferenceSource::VslBias;
            observation.verdict = ObservationVerdict::NoPhysicalContradiction;

            observation
        })
        .collect::<Vec<_>>();

    observations.sort_by(|left, right| (&left.slot, left.offset).cmp(&(&right.slot, right.offset)));
    let status = match report_status(&observations) {
        // A VSL-derived type can explain an EVMole collapse, but it cannot prove
        // that the emitted bytecode has the declared semantic shape.
        ComposeValidationStatus::NoContradiction if !assumptions.is_empty() => {
            ComposeValidationStatus::Uncertain
        }
        status => status,
    };

    ComposeEngineReport {
        engine: ComposeEngine::ComposeVslBias,
        status,
        observations,
        assumptions,
    }
}

#[cfg(test)]
mod tests {
    use super::validate_observations;
    use crate::compose::{
        ComposeValidationInput, ComposeValidationStatus, InferenceSource, VirtualStorageLayout,
        VirtualStorageLayoutKind, VirtualStorageLayoutRecord, VirtualStorageLayoutSource, compose,
        types::RawStorageObservation,
    };

    #[test]
    fn records_vsl_narrowing_as_an_assumption() {
        let input = ComposeValidationInput {
            bytecode: Vec::new(),
            virtual_storage_layout: VirtualStorageLayout {
                records: vec![VirtualStorageLayoutRecord {
                    id: "0x01".to_owned(),
                    virtual_path: "fixture.packed".to_owned(),
                    parent_virtual_path: None,
                    kind: VirtualStorageLayoutKind::Normal,
                    code_width: 1,
                    layout: vec!["0x01".to_owned()],
                    serialized_layout: vec!["0x01".to_owned(), "0x01".to_owned()],
                    slots: vec![vec![8]],
                    source: VirtualStorageLayoutSource::SlotAssignment,
                    source_name: "Fixture.sol".to_owned(),
                    contract_name: "Fixture".to_owned(),
                    struct_name: Some("Storage".to_owned()),
                    diamond_name: None,
                }],
            },
        };
        let observations = vec![RawStorageObservation {
            slot: Some({
                let mut slot = [0_u8; 32];
                slot[31] = 1;
                slot
            }),
            symbolic_path: "Plain(0x01)".to_owned(),
            offset: 0,
            inferred_type: "uint256".to_owned(),
            candidate_types: vec!["uint256".to_owned()],
            score: 0,
            reads: vec![[0x12, 0x34, 0x56, 0x78]],
            writes: Vec::new(),
            is_write: false,
            mask: None,
        }];

        let unbiased = compose::validate_observations(&input, &observations);
        let biased = validate_observations(&input, &observations);

        assert_eq!(unbiased.status, ComposeValidationStatus::Contradiction);
        assert_eq!(biased.status, ComposeValidationStatus::Uncertain);
        assert_eq!(biased.assumptions.len(), 1);
        assert_eq!(
            biased.observations[0].inference_source,
            InferenceSource::VslBias
        );
        assert_eq!(biased.observations[0].inferred_type, "bool");
    }

    #[test]
    fn records_mapping_struct_recovery_as_an_assumption() {
        let root = VirtualStorageLayoutRecord {
            id: "0x02".to_owned(),
            virtual_path: "fixture.mapping".to_owned(),
            parent_virtual_path: None,
            kind: VirtualStorageLayoutKind::Normal,
            code_width: 1,
            layout: vec!["0xf1".to_owned(), "0x53".to_owned(), "0xff".to_owned()],
            serialized_layout: Vec::new(),
            slots: vec![vec![256]],
            source: VirtualStorageLayoutSource::SlotAssignment,
            source_name: "Fixture.sol".to_owned(),
            contract_name: "Fixture".to_owned(),
            struct_name: Some("Storage".to_owned()),
            diamond_name: None,
        };
        let mut child = root.clone();
        child.id = "0x03".to_owned();
        child.virtual_path = "fixture.mapping.0".to_owned();
        child.parent_virtual_path = Some("fixture.mapping".to_owned());
        child.kind = VirtualStorageLayoutKind::Immutable;
        child.layout = vec!["0x2f".to_owned(), "0x01".to_owned()];
        child.slots = vec![vec![256], vec![8]];
        let input = ComposeValidationInput {
            bytecode: Vec::new(),
            virtual_storage_layout: VirtualStorageLayout {
                records: vec![root, child],
            },
        };
        let observations = vec![RawStorageObservation {
            slot: Some({
                let mut slot = [0_u8; 32];
                slot[31] = 2;
                slot
            }),
            symbolic_path: "Mapping { key_type: FixedBytes(4) }".to_owned(),
            offset: 0,
            inferred_type: "mapping(bytes4 => address)".to_owned(),
            candidate_types: vec!["mapping(bytes4 => address)".to_owned()],
            score: 0,
            reads: vec![[0x12, 0x34, 0x56, 0x78]],
            writes: Vec::new(),
            is_write: false,
            mask: None,
        }];

        let unbiased = compose::validate_observations(&input, &observations);
        let biased = validate_observations(&input, &observations);

        assert_eq!(unbiased.status, ComposeValidationStatus::Contradiction);
        assert_eq!(biased.status, ComposeValidationStatus::Uncertain);
        assert_eq!(biased.assumptions.len(), 1);
        assert_eq!(
            biased.observations[0].inferred_type,
            "mapping(bytes4 => virtual-struct(fixture.mapping.0))"
        );
    }
}
