//! VSL-driven validation of persistent bytecode writes.
//!
//! This module deliberately does not infer a replacement storage layout. It
//! compares EVMole's recovered write variables against the canonical VSL that
//! Compose supplies for the full diamond.

mod types;
mod vsl;

use crate::{
    Slot,
    arguments::function_arguments,
    selectors::function_selectors,
    storage::{StorageEvidence, contract_storage_with_hints},
};
use std::collections::BTreeMap;
use vsl::{
    SemanticCompatibility, compare_semantic_types, expected_type_at,
    has_container_shape_contradiction, storage_trace_hints,
};

pub use types::{
    StorageCollision, StorageDiagnostic, StorageLocation, StorageValidationInput,
    StorageValidationReport, UncertainStorageScope, ValidatedVariable, VirtualStorageLayout,
    VirtualStorageLayoutRecord,
};

#[derive(Clone, Debug)]
struct VslVariableMatch {
    virtual_path: String,
    expected_type: Option<String>,
    expected_width: Option<u16>,
}

type WriteEvidenceKey = (Option<Slot>, u8, [u8; 4], Option<usize>, String);

/// Validates persistent `SSTORE` evidence against the supplied full-diamond VSL.
pub fn validate(input: &StorageValidationInput) -> StorageValidationReport {
    let trace_hints = storage_trace_hints(&input.virtual_storage_layout);
    let functions = function_selectors(&input.bytecode, 0, None)
        .0
        .into_iter()
        .map(|(selector, (offset, _))| {
            let arguments = function_arguments(&input.bytecode, &selector, 0);
            (selector, offset, arguments)
        })
        .collect::<Vec<_>>();
    if std::env::var_os("COMPOSE_TRACE_STORAGE").is_some() {
        for (selector, _, arguments) in &functions {
            let arguments = arguments
                .iter()
                .map(|argument| argument.sol_type_name())
                .collect::<Vec<_>>();
            eprintln!(
                "[storage-validation:function] selector={} arguments={arguments:?}",
                hex(selector)
            );
        }
    }
    let layouts = contract_storage_with_hints(
        &input.bytecode,
        functions
            .iter()
            .map(|(selector, offset, arguments)| (*selector, *offset, arguments)),
        0,
        &trace_hints,
    );

    if std::env::var_os("COMPOSE_TRACE_STORAGE").is_some() {
        for evidence in &layouts.evidence {
            eprintln!(
                "[storage-validation:evidence] domain={} slot={:?} path={} offset={} type={} known={} score={} write={} pc={:?} selector={} fallback={} mask={:?}",
                evidence.domain,
                evidence.slot,
                evidence.symbolic_path,
                evidence.offset,
                evidence.inferred_type,
                evidence.value_type_known,
                evidence.score,
                evidence.is_write,
                evidence.write_pc,
                hex(&evidence.selector),
                evidence.is_fallback_probe,
                evidence.mask,
            );
        }
    }

    let mut report = StorageValidationReport::default();
    for evidence in best_write_evidence(&layouts.evidence) {
        validate_write(evidence, &input.virtual_storage_layout, &mut report);
    }
    report
}

fn best_write_evidence(evidence: &[StorageEvidence]) -> Vec<&StorageEvidence> {
    let mut best: BTreeMap<WriteEvidenceKey, &StorageEvidence> = BTreeMap::new();
    for item in evidence
        .iter()
        .filter(|item| item.domain == "persistent" && item.is_write && !item.is_fallback_probe)
    {
        let key = (
            item.slot,
            item.offset,
            item.selector,
            item.write_pc,
            item.symbolic_path.clone(),
        );
        match best.get(&key) {
            Some(current) if current.score >= item.score => {}
            _ => {
                best.insert(key, item);
            }
        }
    }
    best.into_values().collect()
}

fn validate_write(
    evidence: &StorageEvidence,
    layout: &VirtualStorageLayout,
    report: &mut StorageValidationReport,
) {
    let Some(slot) = evidence.slot else {
        report.diagnostics.push(StorageDiagnostic {
            selector: hex(&evidence.selector),
            pc: evidence.write_pc,
            symbolic_path: evidence.symbolic_path.clone(),
            message: "persistent write has no resolved storage root".to_owned(),
        });
        return;
    };
    let location = StorageLocation {
        slot: hex(&slot),
        offset: evidence.offset,
        selector: hex(&evidence.selector),
        pc: evidence.write_pc,
        symbolic_path: evidence.symbolic_path.clone(),
    };
    let Some(matched) = match_vsl_variable(evidence, layout) else {
        report.uncertain_scopes.push(UncertainStorageScope {
            location,
            virtual_path: None,
            reason: "write root is not declared by the canonical VSL".to_owned(),
        });
        return;
    };

    let Some(expected_type) = matched.expected_type else {
        report.uncertain_scopes.push(UncertainStorageScope {
            location,
            virtual_path: Some(matched.virtual_path),
            reason: "VSL type is unsupported by the current comparator".to_owned(),
        });
        return;
    };

    if expected_type.contains("virtual-struct(") {
        report.uncertain_scopes.push(UncertainStorageScope {
            location,
            virtual_path: Some(matched.virtual_path),
            reason: "container child write is known, but its VSL member path is not reconstructed"
                .to_owned(),
        });
        return;
    }

    if is_dynamic_array_length_write(evidence, &expected_type) {
        report.validated_variables.push(ValidatedVariable {
            location,
            virtual_path: matched.virtual_path,
            expected_type,
            observed_type: "dynamic-array length".to_owned(),
        });
        return;
    }

    if !evidence.value_type_known {
        if has_container_shape_contradiction(&evidence.inferred_type, &expected_type) {
            report.collisions.push(StorageCollision {
                location,
                virtual_path: matched.virtual_path,
                expected_type,
                observed_type: evidence.inferred_type.clone(),
                reason: "recovered container shape contradicts the VSL type".to_owned(),
            });
        } else {
            report.uncertain_scopes.push(UncertainStorageScope {
                location,
                virtual_path: Some(matched.virtual_path),
                reason: "write position is known, but its value type was not recovered".to_owned(),
            });
        }
        return;
    }

    let Some(expected_width) = matched.expected_width else {
        report.uncertain_scopes.push(UncertainStorageScope {
            location,
            virtual_path: Some(matched.virtual_path),
            reason: "VSL has no field at the recovered byte offset".to_owned(),
        });
        return;
    };
    let Some(observed_width) = inferred_width(&evidence.inferred_type) else {
        report.uncertain_scopes.push(UncertainStorageScope {
            location,
            virtual_path: Some(matched.virtual_path),
            reason: "bytecode write has no recoverable value width".to_owned(),
        });
        return;
    };

    if observed_width != expected_width {
        report.collisions.push(StorageCollision {
            location,
            virtual_path: matched.virtual_path,
            expected_type,
            observed_type: evidence.inferred_type.clone(),
            reason: format!(
                "write width {observed_width} bits contradicts expected {expected_width} bits"
            ),
        });
        return;
    }

    match compare_semantic_types(&evidence.inferred_type, &expected_type) {
        SemanticCompatibility::Compatible | SemanticCompatibility::KeyMismatch => {
            report.validated_variables.push(ValidatedVariable {
                location,
                virtual_path: matched.virtual_path,
                expected_type,
                observed_type: evidence.inferred_type.clone(),
            });
        }
        SemanticCompatibility::Contradiction => report.collisions.push(StorageCollision {
            location,
            virtual_path: matched.virtual_path,
            expected_type,
            observed_type: evidence.inferred_type.clone(),
            reason: "recovered bytecode variable contradicts the VSL type".to_owned(),
        }),
        SemanticCompatibility::Uncertain => report.uncertain_scopes.push(UncertainStorageScope {
            location,
            virtual_path: Some(matched.virtual_path),
            reason: "bytecode and VSL types cannot be compared conclusively".to_owned(),
        }),
    }
}

fn is_dynamic_array_length_write(evidence: &StorageEvidence, expected_type: &str) -> bool {
    evidence.offset == 0
        && evidence.inferred_type == "uint256"
        && evidence.symbolic_path.starts_with("Plain(")
        && expected_type.trim().ends_with("[]")
}

fn match_vsl_variable(
    evidence: &StorageEvidence,
    layout: &VirtualStorageLayout,
) -> Option<VslVariableMatch> {
    let slot = evidence.slot.as_ref()?;
    layout
        .records
        .iter()
        .filter(|record| record.parent_virtual_path.is_none())
        .find_map(|record| {
            let root = decode_slot(&record.id)?;
            let slot_index = slot_delta(&root, slot, record.slots.len())?;
            let expected_width = expected_width_at(record, slot_index, evidence.offset);
            let expected_type =
                expected_type_at(record, &layout.records, slot_index, evidence.offset);
            Some(VslVariableMatch {
                virtual_path: format!(
                    "{}.slot[{slot_index}].byte[{}]",
                    record.virtual_path, evidence.offset
                ),
                expected_type,
                expected_width,
            })
        })
}

fn expected_width_at(
    record: &VirtualStorageLayoutRecord,
    slot_index: usize,
    offset: u8,
) -> Option<u16> {
    let group = record.slots.get(slot_index)?;
    let mut current_offset = 0_u16;
    for width in group {
        if current_offset / 8 == u16::from(offset) {
            return Some(*width);
        }
        current_offset = current_offset.saturating_add(*width);
    }
    None
}

fn inferred_width(inferred_type: &str) -> Option<u16> {
    if inferred_type.starts_with("mapping(")
        || inferred_type.ends_with(']')
        || matches!(inferred_type, "bytes" | "string" | "uint256")
    {
        return Some(256);
    }
    if inferred_type == "bool" {
        return Some(8);
    }
    if inferred_type == "address" {
        return Some(160);
    }
    if inferred_type.starts_with("function") {
        return Some(192);
    }
    numeric_suffix(inferred_type, "uint")
        .or_else(|| numeric_suffix(inferred_type, "int"))
        .or_else(|| numeric_suffix(inferred_type, "bytes").and_then(|bytes| bytes.checked_mul(8)))
}

fn numeric_suffix(value: &str, prefix: &str) -> Option<u16> {
    let suffix = value.strip_prefix(prefix)?;
    (!suffix.is_empty())
        .then(|| suffix.parse::<u16>().ok())
        .flatten()
}

fn decode_slot(value: &str) -> Option<Slot> {
    let value = value.strip_prefix("0x").unwrap_or(value);
    if value.is_empty() || value.len() > 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return None;
    }
    let padded = format!("{value:0>64}");
    let mut slot = [0_u8; 32];
    for (index, byte) in slot.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&padded[index * 2..index * 2 + 2], 16).ok()?;
    }
    Some(slot)
}

fn slot_delta(root: &Slot, slot: &Slot, span: usize) -> Option<usize> {
    let mut current = *root;
    for delta in 0..span {
        if &current == slot {
            return Some(delta);
        }
        increment_slot(&mut current)?;
    }
    None
}

fn increment_slot(slot: &mut Slot) -> Option<()> {
    for byte in slot.iter_mut().rev() {
        let (next, overflow) = byte.overflowing_add(1);
        *byte = next;
        if !overflow {
            return Some(());
        }
    }
    None
}

fn hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(2 + bytes.len() * 2);
    output.push_str("0x");
    for byte in bytes {
        output.push_str(&format!("{byte:02x}"));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::{inferred_width, slot_delta, validate_write};
    use crate::{
        Selector, Slot,
        compose::{
            VirtualStorageLayoutKind, VirtualStorageLayoutRecord, VirtualStorageLayoutSource,
        },
        storage::StorageEvidence,
        storage_validation::{StorageValidationReport, VirtualStorageLayout},
    };

    #[test]
    fn keeps_mapping_and_arrays_as_word_variables() {
        assert_eq!(inferred_width("mapping(address => uint256)"), Some(256));
        assert_eq!(inferred_width("uint8[]"), Some(256));
    }

    #[test]
    fn resolves_static_vsl_slot_ranges() {
        let root = [0_u8; 32];
        let mut slot: Slot = root;
        slot[31] = 2;
        assert_eq!(slot_delta(&root, &slot, 3), Some(2));
    }

    #[test]
    fn reports_a_proven_width_collision_at_a_known_slot() {
        let layout = VirtualStorageLayout {
            records: vec![VirtualStorageLayoutRecord {
                id: "0x01".to_owned(),
                virtual_path: "fixture.root".to_owned(),
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
        };
        let evidence = StorageEvidence {
            domain: "persistent",
            slot: Some({
                let mut slot = [0_u8; 32];
                slot[31] = 1;
                slot
            }),
            symbolic_path: "Plain(0x01)".to_owned(),
            offset: 0,
            inferred_type: "uint256".to_owned(),
            score: 1,
            is_write: true,
            value_type_known: true,
            write_pc: Some(42),
            selector: Selector::default(),
            is_fallback_probe: false,
            mask: None,
        };
        let mut report = StorageValidationReport::default();

        validate_write(&evidence, &layout, &mut report);

        assert_eq!(report.collisions.len(), 1);
        assert!(report.uncertain_scopes.is_empty());
    }

    #[test]
    fn scopes_an_unmapped_write_to_its_concrete_slot() {
        let layout = VirtualStorageLayout::default();
        let evidence = StorageEvidence {
            domain: "persistent",
            slot: Some({
                let mut slot = [0_u8; 32];
                slot[31] = 9;
                slot
            }),
            symbolic_path: "Plain(0x09)".to_owned(),
            offset: 0,
            inferred_type: "uint256".to_owned(),
            score: 1,
            is_write: true,
            value_type_known: true,
            write_pc: Some(9),
            selector: Selector::default(),
            is_fallback_probe: false,
            mask: None,
        };
        let mut report = StorageValidationReport::default();

        validate_write(&evidence, &layout, &mut report);

        assert!(report.collisions.is_empty());
        assert_eq!(report.uncertain_scopes.len(), 1);
        assert_eq!(
            report.uncertain_scopes[0].location.slot,
            format!("0x{:064x}", 9)
        );
    }
}
